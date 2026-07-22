// Policy-as-Code Engine (Phase 2.3)
// Evaluates Rego/OPA policies against scan findings to enforce custom rules.
// Uses the `regorus` crate — a pure-Rust Rego interpreter (MIT/Apache-2.0/BSD-3-Clause).
//
// Convention:
//   Policies are .rego files in a configurable directory (default: .apeguard/policies/).
//   Each policy file defines a `policy_actions` rule in the `apeguard` package.
//   The rule returns an array of action objects:
//     {
//       "finding_id": "F-001",
//       "action": "block|escalate|downgrade|flag|tag",
//       "severity": "Critical|High|Medium|Low|Info",   // for escalate/downgrade
//       "tags": ["tag1", "tag2"],                       // for tag action
//       "message": "message text",                      // for flag action
//       "reason": "human-readable explanation"          // all actions
//     }
//
// Pipeline placement: after severity filter, before report generation.

use crate::find::{CanonicalFinding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configuration for Policy-as-Code evaluation (also defined in config.rs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Master switch — policy evaluation is opt-in (default: false)
    pub enabled: bool,
    /// Directory containing .rego policy files (default: ".apeguard/policies")
    pub policy_dir: PathBuf,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            policy_dir: PathBuf::from(".apeguard/policies"),
        }
    }
}

/// The action a policy can apply to a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    /// Remove the finding from results entirely
    Block,
    /// Increase severity (requires `severity` field)
    Escalate,
    /// Decrease severity (requires `severity` field)
    Downgrade,
    /// Add a message/note to the finding (requires `message` field)
    Flag,
    /// Add tags to the finding (requires `tags` field)
    Tag,
}

/// A single policy action parsed from Rego output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyActionEntry {
    /// The finding ID to act on
    #[serde(default)]
    pub finding_id: String,
    /// The action type
    pub action: PolicyAction,
    /// Target severity (required for escalate/downgrade)
    #[serde(default)]
    pub severity: Option<String>,
    /// Tags to add (required for tag)
    #[serde(default)]
    pub tags: Vec<String>,
    /// Message/note to add (for flag action)
    #[serde(default)]
    pub message: Option<String>,
    /// Human-readable explanation
    #[serde(default)]
    pub reason: Option<String>,
}

/// Overall result of policy evaluation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyResult {
    /// Whether policy evaluation was enabled
    pub enabled: bool,
    /// Number of policy files loaded
    pub policies_loaded: usize,
    /// Policy file names
    pub policy_files: Vec<String>,
    /// Actions that were applied
    pub actions_applied: Vec<PolicyActionEntry>,
    /// Number of findings blocked (removed)
    pub blocked_count: usize,
    /// Number of findings escalated
    pub escalated_count: usize,
    /// Number of findings downgraded
    pub downgraded_count: usize,
    /// Number of findings flagged
    pub flagged_count: usize,
    /// Number of findings tagged
    pub tagged_count: usize,
    /// Any errors encountered during policy evaluation
    pub errors: Vec<String>,
}

/// Load .rego policy files from a directory.
/// Returns (policy_name -> rego_source) map, or empty if directory doesn't exist.
fn load_policy_files(policy_dir: &Path) -> Vec<(String, String)> {
    if !policy_dir.exists() {
        return Vec::new();
    }

    let mut policies = Vec::new();

    let entries = match std::fs::read_dir(policy_dir) {
        Ok(entries) => entries,
        Err(e) => {
            tracing::warn!(
                "Failed to read policy directory '{}': {}",
                policy_dir.display(),
                e
            );
            return Vec::new();
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rego") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    policies.push((name, content));
                }
                Err(e) => {
                    tracing::warn!("Failed to read policy '{}': {}", path.display(), e);
                }
            }
        }
    }

    policies
}

/// Convert findings to a JSON Value suitable for Rego input.
fn findings_to_rego_input(findings: &[CanonicalFinding]) -> serde_json::Value {
    let finding_list: Vec<serde_json::Value> = findings
        .iter()
        .map(|f| {
            serde_json::json!({
                "id": f.id,
                "scanner": format!("{:?}", f.scanner),
                "rule_id": f.rule_id,
                "severity": format!("{:?}", f.severity),
                "title": f.title,
                "file": f.location.file.to_string_lossy(),
                "line": f.location.line,
                "cwe": f.cwe,
                "cvss": f.cvss,
                "tags": f.tags,
                "zt_pillars": f.zt_pillars,
                "reachable": f.reachable,
            })
        })
        .collect();

    // Collect unique scanner names
    let mut scanners: Vec<String> = findings
        .iter()
        .map(|f| format!("{:?}", f.scanner))
        .collect();
    scanners.sort();
    scanners.dedup();

    serde_json::json!({
        "findings": finding_list,
        "scan": {
            "total_findings": findings.len(),
            "scanners": scanners,
        }
    })
}

/// Evaluate findings against all loaded Rego policies.
/// Returns policy actions parsed from Rego rule evaluations.
fn evaluate_policy_actions(
    policies: &[(String, String)],
    findings: &[CanonicalFinding],
) -> Result<(Vec<PolicyActionEntry>, Vec<String>), String> {
    if policies.is_empty() || findings.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let mut engine = regorus::Engine::new();

    // Load all policies into the engine
    for (name, source) in policies {
        engine
            .add_policy(name.clone(), source.clone())
            .map_err(|e| format!("Failed to add policy '{}': {}", name, e))?;
    }

    // Set the input data using set_input_json (takes a JSON string)
    let input_json = serde_json::to_string(&findings_to_rego_input(findings))
        .map_err(|e| format!("Failed to serialize findings for policy input: {}", e))?;
    engine
        .set_input_json(&input_json)
        .map_err(|e| format!("Failed to set policy input: {}", e))?;

    // Evaluate the `data.apeguard.policy_actions` rule
    let rule_path = "data.apeguard.policy_actions".to_string();
    let result = engine.eval_rule(rule_path);
    let mut actions = Vec::new();
    let mut errors = Vec::new();

    match result {
        Ok(value) => {
            // Convert regorus::Value to serde_json::Value via serialization
            // regorus::Value implements Serialize
            let json_value = serde_json::to_value(&value)
                .map_err(|e| format!("Failed to serialize regorus result: {}", e))?;

            // The result is expected to be an array of action objects
            match json_value {
                serde_json::Value::Array(arr) => {
                    for item in arr {
                        match serde_json::from_value::<PolicyActionEntry>(item) {
                            Ok(entry) => {
                                if !entry.finding_id.is_empty() {
                                    actions.push(entry);
                                }
                            }
                            Err(e) => errors.push(format!("Failed to parse action: {}", e)),
                        }
                    }
                }
                serde_json::Value::Object(_) => {
                    // Single action object (not in an array)
                    match serde_json::from_value::<PolicyActionEntry>(json_value) {
                        Ok(entry) => {
                            if !entry.finding_id.is_empty() {
                                actions.push(entry);
                            }
                        }
                        Err(e) => errors.push(format!("Failed to parse single action: {}", e)),
                    }
                }
                _ => {
                    // Null or undefined - no actions
                    tracing::debug!("Policy evaluation returned non-array result");
                }
            }
        }
        Err(e) => {
            // No matching rule found — not an error, just no policies defined
            tracing::debug!("No policy_actions rule found: {}", e);
        }
    }

    Ok((actions, errors))
}

/// Parse a severity string from Rego output into a Severity.
fn parse_severity(s: &str) -> Option<Severity> {
    match s.to_lowercase().as_str() {
        "critical" => Some(Severity::Critical),
        "high" => Some(Severity::High),
        "medium" => Some(Severity::Medium),
        "low" => Some(Severity::Low),
        "info" => Some(Severity::Info),
        _ => None,
    }
}

/// Apply policy actions to findings.
/// Returns (modified_findings, applied_actions, stats).
fn apply_actions(
    findings: Vec<CanonicalFinding>,
    actions: &[PolicyActionEntry],
) -> (
    Vec<CanonicalFinding>,
    Vec<PolicyActionEntry>,
    PolicyActionStats,
) {
    let mut stats = PolicyActionStats::default();
    let mut kept = Vec::new();
    let mut applied = Vec::new();

    // Build a lookup map from finding ID to find the finding index
    let mut findings_map: HashMap<String, CanonicalFinding> = HashMap::new();
    for f in findings {
        findings_map.insert(f.id.clone(), f);
    }

    // Group actions by finding_id
    let mut actions_by_finding: HashMap<String, Vec<&PolicyActionEntry>> = HashMap::new();
    for action in actions {
        actions_by_finding
            .entry(action.finding_id.clone())
            .or_default()
            .push(action);
    }

    // Process each finding
    for (id, mut finding) in findings_map {
        let finding_actions = actions_by_finding.remove(&id).unwrap_or_default();
        let mut should_keep = true;

        for action_entry in &finding_actions {
            match action_entry.action {
                PolicyAction::Block => {
                    should_keep = false;
                    stats.blocked += 1;
                    applied.push((*action_entry).clone());
                }
                PolicyAction::Escalate => {
                    if let Some(ref sev_str) = action_entry.severity {
                        if let Some(new_sev) = parse_severity(sev_str) {
                            if (new_sev as u8) > (finding.severity as u8) {
                                finding.severity = new_sev;
                                stats.escalated += 1;
                                applied.push((*action_entry).clone());
                            }
                        }
                    }
                }
                PolicyAction::Downgrade => {
                    if let Some(ref sev_str) = action_entry.severity {
                        if let Some(new_sev) = parse_severity(sev_str) {
                            if (new_sev as u8) < (finding.severity as u8) {
                                finding.severity = new_sev;
                                stats.downgraded += 1;
                                applied.push((*action_entry).clone());
                            }
                        }
                    }
                }
                PolicyAction::Flag => {
                    if let Some(ref msg) = action_entry.message {
                        // Add a note to the description or tags
                        let note = format!("[POLICY: {}]", msg);
                        if !finding.tags.contains(&note) {
                            finding.tags.push(note);
                        }
                        stats.flagged += 1;
                        applied.push((*action_entry).clone());
                    }
                }
                PolicyAction::Tag => {
                    if !action_entry.tags.is_empty() {
                        for tag in &action_entry.tags {
                            if !finding.tags.contains(tag) {
                                finding.tags.push(tag.clone());
                            }
                        }
                        stats.tagged += 1;
                        applied.push((*action_entry).clone());
                    }
                }
            }
        }

        if should_keep {
            kept.push(finding);
        }
    }

    // Log actions for findings that had actions but are no longer present (shouldn't happen)
    for (remaining_id, remaining_actions) in actions_by_finding {
        tracing::warn!(
            "Policy actions reference unknown finding '{}' ({} action(s))",
            remaining_id,
            remaining_actions.len()
        );
    }

    (kept, applied, stats)
}

/// Statistics from applying policy actions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyActionStats {
    pub blocked: usize,
    pub escalated: usize,
    pub downgraded: usize,
    pub flagged: usize,
    pub tagged: usize,
}

impl PolicyActionStats {
    pub fn total_affected(&self) -> usize {
        self.blocked + self.escalated + self.downgraded + self.flagged + self.tagged
    }
}

/// Main entry point: evaluate policies against findings and apply actions.
/// Returns (modified_findings, PolicyResult).
pub fn evaluate_policies(
    findings: Vec<CanonicalFinding>,
    config: &PolicyConfig,
) -> (Vec<CanonicalFinding>, PolicyResult) {
    if !config.enabled {
        return (
            findings,
            PolicyResult {
                enabled: false,
                ..Default::default()
            },
        );
    }

    let policy_dir = &config.policy_dir;

    // Load policy files
    let policies = load_policy_files(policy_dir);
    let policies_loaded = policies.len();
    let policy_files: Vec<String> = policies.iter().map(|(name, _)| name.clone()).collect();

    if policies_loaded == 0 {
        tracing::info!(
            "Policy-as-Code enabled but no .rego files found in '{}'",
            policy_dir.display()
        );
        return (
            findings,
            PolicyResult {
                enabled: true,
                policies_loaded: 0,
                policy_files: vec![],
                ..Default::default()
            },
        );
    }

    tracing::info!(
        "Policy-as-Code: loaded {} policy file(s) from '{}'",
        policies_loaded,
        policy_dir.display()
    );

    // Evaluate policies
    let (actions, eval_errors) = match evaluate_policy_actions(&policies, &findings) {
        Ok((actions, errors)) => (actions, errors),
        Err(e) => {
            tracing::error!("Policy evaluation failed: {}", e);
            return (
                findings,
                PolicyResult {
                    enabled: true,
                    policies_loaded,
                    policy_files,
                    errors: vec![e],
                    ..Default::default()
                },
            );
        }
    };

    if actions.is_empty() && eval_errors.is_empty() {
        tracing::info!("Policy-as-Code: all policies evaluated — no actions triggered");
        return (
            findings,
            PolicyResult {
                enabled: true,
                policies_loaded,
                policy_files,
                ..Default::default()
            },
        );
    }

    // Apply actions
    let (final_findings, applied_actions, stats) = apply_actions(findings, &actions);

    // Log summary
    if stats.total_affected() > 0 {
        tracing::info!(
            "Policy-as-Code: {} actions applied (blocked:{}, escalated:{}, downgraded:{}, flagged:{}, tagged:{})",
            stats.total_affected(),
            stats.blocked,
            stats.escalated,
            stats.downgraded,
            stats.flagged,
            stats.tagged,
        );
    }

    if !eval_errors.is_empty() {
        for err in &eval_errors {
            tracing::warn!("Policy evaluation warning: {}", err);
        }
    }

    let result = PolicyResult {
        enabled: true,
        policies_loaded,
        policy_files,
        actions_applied: applied_actions,
        blocked_count: stats.blocked,
        escalated_count: stats.escalated,
        downgraded_count: stats.downgraded,
        flagged_count: stats.flagged,
        tagged_count: stats.tagged,
        errors: eval_errors,
    };

    (final_findings, result)
}

/// Format a summary of policy results for the report context.
pub fn format_policy_summary(result: &PolicyResult) -> String {
    if !result.enabled {
        return "Policy evaluation disabled".to_string();
    }
    if result.policies_loaded == 0 {
        return "No policy files found".to_string();
    }

    let mut parts = vec![format!("{} policy file(s) loaded", result.policies_loaded)];

    if result.blocked_count > 0 {
        parts.push(format!("{} blocked", result.blocked_count));
    }
    if result.escalated_count > 0 {
        parts.push(format!("{} escalated", result.escalated_count));
    }
    if result.downgraded_count > 0 {
        parts.push(format!("{} downgraded", result.downgraded_count));
    }
    if result.flagged_count > 0 {
        parts.push(format!("{} flagged", result.flagged_count));
    }
    if result.tagged_count > 0 {
        parts.push(format!("{} tagged", result.tagged_count));
    }

    parts.join(", ")
}

/// Format detailed policy actions table for reports.
pub fn format_policy_actions_table(actions: &[PolicyActionEntry]) -> String {
    if actions.is_empty() {
        return "No policy actions applied.".to_string();
    }

    let mut table =
        String::from("| Finding ID | Action | Details |\n|------------|--------|---------|\n");
    for action in actions {
        let details = match action.action {
            PolicyAction::Block => action.reason.clone().unwrap_or_default(),
            PolicyAction::Escalate | PolicyAction::Downgrade => {
                let sev = action.severity.as_deref().unwrap_or("unknown");
                let reason = action.reason.as_deref().unwrap_or("");
                format!("→ {} ({})", sev, reason)
            }
            PolicyAction::Flag => {
                let msg = action.message.as_deref().unwrap_or("");
                let reason = action.reason.as_deref().unwrap_or("");
                format!("'{}' ({})", msg, reason)
            }
            PolicyAction::Tag => {
                let tags = action.tags.join(", ");
                let reason = action.reason.as_deref().unwrap_or("");
                format!("+[{}] ({})", tags, reason)
            }
        };
        table.push_str(&format!(
            "| {} | {:?} | {} |\n",
            action.finding_id, action.action, details
        ));
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::ScannerType;

    fn make_test_finding(id: &str, severity: Severity, scanner: ScannerType) -> CanonicalFinding {
        CanonicalFinding {
            id: id.to_string(),
            scanner,
            scanner_version: None,
            rule_id: "test-rule".to_string(),
            severity,
            confidence: crate::find::Confidence::Firm,
            title: "Test finding".to_string(),
            description: "A test finding".to_string(),
            location: crate::find::FindingLocation {
                file: PathBuf::from("test.txt"),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        }
    }

    #[test]
    fn test_load_policy_files_nonexistent_dir() {
        let dir = PathBuf::from("/tmp/nonexistent_policy_dir_xyz");
        let policies = load_policy_files(&dir);
        assert!(policies.is_empty());
    }

    #[test]
    fn test_findings_to_rego_input() {
        let findings = vec![
            make_test_finding("F-001", Severity::High, ScannerType::Gitleaks),
            make_test_finding("F-002", Severity::Critical, ScannerType::Semgrep),
        ];

        let input = findings_to_rego_input(&findings);
        assert_eq!(
            input["findings"]
                .as_array()
                .expect("policy test: findings should be array")
                .len(),
            2
        );
        assert_eq!(input["findings"][0]["id"], "F-001");
        assert_eq!(input["findings"][0]["severity"], "High");
        assert_eq!(input["scan"]["total_findings"], 2);
    }

    #[test]
    fn test_parse_severity() {
        assert_eq!(parse_severity("critical"), Some(Severity::Critical));
        assert_eq!(parse_severity("High"), Some(Severity::High));
        assert_eq!(parse_severity("MEDIUM"), Some(Severity::Medium));
        assert_eq!(parse_severity("low"), Some(Severity::Low));
        assert_eq!(parse_severity("Info"), Some(Severity::Info));
        assert_eq!(parse_severity("unknown"), None);
    }

    #[test]
    fn test_apply_block_action() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Block,
            severity: None,
            tags: vec![],
            message: None,
            reason: Some("Test block".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert!(result.is_empty());
        assert_eq!(applied.len(), 1);
        assert_eq!(stats.blocked, 1);
    }

    #[test]
    fn test_apply_escalate_action() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Escalate,
            severity: Some("Critical".to_string()),
            tags: vec![],
            message: None,
            reason: Some("Test escalate".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].severity, Severity::Critical);
        assert_eq!(applied.len(), 1);
        assert_eq!(stats.escalated, 1);
    }

    #[test]
    fn test_apply_downgrade_action() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Downgrade,
            severity: Some("Low".to_string()),
            tags: vec![],
            message: None,
            reason: Some("Test downgrade".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].severity, Severity::Low);
        assert_eq!(applied.len(), 1);
        assert_eq!(stats.downgraded, 1);
    }

    #[test]
    fn test_apply_tag_action() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Tag,
            severity: None,
            tags: vec!["compliance".to_string(), "pci-dss".to_string()],
            message: None,
            reason: Some("Add compliance tags".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert!(result[0].tags.contains(&"compliance".to_string()));
        assert!(result[0].tags.contains(&"pci-dss".to_string()));
        assert_eq!(applied.len(), 1);
        assert_eq!(stats.tagged, 1);
    }

    #[test]
    fn test_apply_flag_action() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Flag,
            severity: None,
            tags: vec![],
            message: Some("Verify this finding".to_string()),
            reason: Some("Needs manual review".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .tags
            .contains(&"[POLICY: Verify this finding]".to_string()));
        assert_eq!(applied.len(), 1);
        assert_eq!(stats.flagged, 1);
    }

    #[test]
    fn test_apply_escalate_only_if_higher() {
        // Escalate only if target severity is higher than current
        let findings = vec![make_test_finding(
            "F-001",
            Severity::Critical,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Escalate,
            severity: Some("High".to_string()),
            tags: vec![],
            message: None,
            reason: Some("Should not apply".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].severity, Severity::Critical); // Unchanged
        assert_eq!(applied.len(), 0);
        assert_eq!(stats.escalated, 0);
    }

    #[test]
    fn test_apply_downgrade_only_if_lower() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::Low,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Downgrade,
            severity: Some("Critical".to_string()),
            tags: vec![],
            message: None,
            reason: Some("Should not apply".to_string()),
        }];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].severity, Severity::Low); // Unchanged
        assert_eq!(applied.len(), 0);
        assert_eq!(stats.downgraded, 0);
    }

    #[test]
    fn test_evaluate_policies_disabled() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let config = PolicyConfig {
            enabled: false,
            ..Default::default()
        };

        let (result, policy_result) = evaluate_policies(findings, &config);
        assert_eq!(result.len(), 1);
        assert!(!policy_result.enabled);
    }

    #[test]
    fn test_evaluate_policies_no_policy_dir() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let config = PolicyConfig {
            enabled: true,
            policy_dir: PathBuf::from("/tmp/nonexistent_policy_dir_xyz"),
        };

        let (result, policy_result) = evaluate_policies(findings, &config);
        assert_eq!(result.len(), 1);
        assert!(policy_result.enabled);
        assert_eq!(policy_result.policies_loaded, 0);
    }

    #[test]
    fn test_format_policy_summary_disabled() {
        let result = PolicyResult {
            enabled: false,
            ..Default::default()
        };
        let summary = format_policy_summary(&result);
        assert_eq!(summary, "Policy evaluation disabled");
    }

    #[test]
    fn test_format_policy_summary_active() {
        let result = PolicyResult {
            enabled: true,
            policies_loaded: 3,
            blocked_count: 2,
            escalated_count: 1,
            ..Default::default()
        };
        let summary = format_policy_summary(&result);
        assert!(summary.contains("3 policy file(s) loaded"));
        assert!(summary.contains("2 blocked"));
        assert!(summary.contains("1 escalated"));
    }

    #[test]
    fn test_format_policy_actions_table_empty() {
        let table = format_policy_actions_table(&[]);
        assert_eq!(table, "No policy actions applied.");
    }

    #[test]
    fn test_format_policy_actions_table_with_actions() {
        let actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Block,
            severity: None,
            tags: vec![],
            message: None,
            reason: Some("Secret in config".to_string()),
        }];
        let table = format_policy_actions_table(&actions);
        assert!(table.contains("F-001"));
        assert!(table.contains("Block"));
    }

    #[test]
    fn test_policy_action_stats() {
        let stats = PolicyActionStats {
            blocked: 2,
            escalated: 1,
            downgraded: 3,
            flagged: 0,
            tagged: 4,
        };
        assert_eq!(stats.total_affected(), 10);
    }

    #[test]
    fn test_empty_findings_no_actions() {
        let (actions, errors) =
            evaluate_policy_actions(&[("test".to_string(), "package apeguard".to_string())], &[])
                .expect("policy test: evaluate_policy_actions should succeed");
        assert!(actions.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn test_regorus_serde_roundtrip() {
        // Test that regorus::Value can be serialized back to JSON
        let input = serde_json::json!({
            "finding_id": "F-001",
            "action": "block",
            "message": null,
        });
        let regorus_val = regorus::Value::from(input.clone());
        let roundtrip = serde_json::to_value(&regorus_val)
            .expect("policy test: regorus roundtrip should succeed");
        assert_eq!(roundtrip["finding_id"], "F-001");
        assert_eq!(roundtrip["action"], "block");
    }

    #[test]
    fn test_action_applied_but_finding_removed() {
        // Apply actions to findings, then remove the finding
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let _actions = PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Tag,
            severity: None,
            tags: vec!["compliance".to_string()],
            message: None,
            reason: None,
        };

        // Now block it
        let block_actions = vec![PolicyActionEntry {
            finding_id: "F-001".to_string(),
            action: PolicyAction::Block,
            severity: None,
            tags: vec![],
            message: None,
            reason: None,
        }];

        let (result, _applied, stats) = apply_actions(findings, &block_actions);
        assert!(result.is_empty());
        assert_eq!(stats.blocked, 1);
    }

    #[test]
    fn test_multiple_actions_same_finding() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![
            PolicyActionEntry {
                finding_id: "F-001".to_string(),
                action: PolicyAction::Escalate,
                severity: Some("Critical".to_string()),
                tags: vec![],
                message: None,
                reason: Some("Critical severity".to_string()),
            },
            PolicyActionEntry {
                finding_id: "F-001".to_string(),
                action: PolicyAction::Tag,
                severity: None,
                tags: vec!["compliance".to_string()],
                message: None,
                reason: Some("Compliance tag".to_string()),
            },
        ];

        let (result, applied, stats) = apply_actions(findings, &actions);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].severity, Severity::Critical);
        assert!(result[0].tags.contains(&"compliance".to_string()));
        assert_eq!(applied.len(), 2);
        assert_eq!(stats.total_affected(), 2);
    }

    #[test]
    fn test_unknown_finding_actions_logged() {
        let findings = vec![make_test_finding(
            "F-001",
            Severity::High,
            ScannerType::Gitleaks,
        )];
        let actions = vec![PolicyActionEntry {
            finding_id: "F-NONEXISTENT".to_string(),
            action: PolicyAction::Block,
            severity: None,
            tags: vec![],
            message: None,
            reason: None,
        }];

        let (result, _applied, _stats) = apply_actions(findings, &actions);
        // Finding F-001 should survive (no matching action)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "F-001");
    }
}

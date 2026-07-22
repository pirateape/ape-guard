// ApeGuard Finding Normalization
// Post-processes raw findings from all scanners into a unified, enriched format.
// Responsibilities:
//   1. Cross-reference findings across scanners
//   2. Enrich with Unified Zero Trust Framework (UZTF) pillar mappings
//   3. Compute confidence scores
//   4. Tag findings with additional context
//
// The UZTF is an 8-pillar maturity model that builds on the CISA Zero Trust
// Maturity Model as its foundational stepping stone. This implementation maps
// security findings to UZTF pillars (Identity, Devices, Networks, Applications,
// Data, Visibility, Automation, Infrastructure) and computes quantitative
// pillar scores (0-100) and overall scorecard (0-800).
// See: https://github.com/pirateape/unified-zero-trust-framework
use crate::find::{
    CanonicalFinding, GapAnalysis, GapLevel, MaturityTier, PillarScore, ScannerType, Severity,
    ZeroTrustScorecard,
};

/// Zero Trust pillar mapping rules
/// Maps common vulnerability types to ZT pillars and maturity levels.
const ZT_MAPPINGS: &[(&str, &str, MaturityTier)] = &[
    // Secrets → Identity pillar
    ("secret", "identity", MaturityTier::Baseline),
    ("credential", "identity", MaturityTier::Baseline),
    ("password", "identity", MaturityTier::Baseline),
    // SAST → Devices pillar
    ("injection", "devices", MaturityTier::Baseline),
    ("xss", "devices", MaturityTier::Advanced),
    ("rce", "devices", MaturityTier::Advanced),
    // DAST web-app findings → Applications pillar
    ("sqli", "applications", MaturityTier::Baseline),
    ("sql injection", "applications", MaturityTier::Baseline),
    ("idor", "applications", MaturityTier::Advanced),
    ("csrf", "applications", MaturityTier::Baseline),
    ("ssti", "applications", MaturityTier::Advanced),
    ("open redirect", "applications", MaturityTier::Baseline),
    // Dependency vulns → Applications pillar
    ("dependency", "applications", MaturityTier::Baseline),
    ("vulnerability", "applications", MaturityTier::Baseline),
    ("cve", "applications", MaturityTier::Baseline),
    // Misconfig → Networks pillar
    ("misconfig", "networks", MaturityTier::Baseline),
    ("misconfiguration", "networks", MaturityTier::Baseline),
    ("ssrf", "networks", MaturityTier::Advanced),
    ("iac", "networks", MaturityTier::Advanced),
    ("docker", "networks", MaturityTier::Baseline),
    // General
    ("cwe", "applications", MaturityTier::Baseline),
];

/// Gitleaks rule-to-severity overrides.
/// Gitleaks often defaults to "medium" for many rules. This table maps specific
/// rule IDs to their appropriate severity based on the type of secret exposed.
const GITLEAKS_SEVERITY_MAP: &[(&str, Severity)] = &[
    // Cloud provider credentials — immediate compromise risk
    ("aws-access-token", Severity::Critical),
    ("aws-secret-key", Severity::Critical),
    ("gcp-service-account", Severity::Critical),
    ("google-api-key", Severity::High),
    ("azure-client-secret", Severity::Critical),
    ("azure-subscription-key", Severity::High),
    // SaaS / platform tokens — broad access
    ("github-pat", Severity::Critical),
    ("gitlab-pat", Severity::Critical),
    ("slack-token", Severity::High),
    ("slack-webhook-url", Severity::High),
    ("discord-bot-token", Severity::Critical),
    ("telegram-bot-token", Severity::High),
    ("npm-auth-token", Severity::High),
    ("pypi-upload-token", Severity::High),
    ("docker-login", Severity::Critical),
    ("docker-auth", Severity::Critical),
    // Database / infrastructure
    ("private-key", Severity::Critical),
    ("ssh-private-key", Severity::Critical),
    ("pgp-private-key", Severity::Critical),
    ("mysql-connection-string", Severity::High),
    ("postgresql-connection-string", Severity::High),
    ("mongodb-connection-string", Severity::High),
    ("redis-connection-string", Severity::Medium),
    // Payment / finance
    ("stripe-api-key", Severity::Critical),
    ("stripe-live-key", Severity::Critical),
    ("square-oauth-secret", Severity::Critical),
    ("paypal-auth-token", Severity::Critical),
    // Hashed / encoded — lower confidence but still significant
    ("generic-api-key", Severity::Medium),
    ("jwt-token", Severity::High),
    ("password", Severity::High),
    ("connection-string", Severity::High),
    ("pre-shared-key", Severity::High),
    ("bearer-token", Severity::High),
    ("basic-auth", Severity::High),
    ("oauth-client-secret", Severity::High),
    ("s3-bucket-config", Severity::High),
    ("s3-secret-key", Severity::Critical),
    ("heroku-api-key", Severity::Critical),
    ("sauce-token", Severity::High),
    ("sentry-token", Severity::High),
    ("datadog-api-key", Severity::High),
    ("new-relic-api-key", Severity::High),
    ("twilio-api-key", Severity::Critical),
    ("sendgrid-api-key", Severity::Critical),
    ("mailgun-api-key", Severity::High),
    ("hashicorp-token", Severity::Critical),
    ("vault-token", Severity::Critical),
    ("consul-token", Severity::High),
    ("k8s-service-account", Severity::Critical),
    ("k8s-token", Severity::Critical),
    ("kubeconfig", Severity::Critical),
    ("grafana-api-key", Severity::High),
    ("jfrog-api-key", Severity::High),
    ("sonar-token", Severity::Medium),
    ("jira-token", Severity::Medium),
    ("confluence-token", Severity::Medium),
    ("pagerduty-api-key", Severity::High),
    ("sumologic-token", Severity::Medium),
    ("segment-api-key", Severity::Medium),
    ("launchdarkly-token", Severity::Medium),
    ("monday-api-token", Severity::Medium),
    ("notion-api-token", Severity::Medium),
    ("asana-token", Severity::Medium),
];

/// Normalize a batch of findings: enrich with ZT mappings, cross-reference, tag
pub fn normalize_findings(findings: &mut [CanonicalFinding]) {
    for finding in findings.iter_mut() {
        // Apply Gitleaks severity overrides based on rule ID
        if finding.scanner == ScannerType::Gitleaks {
            let rule_lower = finding.rule_id.to_lowercase();
            for (rule_pattern, severity) in GITLEAKS_SEVERITY_MAP {
                if rule_lower.contains(rule_pattern) {
                    finding.severity = *severity;
                    break;
                }
            }
        }

        // Enrich with Zero Trust pillar mappings
        let rule_lower = finding.rule_id.to_lowercase();
        let title_lower = finding.title.to_lowercase();
        let combined = format!("{} {}", rule_lower, title_lower);

        for (keyword, pillar, _maturity) in ZT_MAPPINGS {
            if combined.contains(keyword) && !finding.zt_pillars.contains(&pillar.to_string()) {
                finding.zt_pillars.push(pillar.to_string());
            }
        }

        // Default to "applications" pillar if nothing matched
        if finding.zt_pillars.is_empty() {
            finding.zt_pillars.push("applications".to_string());
        }
    }
}

/// Compute a Zero Trust scorecard from normalized findings
pub fn compute_zt_scorecard(findings: &[CanonicalFinding]) -> ZeroTrustScorecard {
    use std::collections::HashMap;

    let all_pillars = [
        "identity",
        "devices",
        "networks",
        "applications",
        "data",
        "visibility",
        "automation",
        "analytics",
    ];

    // Track findings per pillar with severity-weighted scoring
    let mut pillar_severity_score: HashMap<&str, u32> = HashMap::new();
    let mut pillar_finding_refs: HashMap<&str, Vec<&CanonicalFinding>> = HashMap::new();

    for finding in findings {
        for pillar in &finding.zt_pillars {
            // Severity weight: Critical=10, High=5, Medium=3, Low=1, Info=0
            let weight = match finding.severity {
                crate::find::Severity::Critical => 10,
                crate::find::Severity::High => 5,
                crate::find::Severity::Medium => 3,
                crate::find::Severity::Low => 1,
                crate::find::Severity::Info => 0,
            };
            *pillar_severity_score.entry(pillar).or_insert(0) += weight;
            pillar_finding_refs.entry(pillar).or_default().push(finding);
        }
    }

    let mut total_gaps = 0u32;

    let pillar_scores: Vec<PillarScore> = all_pillars
        .iter()
        .map(|name| {
            let severity_weight = pillar_severity_score.get(name).copied().unwrap_or(0);

            // Severity-weighted gap count: cap at 10 to keep scoring reasonable
            let gap_count = severity_weight.min(10);
            total_gaps += gap_count;

            // Maturity determined by severity-weighted score
            let maturity = if severity_weight == 0 {
                MaturityTier::Adaptive
            } else if severity_weight <= 3 {
                MaturityTier::Advanced
            } else {
                MaturityTier::Baseline
            };

            // Score: 100 - (gap_count * 10), minimum 0
            let score = 100u32.saturating_sub(gap_count * 10);

            PillarScore {
                name: name.to_string(),
                maturity,
                gap_count,
                score,
            }
        })
        .collect();

    let max_score = all_pillars.len() as u32 * 100;
    let overall_score = pillar_scores.iter().map(|p| p.score).sum::<u32>();
    let at_advanced = pillar_scores
        .iter()
        .filter(|p| matches!(p.maturity, MaturityTier::Advanced | MaturityTier::Adaptive))
        .count() as u32;

    // Compute gap analysis
    let gap_analysis =
        compute_gap_analysis(&all_pillars, &pillar_severity_score, &pillar_finding_refs);

    ZeroTrustScorecard {
        overall_score,
        max_score,
        pillars: pillar_scores,
        pillars_at_advanced_or_higher: at_advanced,
        target_maturity: MaturityTier::Advanced,
        gap_analysis,
    }
}

/// Compute detailed gap analysis for each pillar (severity-weighted).
pub fn compute_gap_analysis(
    all_pillars: &[&str],
    pillar_severity: &std::collections::HashMap<&str, u32>,
    pillar_finding_refs: &std::collections::HashMap<&str, Vec<&CanonicalFinding>>,
) -> Vec<GapAnalysis> {
    let target = MaturityTier::Advanced;
    let mut analysis = Vec::new();

    for pillar_name in all_pillars {
        let severity_weight = pillar_severity.get(*pillar_name).copied().unwrap_or(0);

        // Determine current maturity based on severity weight
        let current = if severity_weight == 0 {
            MaturityTier::Adaptive
        } else if severity_weight <= 3 {
            MaturityTier::Advanced
        } else {
            MaturityTier::Baseline
        };

        // Compute gap level
        let gap = match (&current, &target) {
            (a, b) if a == b => GapLevel::None,
            (MaturityTier::Adaptive, _) => GapLevel::None, // Already exceeded
            (MaturityTier::Advanced, MaturityTier::Advanced) => GapLevel::None,
            (MaturityTier::Baseline, MaturityTier::Advanced) => {
                if severity_weight > 10 {
                    GapLevel::Large
                } else if severity_weight > 5 {
                    GapLevel::Medium
                } else {
                    GapLevel::Small
                }
            }
            _ => GapLevel::Small,
        };

        // Build recommendations from findings
        let recommendations: Vec<String> = pillar_finding_refs
            .get(*pillar_name)
            .map(|refs| {
                refs.iter()
                    .take(3)
                    .map(|f| format!("{}: {}", f.rule_id, f.title))
                    .collect()
            })
            .unwrap_or_default();

        let blocking_count = pillar_finding_refs
            .get(*pillar_name)
            .map(|refs| refs.len() as u32)
            .unwrap_or(0);

        analysis.push(GapAnalysis {
            pillar: pillar_name.to_string(),
            current_maturity: current,
            target_maturity: target.clone(),
            gap,
            blocking_findings: blocking_count,
            recommendations,
        });
    }

    analysis
}

/// Generate pillar-specific remediation recommendations.
#[allow(dead_code)] // P3/P4: pillar recommendations not yet wired into report generation
pub fn generate_pillar_recommendations(
    pillar: &str,
    maturity: MaturityTier,
    finding_count: u32,
) -> Vec<String> {
    let mut recs = Vec::new();

    // Add maturity-aware prefix
    let urgency = match maturity {
        MaturityTier::Baseline => "[HIGH PRIORITY] ",
        MaturityTier::Advanced => "",
        MaturityTier::Adaptive => "[MAINTAIN] ",
    };

    match pillar {
        "identity" => {
            recs.push(format!(
                "{}Implement credential scanning in CI/CD pipeline",
                urgency
            ));
            recs.push(format!("{}Rotate hardcoded secrets regularly", urgency));
            if finding_count > 0 {
                recs.push(format!(
                    "{}Address {} exposed credential finding(s)",
                    urgency, finding_count
                ));
            }
            if maturity == MaturityTier::Baseline {
                recs.push("URGENT: Move identity security to Advanced maturity".into());
            }
        }
        "devices" => {
            recs.push(format!(
                "{}Enable runtime code analysis in staging",
                urgency
            ));
            recs.push(format!(
                "{}Add input validation and output encoding",
                urgency
            ));
            if finding_count > 0 {
                recs.push(format!(
                    "{}Fix {} code quality finding(s)",
                    urgency, finding_count
                ));
            }
        }
        "networks" => {
            recs.push(format!("{}Use IaC scanning before deployment", urgency));
            recs.push(format!("{}Implement network segmentation", urgency));
            if finding_count > 0 {
                recs.push(format!(
                    "{}Resolve {} misconfiguration finding(s)",
                    urgency, finding_count
                ));
            }
        }
        "applications" => {
            recs.push(format!("{}Enable automated dependency scanning", urgency));
            recs.push(format!("{}Patch known CVEs in dependencies", urgency));
            if finding_count > 0 {
                recs.push(format!(
                    "{}Update {} vulnerable dependenc(y/ies)",
                    urgency, finding_count
                ));
            }
        }
        "data" => {
            recs.push(format!("{}Classify data by sensitivity level", urgency));
            recs.push(format!(
                "{}Implement encryption at rest and in transit",
                urgency
            ));
        }
        "visibility" => {
            recs.push(format!("{}Enable centralized logging", urgency));
            recs.push(format!("{}Set up security monitoring dashboards", urgency));
        }
        "automation" => {
            recs.push(format!("{}Automate security checks in CI/CD", urgency));
            recs.push(format!("{}Implement policy-as-code", urgency));
        }
        "analytics" => {
            recs.push(format!("{}Deploy SIEM or log analytics", urgency));
            recs.push(format!(
                "{}Set up automated alerting on security events",
                urgency
            ));
        }
        _ => {
            recs.push(format!(
                "{}Review {} finding(s) in {}",
                urgency, finding_count, pillar
            ));
        }
    }

    recs
}

/// Enrich a finding with MITRE ATT&CK mapping (simplified)
pub fn mitre_mapping(finding: &CanonicalFinding) -> Vec<String> {
    let rule_lower = finding.rule_id.to_lowercase();
    let title_lower = finding.title.to_lowercase();
    let combined = format!("{} {}", rule_lower, title_lower);

    let mut tactics = Vec::new();

    if combined.contains("secret")
        || combined.contains("credential")
        || combined.contains("password")
    {
        tactics.push("TA0006".to_string()); // Credential Access
    }
    if combined.contains("injection") {
        tactics.push("TA0001".to_string()); // Initial Access
    }
    if combined.contains("rce") || combined.contains("remote") {
        tactics.push("TA0002".to_string()); // Execution
    }
    if combined.contains("xss") {
        tactics.push("TA0001".to_string()); // Initial Access
    }
    if combined.contains("misconfig") || combined.contains("iac") {
        tactics.push("TA0004".to_string()); // Privilege Escalation
    }

    tactics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::*;
    use std::path::PathBuf;

    fn make_finding(id: &str, rule_id: &str, title: &str) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: rule_id.into(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: title.into(),
            description: "desc".into(),
            location: FindingLocation {
                file: PathBuf::from("test.py"),
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
    fn test_zt_mapping_secret() {
        let mut findings = vec![make_finding(
            "1",
            "gitleaks-aws-key",
            "AWS Secret Key Found",
        )];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"identity".to_string()));
    }

    #[test]
    fn test_zt_mapping_injection() {
        let mut findings = vec![make_finding("1", "semgrep-sqli", "SQL Injection detected")];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"devices".to_string()));
    }

    #[test]
    fn test_zt_mapping_xss() {
        let mut findings = vec![make_finding("1", "semgrep-xss", "XSS vulnerability")];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"devices".to_string()));
    }

    #[test]
    fn test_zt_mapping_dependency() {
        let mut findings = vec![make_finding(
            "1",
            "CVE-2024-1234",
            "Critical vulnerability in dep",
        )];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
    }

    #[test]
    fn test_zt_mapping_misconfig() {
        let mut findings = vec![make_finding(
            "1",
            "trivy-misconfig-001",
            "Misconfigured S3 bucket",
        )];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"networks".to_string()));
    }

    #[test]
    fn test_zt_default_pillar() {
        let mut findings = vec![make_finding("1", "custom-rule", "Generic security issue")];
        normalize_findings(&mut findings);
        // Falls back to "applications" when nothing matches
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
    }

    #[test]
    fn test_zt_mapping_no_duplicates() {
        let mut findings = vec![make_finding("1", "secret", "Credential password found")];
        normalize_findings(&mut findings);
        // "secret" and "credential" and "password" all map to "identity"
        // but it should only appear once
        let identity_count = findings[0]
            .zt_pillars
            .iter()
            .filter(|p| *p == "identity")
            .count();
        assert_eq!(identity_count, 1);
    }

    #[test]
    fn test_scorecard_no_findings() {
        let findings = vec![];
        let sc = compute_zt_scorecard(&findings);
        assert_eq!(sc.overall_score, 800); // 8 pillars × 100
        assert_eq!(sc.pillars.len(), 8);
        for pillar in &sc.pillars {
            assert_eq!(pillar.score, 100);
            assert_eq!(pillar.maturity, MaturityTier::Adaptive);
        }
    }

    #[test]
    fn test_scorecard_single_finding() {
        // Findings need zt_pillars populated for scorecard to see them
        let mut findings = vec![make_finding("1", "secret", "Secret")];
        normalize_findings(&mut findings); // Sets zt_pillars to ["identity"]
        let sc = compute_zt_scorecard(&findings);
        let identity = sc
            .pillars
            .iter()
            .find(|p| p.name == "identity")
            .expect("normalize test: identity pillar should exist");
        assert!(identity.score < 100); // Should lose points
        assert_eq!(identity.gap_count, 5); // High severity weight=5
    }

    #[test]
    fn test_scorecard_multiple_findings_capped() {
        let mut findings = vec![
            make_finding("1", "secret", "Secret 1"),
            make_finding("2", "secret", "Secret 2"),
            make_finding("3", "secret", "Secret 3"),
            make_finding("4", "secret", "Secret 4"),
            make_finding("5", "secret", "Secret 5"),
            make_finding("6", "secret", "Secret 6"), // 6th adds more weight
        ];
        normalize_findings(&mut findings); // Sets zt_pillars for all
        let sc = compute_zt_scorecard(&findings);
        let identity = sc
            .pillars
            .iter()
            .find(|p| p.name == "identity")
            .expect("normalize test: identity pillar should exist");
        assert_eq!(identity.gap_count, 10); // 6 High × 5 = 30, capped at 10
        assert_eq!(identity.score, 0); // 10 gaps * 10 = 100 deduction → 0
    }

    #[test]
    fn test_scorecard_maturity_levels() {
        // 1 low-severity finding → Advanced
        let mut findings = vec![make_finding("1", "info-rule", "Info level")];
        findings[0].severity = Severity::Low;
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let app = sc
            .pillars
            .iter()
            .find(|p| p.name == "applications")
            .expect("normalize test: applications pillar should exist");
        assert_eq!(app.maturity, MaturityTier::Advanced);

        // Multiple high severity → Baseline
        let mut findings = vec![
            make_finding("1", "secret", "Critical 1"),
            make_finding("2", "secret", "Critical 2"),
            make_finding("3", "secret", "Critical 3"),
        ];
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let identity = sc
            .pillars
            .iter()
            .find(|p| p.name == "identity")
            .expect("normalize test: identity pillar should exist");
        assert_eq!(identity.maturity, MaturityTier::Baseline);
    }

    #[test]
    fn test_mitre_mapping_secret() {
        let finding = make_finding("1", "gitleaks-aws-key", "AWS Secret Key Found");
        let tactics = mitre_mapping(&finding);
        assert!(tactics.contains(&"TA0006".to_string())); // Credential Access
    }

    #[test]
    fn test_multiple_zt_pillars() {
        let mut findings = vec![make_finding(
            "1",
            "CVE-2024-secret",
            "CVE with credential leak",
        )];
        normalize_findings(&mut findings);
        // Should map to both "applications" (lowercase cve) and "identity" (secret/credential)
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
        assert!(findings[0].zt_pillars.contains(&"identity".to_string()));
    }

    #[test]
    fn test_dast_keyword_mappings_app_and_network() {
        let mut findings = vec![make_finding(
            "1",
            "nuclei-idor-ssrf",
            "IDOR and SSRF in endpoint",
        )];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
        assert!(findings[0].zt_pillars.contains(&"networks".to_string()));
    }

    #[test]
    fn test_gap_analysis_no_findings() {
        let findings = vec![];
        let sc = compute_zt_scorecard(&findings);
        // All pillars at Adaptive with zero gaps
        for ga in &sc.gap_analysis {
            assert_eq!(ga.current_maturity, MaturityTier::Adaptive);
            assert_eq!(ga.gap, GapLevel::None);
            assert_eq!(ga.blocking_findings, 0);
        }
        assert_eq!(sc.gap_analysis.len(), 8);
    }

    #[test]
    fn test_gap_analysis_with_secrets() {
        let mut findings = vec![
            make_finding("1", "secret-key", "AWS Secret Key"),
            make_finding("2", "secret-password", "Hardcoded Password"),
            make_finding("3", "secret-token", "API Token"),
        ];
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let identity_ga = sc
            .gap_analysis
            .iter()
            .find(|g| g.pillar == "identity")
            .expect("normalize test: identity gap analysis should exist");
        assert_eq!(identity_ga.current_maturity, MaturityTier::Baseline);
        assert_eq!(identity_ga.blocking_findings, 3);
    }

    #[test]
    fn test_pillar_recommendations() {
        let recs = super::generate_pillar_recommendations("identity", MaturityTier::Baseline, 3);
        assert!(!recs.is_empty());
        assert!(recs[0].contains("credential scanning"));
    }

    #[test]
    fn test_recommendations_missing_findings() {
        let recs = super::generate_pillar_recommendations("networks", MaturityTier::Adaptive, 0);
        assert!(!recs.is_empty());
    }
}

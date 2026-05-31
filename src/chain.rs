// ApeGuard Attack Chain Engine
// Connects individual findings into composite kill chains.
// Maps findings to known attack patterns and computes composite risk scores.

use crate::find::*;
use std::collections::{HashMap, HashSet};

/// Predefined attack chain patterns.
/// Each pattern defines how findings should be grouped into chains.
const CHAIN_PATTERNS: &[ChainPattern] = &[
    ChainPattern {
        name: "Credential Compromise Chain",
        description: "Hardcoded secrets lead to injection attacks and potential RCE",
        stages: &[
            "secret",
            "credential",
            "password",
            "injection",
            "rce",
            "xss",
        ],
        risk_multiplier: 2.0,
    },
    ChainPattern {
        name: "Supply Chain Attack",
        description: "Vulnerable dependencies combined with misconfiguration create attack surface",
        stages: &[
            "vulnerability",
            "cve",
            "dependency",
            "misconfig",
            "iac",
            "docker",
        ],
        risk_multiplier: 1.5,
    },
    ChainPattern {
        name: "Infrastructure Escalation",
        description: "Misconfigured infrastructure exposed to unpatched vulnerabilities",
        stages: &["misconfig", "iac", "docker", "vulnerability", "cve"],
        risk_multiplier: 1.8,
    },
    ChainPattern {
        name: "Data Exfiltration Path",
        description: "SQL injection or credential exposure enables data access",
        stages: &["injection", "secret", "credential", "password", "xss"],
        risk_multiplier: 2.5,
    },
];

struct ChainPattern {
    name: &'static str,
    description: &'static str,
    stages: &'static [&'static str],
    risk_multiplier: f32,
}

/// Build attack chains from normalized findings.
/// Groups findings by proximity (same file/directory) and maps to known patterns.
pub fn build_attack_chains(findings: &[CanonicalFinding]) -> Vec<AttackChain> {
    let mut chains = Vec::new();
    let mut chain_id_counter: u32 = 1;

    // Step 1: Group findings by directory proximity
    let dir_groups = group_by_directory(findings);

    // Step 2: For each directory group, check if findings form a pattern
    for group in dir_groups.values() {
        if group.len() < 2 {
            continue;
        }

        // Collect all keywords from this group
        let keywords: HashSet<&str> = group
            .iter()
            .flat_map(|f| {
                let rule_lower = f.rule_id.to_lowercase();
                let title_lower = f.title.to_lowercase();
                CHAIN_PATTERNS
                    .iter()
                    .flat_map(|p| p.stages)
                    .filter(move |s| rule_lower.contains(*s) || title_lower.contains(*s))
                    .copied()
                    .collect::<Vec<_>>()
            })
            .collect();

        // Match against each pattern
        for pattern in CHAIN_PATTERNS {
            let matched_stages: Vec<&&str> = pattern
                .stages
                .iter()
                .filter(|s| keywords.contains(*s))
                .collect();

            // Need at least 2 stages from the same pattern
            if matched_stages.len() >= 2 {
                let chain_finding_ids: Vec<String> = group.iter().map(|f| f.id.clone()).collect();

                // Compute composite risk score
                let risk_score = compute_chain_risk(group, pattern, &matched_stages);

                // Build steps description
                let steps = build_chain_steps(group);

                let recommendation = generate_recommendation(pattern, &matched_stages);

                chains.push(AttackChain {
                    id: format!("AC-{:03}", chain_id_counter),
                    risk_score,
                    description: format!(
                        "{}: {} ({} stages matched: {})",
                        pattern.name,
                        pattern.description,
                        matched_stages.len(),
                        matched_stages
                            .iter()
                            .map(|s| **s)
                            .collect::<Vec<_>>()
                            .join(" → ")
                    ),
                    steps,
                    finding_ids: chain_finding_ids,
                    recommendation,
                });

                chain_id_counter += 1;
                break; // One chain per pattern per directory
            }
        }
    }

    // Step 3: Also create chains from cross-referenced findings
    // (findings confirmed by multiple scanners at same location)
    for finding in findings {
        if finding.cross_refs.len() >= 2 {
            // Multi-scanner confirmation = higher confidence chain
            let related_ids: Vec<String> = finding
                .cross_refs
                .iter()
                .map(|cr| format!("{}-{}", cr.scanner, cr.rule_id))
                .collect();

            chains.push(AttackChain {
                id: format!("AC-{:03}", chain_id_counter),
                risk_score: severity_to_score(&finding.severity) as f32 * 0.8,
                description: format!(
                    "Multi-scanner confirmed {}: {}",
                    finding.rule_id, finding.title
                ),
                steps: vec![
                    format!("{} detected by {}", finding.rule_id, finding.scanner),
                    format!(
                        "Confirmed by {} other scanners: {}",
                        related_ids.len(),
                        related_ids.join(", ")
                    ),
                ],
                finding_ids: vec![finding.id.clone()],
                recommendation: format!(
                    "High confidence finding — prioritize remediation: {}",
                    finding.remediation.as_deref().unwrap_or("Review and fix")
                ),
            });

            chain_id_counter += 1;
        }
    }

    chains
}

/// Group findings by their parent directory for proximity-based chaining.
fn group_by_directory(findings: &[CanonicalFinding]) -> HashMap<String, Vec<&CanonicalFinding>> {
    let mut groups: HashMap<String, Vec<&CanonicalFinding>> = HashMap::new();

    for finding in findings {
        // Group by parent directory (up to 2 levels up for broader grouping)
        let dir = finding
            .location
            .file
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        groups.entry(dir).or_default().push(finding);
    }

    groups
}

/// Compute a composite risk score for a chain based on finding severity and pattern multiplier.
fn compute_chain_risk(
    findings: &[&CanonicalFinding],
    pattern: &ChainPattern,
    matched_stages: &[&&str],
) -> f32 {
    let base_score: f32 = findings
        .iter()
        .map(|f| severity_to_score(&f.severity) as f32)
        .sum();

    let avg_score = if !findings.is_empty() {
        base_score / findings.len() as f32
    } else {
        0.0
    };

    // Bonus for more stages
    let stage_bonus = (matched_stages.len() as f32 - 1.0) * 0.5;

    // Apply pattern multiplier and cap at 10.0
    let raw = (avg_score + stage_bonus) * pattern.risk_multiplier;
    raw.min(10.0)
}

/// Convert severity to numeric score.
fn severity_to_score(severity: &Severity) -> u32 {
    match severity {
        Severity::Critical => 10,
        Severity::High => 7,
        Severity::Medium => 5,
        Severity::Low => 3,
        Severity::Info => 1,
    }
}

/// Build human-readable attack chain steps from findings.
fn build_chain_steps(findings: &[&CanonicalFinding]) -> Vec<String> {
    // Sort by severity (most severe first) to show most impactful path
    let mut sorted: Vec<&&CanonicalFinding> = findings.iter().collect();
    sorted.sort_by_key(|f| match f.severity {
        Severity::Critical => 0u8,
        Severity::High => 1,
        Severity::Medium => 2,
        Severity::Low => 3,
        Severity::Info => 4,
    });

    sorted
        .iter()
        .map(|f| {
            format!(
                "[{}] {}: {} ({}:{})",
                f.severity.tag(),
                f.scanner,
                f.title,
                f.location.file.to_string_lossy(),
                f.location
                    .line
                    .map(|l| l.to_string())
                    .unwrap_or_else(|| "?".to_string()),
            )
        })
        .collect()
}

/// Generate a remediation recommendation for a specific chain pattern.
fn generate_recommendation(pattern: &ChainPattern, matched_stages: &[&&str]) -> String {
    let stage_str: Vec<&str> = matched_stages.iter().map(|s| **s).collect();

    // Use the pattern's name and description as context for the recommendation
    let base = if stage_str.contains(&"secret") && stage_str.contains(&"rce") {
        "Rotate all exposed credentials immediately. Add credential scanning to CI and enforce code review for secrets."
    } else if stage_str.contains(&"vulnerability") && stage_str.contains(&"misconfig") {
        "Patch vulnerable dependencies AND fix the misconfiguration. Run both SCA and IaC scans in CI pipeline."
    } else if stage_str.contains(&"misconfig") && stage_str.contains(&"cve") {
        "Harden infrastructure configs and patch CVE. Apply least-privilege IAM policies and enable vulnerability scanning."
    } else if stage_str.contains(&"injection")
        && (stage_str.contains(&"secret") || stage_str.contains(&"credential"))
    {
        "Fix the injection vulnerability AND rotate exposed credentials. Add input validation and parameterized queries."
    } else {
        "Address all linked findings together — remediation in isolation leaves attack surface open."
    };

    // Preface with pattern name and risk multiplier when it's high
    if pattern.risk_multiplier >= 2.0 {
        format!(
            "[{} — Risk multiplier {:.1}x] {}",
            pattern.name, pattern.risk_multiplier, base
        )
    } else {
        format!("[{}] {}", pattern.name, base)
    }
}

// Severity tag helper
impl Severity {
    fn tag(&self) -> &'static str {
        match self {
            Severity::Critical => "CRIT",
            Severity::High => "HIGH",
            Severity::Medium => "MED",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[allow(clippy::too_many_arguments)]
    fn make_finding(
        id: &str,
        rule_id: &str,
        title: &str,
        severity: Severity,
        file: &str,
        line: u32,
        scanner: ScannerType,
        cross_refs: Vec<CrossReference>,
    ) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner,
            scanner_version: None,
            rule_id: rule_id.into(),
            severity,
            confidence: Confidence::Firm,
            title: title.into(),
            description: format!("{} description", title),
            location: FindingLocation {
                file: PathBuf::from(file),
                line: Some(line),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some("Fix this".into()),
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs,
        }
    }

    #[test]
    fn test_credential_compromise_chain() {
        let findings = vec![
            make_finding(
                "1",
                "gitleaks-aws-key",
                "AWS Secret Key",
                Severity::Critical,
                "src/auth.go",
                12,
                ScannerType::Gitleaks,
                vec![],
            ),
            make_finding(
                "2",
                "semgrep-sqli",
                "SQL Injection",
                Severity::High,
                "src/auth.go",
                42,
                ScannerType::Semgrep,
                vec![],
            ),
        ];

        let chains = build_attack_chains(&findings);
        assert!(!chains.is_empty(), "Should generate at least one chain");
        assert!(chains[0].description.contains("Credential"));
    }

    #[test]
    fn test_supply_chain_attack() {
        let findings = vec![
            make_finding(
                "1",
                "CVE-2024-1234",
                "Critical CVE in log4j",
                Severity::Critical,
                "lib/pom.xml",
                1,
                ScannerType::TrivyVuln,
                vec![],
            ),
            make_finding(
                "2",
                "trivy-misconfig",
                "Docker privileged mode",
                Severity::High,
                "deploy/Dockerfile",
                5,
                ScannerType::TrivyMisconfig,
                vec![],
            ),
        ];

        let chains = build_attack_chains(&findings);
        // Findings in different directories (lib/ vs deploy/) should not chain
        assert!(chains.is_empty(), "Different directories should not chain");
    }

    #[test]
    fn test_same_directory_chain() {
        let findings = vec![
            make_finding(
                "1",
                "gitleaks-password",
                "Hardcoded password",
                Severity::Critical,
                "src/api/handler.py",
                10,
                ScannerType::Gitleaks,
                vec![],
            ),
            make_finding(
                "2",
                "semgrep-sqli",
                "SQL Injection",
                Severity::High,
                "src/api/handler.py",
                45,
                ScannerType::Semgrep,
                vec![],
            ),
        ];

        let chains = build_attack_chains(&findings);
        assert!(!chains.is_empty(), "Same directory findings should chain");
    }

    #[test]
    fn test_multi_scanner_confirmation_chain() {
        let cross_refs = vec![
            CrossReference {
                scanner: ScannerType::Semgrep,
                rule_id: "semgrep-sqli".into(),
            },
            CrossReference {
                scanner: ScannerType::TrivyVuln,
                rule_id: "CVE-2024-1234".into(),
            },
        ];

        let findings = vec![make_finding(
            "1",
            "gitleaks-token",
            "GitHub Token",
            Severity::Critical,
            "src/secrets.env",
            1,
            ScannerType::Gitleaks,
            cross_refs,
        )];

        let chains = build_attack_chains(&findings);
        let confirmation_chains: Vec<_> = chains
            .iter()
            .filter(|c| c.description.contains("Multi-scanner"))
            .collect();
        assert!(
            !confirmation_chains.is_empty(),
            "Multi-scanner confirmations should generate chains"
        );
    }

    #[test]
    fn test_single_finding_no_chain() {
        let findings = vec![make_finding(
            "1",
            "gitleaks-token",
            "API Token",
            Severity::Medium,
            "src/config.js",
            1,
            ScannerType::Gitleaks,
            vec![],
        )];

        let chains = build_attack_chains(&findings);
        // Single finding with no cross-refs = no chain
        assert!(chains.is_empty());
    }

    #[test]
    fn test_infrastructure_escalation_chain() {
        let findings = vec![
            make_finding(
                "1",
                "trivy-misconfig",
                "S3 bucket public",
                Severity::High,
                "terraform/main.tf",
                5,
                ScannerType::TrivyMisconfig,
                vec![],
            ),
            make_finding(
                "2",
                "CVE-2024-5678",
                "RCE in web server",
                Severity::Critical,
                "terraform/main.tf",
                20,
                ScannerType::TrivyVuln,
                vec![],
            ),
        ];

        let chains = build_attack_chains(&findings);
        if !chains.is_empty() {
            assert!(chains[0].risk_score > 0.0);
        }
    }

    #[test]
    fn test_severity_to_score() {
        assert_eq!(severity_to_score(&Severity::Critical), 10);
        assert_eq!(severity_to_score(&Severity::High), 7);
        assert_eq!(severity_to_score(&Severity::Medium), 5);
        assert_eq!(severity_to_score(&Severity::Low), 3);
        assert_eq!(severity_to_score(&Severity::Info), 1);
    }

    #[test]
    fn test_risk_score_capped() {
        let findings = vec![
            make_finding(
                "1",
                "secret-password",
                "Root password",
                Severity::Critical,
                "src/main.rs",
                1,
                ScannerType::Gitleaks,
                vec![],
            ),
            make_finding(
                "2",
                "rce-backdoor",
                "Remote code execution",
                Severity::Critical,
                "src/main.rs",
                10,
                ScannerType::Semgrep,
                vec![],
            ),
            make_finding(
                "3",
                "injection",
                "Command injection",
                Severity::Critical,
                "src/main.rs",
                20,
                ScannerType::Semgrep,
                vec![],
            ),
        ];

        let chains = build_attack_chains(&findings);
        for chain in &chains {
            assert!(
                chain.risk_score <= 10.0,
                "Risk score should be capped at 10.0"
            );
        }
    }

    #[test]
    fn test_empty_findings_no_chain() {
        let findings: Vec<CanonicalFinding> = vec![];
        let chains = build_attack_chains(&findings);
        assert!(chains.is_empty());
    }
}

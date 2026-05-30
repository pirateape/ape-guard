// ApeGuard Finding Normalization
// Post-processes raw findings from all scanners into a unified, enriched format.
// Responsibilities:
//   1. Cross-reference findings across scanners
//   2. Enrich with Zero Trust pillar mappings
//   3. Compute confidence scores
//   4. Tag findings with additional context
use crate::find::{CanonicalFinding, MaturityTier, PillarScore, ZeroTrustScorecard};

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
    // Dependency vulns → Applications pillar
    ("dependency", "applications", MaturityTier::Baseline),
    ("vulnerability", "applications", MaturityTier::Baseline),
    ("cve", "applications", MaturityTier::Baseline),
    // Misconfig → Networks pillar
    ("misconfig", "networks", MaturityTier::Baseline),
    ("iac", "networks", MaturityTier::Advanced),
    ("docker", "networks", MaturityTier::Baseline),
    // General
    ("cwe", "applications", MaturityTier::Baseline),
];

/// Normalize a batch of findings: enrich with ZT mappings, cross-reference, tag
pub fn normalize_findings(findings: &mut [CanonicalFinding]) {
    for finding in findings.iter_mut() {
        // Enrich with Zero Trust pillar mappings
        let rule_lower = finding.rule_id.to_lowercase();
        let title_lower = finding.title.to_lowercase();
        let combined = format!("{} {}", rule_lower, title_lower);

        for (keyword, pillar, _maturity) in ZT_MAPPINGS {
            if combined.contains(keyword) {
                if !finding.zt_pillars.contains(&pillar.to_string()) {
                    finding.zt_pillars.push(pillar.to_string());
                }
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
        "identity", "devices", "networks", "applications",
        "data", "visibility", "automation", "analytics",
    ];

    let mut pillar_findings: HashMap<&str, u32> = HashMap::new();
    let mut pillar_severity: HashMap<&str, u32> = HashMap::new();

    for finding in findings {
        for pillar in &finding.zt_pillars {
            *pillar_findings.entry(pillar).or_insert(0) += 1;
            let sev_weight = match finding.severity {
                crate::find::Severity::Critical => 10,
                crate::find::Severity::High => 5,
                crate::find::Severity::Medium => 2,
                crate::find::Severity::Low => 1,
                crate::find::Severity::Info => 0,
            };
            *pillar_severity.entry(pillar).or_insert(0) += sev_weight;
        }
    }

    let _max_gap_score = 40u32; // 8 pillars × max 5 gaps per pillar
    let mut total_gaps = 0u32;

    let pillar_scores: Vec<PillarScore> = all_pillars
        .iter()
        .map(|name| {
            let count = pillar_findings.get(name).copied().unwrap_or(0);
            let severity_weight = pillar_severity.get(name).copied().unwrap_or(0);
            let gap_count = count.min(5); // Cap at 5 gaps per pillar
            total_gaps += gap_count;

            // Maturity: fewer findings = higher maturity
            let maturity = if count == 0 {
                MaturityTier::Adaptive
            } else if count <= 2 && severity_weight < 5 {
                MaturityTier::Advanced
            } else {
                MaturityTier::Baseline
            };

            let score = (5u32.saturating_sub(gap_count)) * 20; // 0-100 per pillar

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

    ZeroTrustScorecard {
        overall_score,
        max_score,
        pillars: pillar_scores,
        pillars_at_advanced_or_higher: at_advanced,
        target_maturity: MaturityTier::Advanced,
    }
}

/// Enrich a finding with MITRE ATT&CK mapping (simplified)
pub fn mitre_mapping(finding: &CanonicalFinding) -> Vec<String> {
    let rule_lower = finding.rule_id.to_lowercase();
    let title_lower = finding.title.to_lowercase();
    let combined = format!("{} {}", rule_lower, title_lower);

    let mut tactics = Vec::new();

    if combined.contains("secret") || combined.contains("credential") || combined.contains("password") {
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
        }
    }

    #[test]
    fn test_zt_mapping_secret() {
        let mut findings = vec![make_finding("1", "gitleaks-aws-key", "AWS Secret Key Found")];
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
        let mut findings = vec![make_finding("1", "CVE-2024-1234", "Critical vulnerability in dep")];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
    }

    #[test]
    fn test_zt_mapping_misconfig() {
        let mut findings = vec![make_finding("1", "trivy-misconfig-001", "Misconfigured S3 bucket")];
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
        let identity_count = findings[0].zt_pillars.iter().filter(|p| *p == "identity").count();
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
        let identity = sc.pillars.iter().find(|p| p.name == "identity").unwrap();
        assert!(identity.score < 100); // Should lose points
        assert_eq!(identity.gap_count, 1);
    }

    #[test]
    fn test_scorecard_multiple_findings_capped() {
        let mut findings = vec![
            make_finding("1", "secret", "Secret 1"),
            make_finding("2", "secret", "Secret 2"),
            make_finding("3", "secret", "Secret 3"),
            make_finding("4", "secret", "Secret 4"),
            make_finding("5", "secret", "Secret 5"),
            make_finding("6", "secret", "Secret 6"), // 6th capped
        ];
        normalize_findings(&mut findings); // Sets zt_pillars for all
        let sc = compute_zt_scorecard(&findings);
        let identity = sc.pillars.iter().find(|p| p.name == "identity").unwrap();
        assert_eq!(identity.gap_count, 5); // Capped at 5
        assert_eq!(identity.score, 0); // 5 gaps = 0 score
    }

    #[test]
    fn test_scorecard_maturity_levels() {
        // 1 low-severity finding → Advanced
        let mut findings = vec![make_finding("1", "info-rule", "Info level")];
        findings[0].severity = Severity::Low;
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let app = sc.pillars.iter().find(|p| p.name == "applications").unwrap();
        assert_eq!(app.maturity, MaturityTier::Advanced);

        // Multiple high severity → Baseline
        let mut findings = vec![
            make_finding("1", "secret", "Critical 1"),
            make_finding("2", "secret", "Critical 2"),
            make_finding("3", "secret", "Critical 3"),
        ];
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let identity = sc.pillars.iter().find(|p| p.name == "identity").unwrap();
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
        let mut findings = vec![make_finding("1", "CVE-2024-secret", "CVE with credential leak")];
        normalize_findings(&mut findings);
        // Should map to both "applications" (lowercase cve) and "identity" (secret/credential)
        assert!(findings[0].zt_pillars.contains(&"applications".to_string()));
        assert!(findings[0].zt_pillars.contains(&"identity".to_string()));
    }

}

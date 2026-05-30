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
    ("CVE", "applications", MaturityTier::Baseline),
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

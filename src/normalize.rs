// ApeGuard Finding Normalization
// Post-processes raw findings from all scanners into a unified, enriched format.
// Responsibilities:
//   1. Cross-reference findings across scanners
//   2. Enrich with Unified Zero Trust Framework (UZTF) pillar mappings
//   3. Compute confidence scores
//   4. Tag findings with additional context
//
// The UZTF is a 12-pillar maturity model that builds on the CISA Zero Trust
// Maturity Model as its foundational stepping stone. This implementation maps
// security findings to UZTF v2.0 pillars (Identity, Endpoints, IoT & OT,
// Networks, Infrastructure, Applications, Supply Chain, Data, AI Systems,
// Operations, Resilience, Governance) and computes quantitative,
// severity-weighted pillar scores (0-100) and overall scorecard (0-1200).
// See: https://github.com/pirateape/unified-zero-trust-framework
use crate::find::{
    CanonicalFinding, FindingsBySeverity, GapAnalysis, GapLevel, MaturityTier, PillarScore,
    PostureClassification, ScannerType, Severity, ZeroTrustScorecard,
};

/// Zero Trust pillar mapping rules (UZTF v2.0 — 12 pillars).
/// Maps common vulnerability types to ZT pillars. The maturity tier is derived
/// from the severity-weighted score at scoring time, so mappings only need the
/// keyword → pillar association.
const ZT_MAPPINGS: &[(&str, &str)] = &[
    // Secrets / Identity → Identity
    ("secret", "identity"),
    ("credential", "identity"),
    ("password", "identity"),
    ("token", "identity"),
    ("key", "identity"),
    ("jwt", "identity"),
    ("mfa", "identity"),
    ("sso", "identity"),
    ("oauth", "identity"),
    ("session", "identity"),
    // SAST / Code quality → Endpoints
    ("injection", "endpoints"),
    ("xss", "endpoints"),
    ("rce", "endpoints"),
    ("malware", "endpoints"),
    ("buffer", "endpoints"),
    ("memory", "endpoints"),
    ("traversal", "endpoints"),
    ("deserializ", "endpoints"),
    ("xxe", "endpoints"),
    ("dependency", "endpoints"),
    ("vulnerability", "endpoints"),
    ("cve", "endpoints"),
    ("cwe", "endpoints"),
    ("header", "endpoints"),
    // IoT / OT → IoT & OT Systems
    ("iot", "iot_ot"),
    ("ot ", "iot_ot"),
    ("operational technology", "iot_ot"),
    ("scada", "iot_ot"),
    ("modbus", "iot_ot"),
    ("plc", "iot_ot"),
    ("unmanaged device", "iot_ot"),
    ("vlan", "iot_ot"),
    // Network / Perimeter → Networks
    ("misconfig", "networks"),
    ("misconfiguration", "networks"),
    ("ssrf", "networks"),
    ("firewall", "networks"),
    ("port", "networks"),
    ("exposure", "networks"),
    ("dns", "networks"),
    ("segmentation", "networks"),
    ("vpc", "networks"),
    // Cloud / IaC / Host → Infrastructure
    ("iac", "infrastructure"),
    ("docker", "infrastructure"),
    ("kubernetes", "infrastructure"),
    ("k8s", "infrastructure"),
    ("terraform", "infrastructure"),
    ("cloudformation", "infrastructure"),
    ("iam", "infrastructure"),
    ("container", "infrastructure"),
    ("root", "infrastructure"),
    ("hardening", "infrastructure"),
    ("stig", "infrastructure"),
    ("bucket", "infrastructure"),
    ("rbac", "infrastructure"),
    // Web / API → Applications
    ("sqli", "applications"),
    ("sql injection", "applications"),
    ("idor", "applications"),
    ("csrf", "applications"),
    ("ssti", "applications"),
    ("open redirect", "applications"),
    ("api", "applications"),
    ("business logic", "applications"),
    ("auth", "applications"),
    ("authorization", "applications"),
    // SBOM / Vendor / Provenance → Supply Chain
    ("sbom", "supply_chain"),
    ("vendor", "supply_chain"),
    ("typosquat", "supply_chain"),
    ("provenance", "supply_chain"),
    ("unsigned", "supply_chain"),
    ("build pipeline", "supply_chain"),
    ("third-party", "supply_chain"),
    ("transitive", "supply_chain"),
    ("ztna", "supply_chain"),
    // Data Security → Data
    ("pii", "data"),
    ("phi", "data"),
    ("encryption", "data"),
    ("crypto", "data"),
    ("tls", "data"),
    ("ssl", "data"),
    ("cleartext", "data"),
    ("leak", "data"),
    ("dlp", "data"),
    ("classification", "data"),
    ("retention", "data"),
    ("exfiltration", "data"),
    // AI / ML → AI Systems
    ("prompt injection", "ai_systems"),
    ("model inversion", "ai_systems"),
    ("training data", "ai_systems"),
    ("model access", "ai_systems"),
    ("ai deployment", "ai_systems"),
    ("ai output", "ai_systems"),
    ("model supply", "ai_systems"),
    ("llm", "ai_systems"),
    ("ai privacy", "ai_systems"),
    // Logging / Audit / TI → Operations
    ("logging", "operations"),
    ("audit", "operations"),
    ("monitor", "operations"),
    ("observability", "operations"),
    ("telemetry", "operations"),
    ("threat intelligence", "operations"),
    ("alerting", "operations"),
    ("tracing", "operations"),
    ("analytics", "operations"),
    // Backup / IR / BC → Resilience
    ("backup", "resilience"),
    ("recovery", "resilience"),
    ("incident response", "resilience"),
    ("business continuity", "resilience"),
    ("disaster recovery", "resilience"),
    ("isolation", "resilience"),
    ("tabletop", "resilience"),
    ("immutable backup", "resilience"),
    // Training / Risk / Policy → Governance
    ("training", "governance"),
    ("insider threat", "governance"),
    ("policy", "governance"),
    ("compliance", "governance"),
    ("risk scoring", "governance"),
    ("awareness", "governance"),
    ("culture", "governance"),
    ("governance", "governance"),
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
            let mut matched = false;
            for (rule_pattern, severity) in GITLEAKS_SEVERITY_MAP {
                if rule_lower.contains(rule_pattern) {
                    finding.severity = *severity;
                    matched = true;
                    break;
                }
            }
            if !matched {
                if rule_lower.contains("key")
                    || rule_lower.contains("token")
                    || rule_lower.contains("secret")
                    || rule_lower.contains("credential")
                {
                    finding.severity = Severity::High;
                } else {
                    finding.severity = Severity::Medium;
                }
            }
        }

        // Enrich with Zero Trust pillar mappings
        let rule_lower = finding.rule_id.to_lowercase();
        let title_lower = finding.title.to_lowercase();
        let combined = format!("{} {}", rule_lower, title_lower);

        for (keyword, pillar) in ZT_MAPPINGS {
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

/// Severity weight for UZTF v2.0 pillar scoring (SPEC §4.1).
fn severity_weight(sev: Severity) -> u32 {
    match sev {
        Severity::Critical => 20,
        Severity::High => 15,
        Severity::Medium => 10,
        Severity::Low => 5,
        Severity::Info => 1,
    }
}

/// Derive per-pillar maturity tier from the pillar score (SPEC §4.4).
/// Score 100 (no findings) → Adaptive; score ≥ 51 → Advanced; else Baseline.
fn derive_maturity(score: u32) -> MaturityTier {
    if score >= 100 {
        MaturityTier::Adaptive
    } else if score >= 51 {
        MaturityTier::Advanced
    } else {
        MaturityTier::Baseline
    }
}

/// Derive overall posture classification from the overall score (SPEC §4.3).
fn derive_classification(overall: u32) -> PostureClassification {
    if overall >= 961 {
        PostureClassification::MostlyAdaptive
    } else if overall >= 601 {
        PostureClassification::MostlyAdvanced
    } else if overall >= 241 {
        PostureClassification::MostlyBaseline
    } else {
        PostureClassification::Initial
    }
}

/// Compute a Zero Trust scorecard from normalized findings
pub fn compute_zt_scorecard(findings: &[CanonicalFinding]) -> ZeroTrustScorecard {
    use std::collections::HashMap;

    let all_pillars = [
        "identity",
        "endpoints",
        "iot_ot",
        "networks",
        "infrastructure",
        "applications",
        "supply_chain",
        "data",
        "ai_systems",
        "operations",
        "resilience",
        "governance",
    ];

    // Track severity-weighted deduction and finding refs per pillar
    let mut pillar_deduction: HashMap<&str, u32> = HashMap::new();
    let mut pillar_finding_refs: HashMap<&str, Vec<&CanonicalFinding>> = HashMap::new();

    for finding in findings {
        let weight = severity_weight(finding.severity);
        for pillar in &finding.zt_pillars {
            *pillar_deduction.entry(pillar).or_insert(0) += weight;
            pillar_finding_refs.entry(pillar).or_default().push(finding);
        }
    }

    let pillar_scores: Vec<PillarScore> = all_pillars
        .iter()
        .map(|name| {
            let deduction = pillar_deduction.get(name).copied().unwrap_or(0);
            let gap_count = pillar_finding_refs
                .get(name)
                .map(|refs| refs.len() as u32)
                .unwrap_or(0);

            // Score: 100 - severity-weighted deduction, floored at 0 (SPEC §4.1)
            let score = 100u32.saturating_sub(deduction);
            let maturity = derive_maturity(score);

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
    let classification = derive_classification(overall_score);

    // Compute gap analysis
    let gap_analysis = compute_gap_analysis(&all_pillars, &pillar_deduction, &pillar_finding_refs);

    ZeroTrustScorecard {
        overall_score,
        max_score,
        pillars: pillar_scores,
        pillars_at_advanced_or_higher: at_advanced,
        target_maturity: MaturityTier::Advanced,
        classification,
        gap_analysis,
    }
}

/// Compute detailed gap analysis for each pillar (severity-weighted).
pub fn compute_gap_analysis(
    all_pillars: &[&str],
    pillar_deduction: &std::collections::HashMap<&str, u32>,
    pillar_finding_refs: &std::collections::HashMap<&str, Vec<&CanonicalFinding>>,
) -> Vec<GapAnalysis> {
    let target = MaturityTier::Advanced;
    let mut analysis = Vec::new();

    for pillar_name in all_pillars {
        let deduction = pillar_deduction.get(*pillar_name).copied().unwrap_or(0);
        let score = 100u32.saturating_sub(deduction);
        let current = derive_maturity(score);

        // Compute gap level (SPEC §5.2)
        let gap = match (&current, &target) {
            (MaturityTier::Adaptive, _) => GapLevel::None, // Exceeded target
            (MaturityTier::Advanced, MaturityTier::Advanced) => GapLevel::None,
            (MaturityTier::Baseline, MaturityTier::Advanced) => {
                if deduction >= 51 {
                    GapLevel::Large // 2-tier gap (Baseline → Adaptive)
                } else {
                    GapLevel::Small // 1-tier gap (Baseline → Advanced)
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

        // Severity breakdown of blocking findings
        let mut sev_counts = FindingsBySeverity {
            critical: 0,
            high: 0,
            medium: 0,
            low: 0,
            info: 0,
        };
        if let Some(refs) = pillar_finding_refs.get(*pillar_name) {
            for f in refs {
                match f.severity {
                    Severity::Critical => sev_counts.critical += 1,
                    Severity::High => sev_counts.high += 1,
                    Severity::Medium => sev_counts.medium += 1,
                    Severity::Low => sev_counts.low += 1,
                    Severity::Info => sev_counts.info += 1,
                }
            }
        }
        let blocking_count = sev_counts.critical
            + sev_counts.high
            + sev_counts.medium
            + sev_counts.low
            + sev_counts.info;

        analysis.push(GapAnalysis {
            pillar: pillar_name.to_string(),
            current_maturity: current,
            target_maturity: target.clone(),
            gap,
            blocking_findings: blocking_count,
            total_deduction: deduction,
            findings_by_severity: sev_counts,
            recommendations,
        });
    }

    analysis
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
        assert!(findings[0].zt_pillars.contains(&"endpoints".to_string()));
    }

    #[test]
    fn test_zt_mapping_xss() {
        let mut findings = vec![make_finding("1", "semgrep-xss", "XSS vulnerability")];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"endpoints".to_string()));
    }

    #[test]
    fn test_zt_mapping_dependency() {
        let mut findings = vec![make_finding(
            "1",
            "CVE-2024-1234",
            "Critical vulnerability in dep",
        )];
        normalize_findings(&mut findings);
        assert!(findings[0].zt_pillars.contains(&"endpoints".to_string()));
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
        assert_eq!(sc.overall_score, 1200); // 12 pillars × 100
        assert_eq!(sc.pillars.len(), 12);
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
        assert!(identity.score < 100); // Should lose points (High weight=15)
        assert_eq!(identity.gap_count, 1); // 1 finding
    }

    #[test]
    fn test_scorecard_multiple_findings() {
        let mut findings = vec![
            make_finding("1", "secret", "Secret 1"),
            make_finding("2", "secret", "Secret 2"),
            make_finding("3", "secret", "Secret 3"),
            make_finding("4", "secret", "Secret 4"),
            make_finding("5", "secret", "Secret 5"),
            make_finding("6", "secret", "Secret 6"),
        ];
        normalize_findings(&mut findings); // Sets zt_pillars for all
        let sc = compute_zt_scorecard(&findings);
        let identity = sc
            .pillars
            .iter()
            .find(|p| p.name == "identity")
            .expect("normalize test: identity pillar should exist");
        assert_eq!(identity.gap_count, 6); // 6 findings
        assert_eq!(identity.score, 10); // 6 High × 15 = 90 deduction → 10
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

        // 4 high-severity findings → 60 deduction → score 40 → Baseline
        let mut findings = vec![
            make_finding("1", "secret", "Critical 1"),
            make_finding("2", "secret", "Critical 2"),
            make_finding("3", "secret", "Critical 3"),
            make_finding("4", "secret", "Critical 4"),
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
        // "cve" → endpoints, "credential" → identity, "leak" → data
        assert!(findings[0].zt_pillars.contains(&"endpoints".to_string()));
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
        assert_eq!(sc.gap_analysis.len(), 12);
    }

    #[test]
    fn test_gap_analysis_with_secrets() {
        let mut findings = vec![
            make_finding("1", "secret-key", "AWS Secret Key"),
            make_finding("2", "secret-password", "Hardcoded Password"),
            make_finding("3", "secret-token", "API Token"),
            make_finding("4", "secret-cred", "Client Credential"),
        ];
        // Gitleaks override sets these to High (15 each) → 4 × 15 = 60 deduction
        // → score 40 → Baseline
        normalize_findings(&mut findings);
        let sc = compute_zt_scorecard(&findings);
        let identity_ga = sc
            .gap_analysis
            .iter()
            .find(|g| g.pillar == "identity")
            .expect("normalize test: identity gap analysis should exist");
        assert_eq!(identity_ga.current_maturity, MaturityTier::Baseline);
        assert_eq!(identity_ga.blocking_findings, 4);
        assert_eq!(identity_ga.total_deduction, 60);
    }
}

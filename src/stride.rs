// ApeGuard STRIDE Threat Modeling Integration (Phase 2.1)
//
// Maps each security finding to one or more STRIDE categories
// (Spoofing, Tampering, Repudiation, Information Disclosure,
//  Denial of Service, Elevation of Privilege) and analyses
// coverage across the threat model.
//
// STRIDE is a Microsoft threat classification taxonomy developed
// by Loren Kohnfelder and Praerit Garg in 1999. This module
// maps scanner findings to STRIDE categories via keyword matching
// on rule_id, title, CWE, tags, and scanner type.
//
// The coverage analysis identifies which threat categories are
// well-covered by the current scan configuration and which have
// gaps — enabling teams to tune scanner selection and rule sets.

use crate::find::CanonicalFinding;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// STRIDE Category Enum
// ---------------------------------------------------------------------------

/// The six STRIDE threat categories.
///
/// STRIDE is a mnemonic for six categories of security threats:
/// - **Spoofing** – impersonating a user, process, or device
/// - **Tampering** – modifying data or code in unauthorised ways
/// - **Repudiation** – denying an action without the ability to prove otherwise
/// - **Information Disclosure** – exposing data to unauthorised parties
/// - **Denial of Service** – degrading or denying access to services
/// - **Elevation of Privilege** – gaining unauthorised access to higher privileges
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrideCategory {
    Spoofing,
    Tampering,
    Repudiation,
    InformationDisclosure,
    DenialOfService,
    ElevationOfPrivilege,
}

impl StrideCategory {
    /// Human-readable label
    pub fn label(&self) -> &'static str {
        match self {
            StrideCategory::Spoofing => "Spoofing",
            StrideCategory::Tampering => "Tampering",
            StrideCategory::Repudiation => "Repudiation",
            StrideCategory::InformationDisclosure => "Information Disclosure",
            StrideCategory::DenialOfService => "Denial of Service",
            StrideCategory::ElevationOfPrivilege => "Elevation of Privilege",
        }
    }

    /// Short identifier (for report sections)
    pub fn id(&self) -> &'static str {
        match self {
            StrideCategory::Spoofing => "S",
            StrideCategory::Tampering => "T",
            StrideCategory::Repudiation => "R",
            StrideCategory::InformationDisclosure => "I",
            StrideCategory::DenialOfService => "D",
            StrideCategory::ElevationOfPrivilege => "E",
        }
    }

    /// Description of the threat category
    pub fn description(&self) -> &'static str {
        match self {
            StrideCategory::Spoofing => {
                "Impersonating a user, process, or device to gain unauthorised access"
            }
            StrideCategory::Tampering => "Modifying data or code in unauthorised ways",
            StrideCategory::Repudiation => {
                "Denying an action without the ability for others to prove otherwise"
            }
            StrideCategory::InformationDisclosure => {
                "Exposing data to parties who should not have access"
            }
            StrideCategory::DenialOfService => {
                "Degrading or denying access to services or resources"
            }
            StrideCategory::ElevationOfPrivilege => {
                "Gaining unauthorised access to higher privilege levels"
            }
        }
    }

    /// All six categories in STRIDE order
    pub fn all() -> [StrideCategory; 6] {
        [
            StrideCategory::Spoofing,
            StrideCategory::Tampering,
            StrideCategory::Repudiation,
            StrideCategory::InformationDisclosure,
            StrideCategory::DenialOfService,
            StrideCategory::ElevationOfPrivilege,
        ]
    }
}

// ---------------------------------------------------------------------------
// Coverage Types
// ---------------------------------------------------------------------------

/// Per-category coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrideCoverage {
    /// The STRIDE category
    pub category: StrideCategory,
    /// Number of findings mapped to this category
    pub finding_count: usize,
    /// Proportion of total findings that map to this category (0.0–1.0)
    pub coverage_ratio: f64,
    /// Whether this category is considered "covered" (ratio >= threshold)
    pub covered: bool,
}

/// Overall STRIDE coverage analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrideResult {
    /// Per-category coverage
    pub coverage: Vec<StrideCoverage>,
    /// Total number of findings analysed
    pub total_findings: usize,
    /// Number of categories with at least one finding mapped
    pub covered_categories: usize,
    /// Total coverage score: covered_categories / 6 (0.0–1.0)
    pub coverage_score: f64,
    /// Categories with no findings mapped (gaps)
    pub gaps: Vec<StrideCategory>,
    /// Coverage threshold used for analysis
    pub threshold: f64,
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// STRIDE analysis configuration
///
/// Placed here for co-location with the analysis logic; also mirrored in
/// `config::Config` via a `#[serde(default)]` field for file/CLI merging.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // P3/P4: STRIDE config parsed but analysis not yet implemented
pub struct StrideConfig {
    /// Master switch — STRIDE analysis is opt-in (default: false)
    pub enabled: bool,
    /// Minimum coverage ratio (0.0–1.0) for a category to be considered "covered"
    pub coverage_threshold: f64,
}

impl Default for StrideConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            coverage_threshold: 0.05, // 5% of findings = "covered"
        }
    }
}

// ---------------------------------------------------------------------------
// Mapping Logic
// ---------------------------------------------------------------------------

/// Map a single finding to the STRIDE categories it represents.
///
/// Uses keyword matching on the combined lowercased `rule_id`, `title`,
/// `cwe`, and `tags` fields. Each category has a distinct set of trigger
/// keywords derived from real-world security scanner output patterns.
pub fn map_finding_to_stride(finding: &CanonicalFinding) -> Vec<StrideCategory> {
    let rule_lower = finding.rule_id.to_lowercase();
    let title_lower = finding.title.to_lowercase();
    let cwe_lower = finding.cwe.as_deref().unwrap_or("").to_lowercase();
    let tags_lower: Vec<String> = finding.tags.iter().map(|t| t.to_lowercase()).collect();
    let combined = format!("{} {}", rule_lower, title_lower);
    let all_text = format!(
        "{} {} {} {}",
        rule_lower,
        title_lower,
        cwe_lower,
        tags_lower.join(" ")
    );

    let mut categories = Vec::new();

    // --- Spoofing ---
    // Covers: authentication bypass, session hijacking, JWT/Token forgery,
    //         OAuth misconfiguration, SAML attacks, weak certificate validation
    if combined.contains("spoof")
        || combined.contains("authn")
        || combined.contains("authent")
        || combined.contains("session")
        || combined.contains("jwt")
        || combined.contains("oauth")
        || combined.contains("saml")
        || combined.contains("sso")
        || combined.contains("token")
        || combined.contains("login")
        || combined.contains("credential")
        || combined.contains("password")
        || combined.contains("secret")
        || combined.contains("certificate")
        || combined.contains("mfa")
        || combined.contains("2fa")
        || combined.contains("otp")
        || combined.contains("csrf")
        || combined.contains("fixation")
        || combined.contains("idp")
        || all_text.contains("cwe-287") // Improper Authentication
        || all_text.contains("cwe-613") // Session Fixation
        || all_text.contains("cwe-798") // Hardcoded Credentials
        || all_text.contains("cwe-521")
    // Weak Password Requirements
    {
        categories.push(StrideCategory::Spoofing);
    }

    // --- Tampering ---
    // Covers: code injection, data modification, integrity violations,
    //         prototype pollution, mass assignment, race conditions
    if combined.contains("tamper")
        || combined.contains("injection")
        || combined.contains("sqli")
        || combined.contains("xss")
        || combined.contains("command")
        || combined.contains("ldapi")
        || combined.contains("path traversal")
        || combined.contains("lfi")
        || combined.contains("rfi")
        || combined.contains("xxe")
        || combined.contains("prototype pollution")
        || combined.contains("mass assignment")
        || combined.contains("race condition")
        || combined.contains("toctou")
        || combined.contains("format string")
        || combined.contains("buffer overflow")
        || combined.contains("integer overflow")
        || combined.contains("mutation")
        || combined.contains("modif")
        || combined.contains("integrity")
        || combined.contains("ssti")
        || combined.contains("template")
        || combined.contains("deserialization")
        || combined.contains("untrusted data")
        || combined.contains("cors")
        || all_text.contains("cwe-79")   // XSS
        || all_text.contains("cwe-89")   // SQL Injection
        || all_text.contains("cwe-94")   // Code Injection
        || all_text.contains("cwe-77")   // Command Injection
        || all_text.contains("cwe-611")  // XXE
        || all_text.contains("cwe-915")  // Mass Assignment
        || all_text.contains("cwe-502")
    // Deserialisation
    {
        categories.push(StrideCategory::Tampering);
    }

    // --- Repudiation ---
    // Covers: missing audit logs, insufficient logging, lack of non-repudiation,
    //         tampered logs, missing trails
    if combined.contains("repudiation")
        || combined.contains("audit")
        || combined.contains("logging")
        || combined.contains("non-repudiation")
        || combined.contains("log")
        || combined.contains("trail")
        || combined.contains("accounting")
        || combined.contains("forensic")
        || combined.contains("telemetry")
        || combined.contains("monitoring")
        || combined.contains("observability")
    {
        categories.push(StrideCategory::Repudiation);
    }

    // --- Information Disclosure ---
    // Covers: data leaks, secret exposure, PII leakage, directory listing,
    //         source map exposure, error message leaks, side channels
    if combined.contains("secret")
        || combined.contains("credential")
        || combined.contains("password")
        || combined.contains("api key")
        || combined.contains("token")
        || combined.contains("leak")
        || combined.contains("disclosure")
        || combined.contains("exposure")
        || combined.contains("information")
        || combined.contains("pii")
        || combined.contains("sensitive")
        || combined.contains("source map")
        || combined.contains(".env")
        || combined.contains("directory listing")
        || combined.contains("error message")
        || combined.contains("stack trace")
        || combined.contains("side channel")
        || combined.contains("timing")
        || combined.contains("oracle")
        || combined.contains("information disclosure")
        || combined.contains("ssrf")
        || combined.contains("idor")
        || combined.contains("insecure direct object")
        || combined.contains("misconfig")
        || combined.contains("bucket")
        || combined.contains("s3")
        || combined.contains("storage")
        || combined.contains("backup")
        || combined.contains("dump")
        || combined.contains("heap")
        || combined.contains("memory")
        || combined.contains("snapshot")
        || all_text.contains("cwe-200")  // Information Exposure
        || all_text.contains("cwe-201")  // Information Exposure Through Sent Data
        || all_text.contains("cwe-312")  // Cleartext Storage of Sensitive Information
        || all_text.contains("cwe-319")  // Cleartext Transmission
        || all_text.contains("cwe-532")  // Information Exposure Through Log Files
        || all_text.contains("cwe-548")
    // Directory Listing
    {
        categories.push(StrideCategory::InformationDisclosure);
    }

    // --- Denial of Service ---
    // Covers: resource exhaustion, rate limiting, DoS amplification,
    //         deadlocks, crashes, infinite loops, resource consumption
    if combined.contains("denial")
        || combined.contains("dos")
        || combined.contains("ddos")
        || combined.contains("resource exhaustion")
        || combined.contains("rate limit")
        || combined.contains("throttle")
        || combined.contains("crash")
        || combined.contains("infinite loop")
        || combined.contains("deadlock")
        || combined.contains("resource leak")
        || combined.contains("memory leak")
        || combined.contains("cpu")
        || combined.contains("amplification")
        || combined.contains("flood")
        || combined.contains("timeout")
        || combined.contains("hang")
        || combined.contains("unresponsive")
        || combined.contains("availability")
        || combined.contains("reentrancy")
        || all_text.contains("cwe-400")  // Resource Exhaustion
        || all_text.contains("cwe-770")  // Allocation without Limits
        || all_text.contains("cwe-834")
    // Excessive Iteration
    {
        categories.push(StrideCategory::DenialOfService);
    }

    // --- Elevation of Privilege ---
    // Covers: privilege escalation, RCE, sandbox escape, ACL bypass,
    //         role escalation, container escape, sudo misconfig
    if combined.contains("privilege")
        || combined.contains("escalation")
        || combined.contains("rce")
        || combined.contains("remote code execution")
        || combined.contains("eop")
        || combined.contains("root")
        || combined.contains("admin")
        || combined.contains("sandbox")
        || combined.contains("escape")
        || combined.contains("container")
        || combined.contains("acl")
        || combined.contains("permission")
        || combined.contains("authorization")
        || combined.contains("role")
        || combined.contains("sudo")
        || combined.contains("capability")
        || combined.contains("chmod")
        || combined.contains("chown")
        || combined.contains("setuid")
        || combined.contains("setgid")
        || combined.contains("shell")
        || combined.contains("backdoor")
        || combined.contains("webshell")
        || combined.contains("deserialization")
        || combined.contains("buffer overflow")
        || combined.contains("integer overflow")
        || combined.contains("use-after-free")
        || combined.contains("dangling pointer")
        || combined.contains("type confusion")
        || all_text.contains("cwe-269")  // Improper Privilege Management
        || all_text.contains("cwe-276")  // Incorrect Default Permissions
        || all_text.contains("cwe-732")  // Incorrect Permission Assignment
        || all_text.contains("cwe-94")   // Code Injection
        || all_text.contains("cwe-77")
    // Command Injection
    {
        categories.push(StrideCategory::ElevationOfPrivilege);
    }

    categories
}

// ---------------------------------------------------------------------------
// Coverage Analysis
// ---------------------------------------------------------------------------

/// Analyse STRIDE coverage across a set of findings.
///
/// Returns a `StrideResult` with per-category counts, coverage ratios,
/// gap identification, and an overall coverage score.
///
/// # Parameters
/// - `findings`: slice of normalised findings to analyse
/// - `threshold`: minimum coverage ratio (0.0–1.0) for a category
///   to be considered "covered". The default of 0.05 means a category
///   needs at least 5% of all findings mapped to it.
pub fn analyze_stride_coverage(findings: &[CanonicalFinding], threshold: f64) -> StrideResult {
    let total_findings = findings.len();
    let mut category_counts: Vec<(StrideCategory, usize)> =
        StrideCategory::all().iter().map(|&c| (c, 0)).collect();

    // Map each finding to its STRIDE categories and count
    for finding in findings {
        let categories = map_finding_to_stride(finding);
        for cat in categories {
            if let Some(entry) = category_counts.iter_mut().find(|(c, _)| *c == cat) {
                entry.1 += 1;
            }
        }
    }

    let effective_threshold = if total_findings == 0 {
        1.0 // No findings = empty = no coverage
    } else {
        threshold
    };

    let mut coverage = Vec::with_capacity(6);
    let mut covered_count = 0usize;
    let mut gaps = Vec::new();

    for (category, count) in &category_counts {
        let ratio = if total_findings > 0 {
            *count as f64 / total_findings as f64
        } else {
            0.0
        };
        let covered = ratio >= effective_threshold;
        if covered {
            covered_count += 1;
        } else {
            gaps.push(*category);
        }
        coverage.push(StrideCoverage {
            category: *category,
            finding_count: *count,
            coverage_ratio: ratio,
            covered,
        });
    }

    let coverage_score = if total_findings == 0 {
        0.0
    } else {
        covered_count as f64 / 6.0
    };

    StrideResult {
        coverage,
        total_findings,
        covered_categories: covered_count,
        coverage_score,
        gaps,
        threshold: effective_threshold,
    }
}

// ---------------------------------------------------------------------------
// Formatting helpers
// ---------------------------------------------------------------------------

/// Format a STRIDE coverage result as a human-readable table for reports.
pub fn format_stride_table(result: &StrideResult) -> String {
    let mut lines = Vec::new();
    lines.push("| Category | Count | Coverage | Status |".to_string());
    lines.push("|----------|-------|----------|--------|".to_string());

    for cov in &result.coverage {
        let pct = cov.coverage_ratio * 100.0;
        let status = if cov.covered { "✅" } else { "⚠️ Gap" };
        lines.push(format!(
            "| {} ({}) | {} | {:.1}% | {} |",
            cov.category.label(),
            cov.category.id(),
            cov.finding_count,
            pct,
            status,
        ));
    }

    lines.push(String::new());
    lines.push(format!(
        "**Coverage Score**: {:.0}% ({}/6 categories covered)",
        result.coverage_score * 100.0,
        result.covered_categories,
    ));

    if !result.gaps.is_empty() {
        lines.push(String::new());
        lines.push("**Gaps (uncovered categories):**".to_string());
        for gap in &result.gaps {
            lines.push(format!("- **{}**: {}", gap.label(), gap.description()));
        }
    }

    lines.join("\n")
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::*;
    use std::path::PathBuf;

    // Helper — builds a minimal CanonicalFinding for testing
    fn make_finding(
        id: &str,
        rule_id: &str,
        title: &str,
        cwe: Option<&str>,
        tags: Vec<&str>,
    ) -> CanonicalFinding {
        CanonicalFinding {
            id: id.into(),
            scanner: ScannerType::Semgrep,
            scanner_version: None,
            rule_id: rule_id.into(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: title.into(),
            description: "test description".into(),
            location: FindingLocation {
                file: PathBuf::from("test.py"),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: cwe.map(String::from),
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: tags.into_iter().map(String::from).collect(),
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        }
    }

    // ------------------------------------------------------------------
    // StrideCategory tests
    // ------------------------------------------------------------------

    #[test]
    fn test_category_labels() {
        assert_eq!(StrideCategory::Spoofing.label(), "Spoofing");
        assert_eq!(StrideCategory::Tampering.label(), "Tampering");
        assert_eq!(StrideCategory::Repudiation.label(), "Repudiation");
        assert_eq!(
            StrideCategory::InformationDisclosure.label(),
            "Information Disclosure"
        );
        assert_eq!(StrideCategory::DenialOfService.label(), "Denial of Service");
        assert_eq!(
            StrideCategory::ElevationOfPrivilege.label(),
            "Elevation of Privilege"
        );
    }

    #[test]
    fn test_category_ids() {
        assert_eq!(StrideCategory::Spoofing.id(), "S");
        assert_eq!(StrideCategory::Tampering.id(), "T");
        assert_eq!(StrideCategory::Repudiation.id(), "R");
        assert_eq!(StrideCategory::InformationDisclosure.id(), "I");
        assert_eq!(StrideCategory::DenialOfService.id(), "D");
        assert_eq!(StrideCategory::ElevationOfPrivilege.id(), "E");
    }

    #[test]
    fn test_all_categories() {
        let all = StrideCategory::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0] as u8, 0); // Spoofing
        assert_eq!(all[5] as u8, 5); // ElevationOfPrivilege
    }

    // ------------------------------------------------------------------
    // Mapping tests — each category
    // ------------------------------------------------------------------

    #[test]
    fn test_map_secret_to_spoofing_and_disclosure() {
        let f = make_finding(
            "1",
            "gitleaks-aws-key",
            "AWS Secret Key Found",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(
            cats.contains(&StrideCategory::Spoofing),
            "secrets should map to Spoofing (credential)"
        );
        assert!(
            cats.contains(&StrideCategory::InformationDisclosure),
            "secrets should map to Information Disclosure"
        );
    }

    #[test]
    fn test_map_sqli_to_tampering() {
        let f = make_finding(
            "2",
            "semgrep-sqli",
            "SQL Injection detected",
            Some("CWE-89"),
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Tampering));
        // SQLi can also lead to info disclosure
    }

    #[test]
    fn test_map_xss_to_tampering() {
        let f = make_finding(
            "3",
            "semgrep-xss",
            "XSS vulnerability",
            Some("CWE-79"),
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Tampering));
    }

    #[test]
    fn test_map_rce_to_elevation_of_privilege() {
        let f = make_finding(
            "4",
            "semgrep-rce",
            "Remote Code Execution",
            Some("CWE-94"),
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::ElevationOfPrivilege));
        assert!(cats.contains(&StrideCategory::Tampering)); // injection → tampering
    }

    #[test]
    fn test_map_ssrf_to_information_disclosure() {
        let f = make_finding(
            "5",
            "nuclei-ssrf",
            "Server-Side Request Forgery",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::InformationDisclosure));
    }

    #[test]
    fn test_map_idor_to_information_disclosure() {
        let f = make_finding(
            "6",
            "semgrep-idor",
            "Insecure Direct Object Reference",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::InformationDisclosure));
    }

    #[test]
    fn test_map_jwt_to_spoofing() {
        let f = make_finding(
            "7",
            "semgrep-jwt",
            "JWT token validation bypass",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Spoofing));
    }

    #[test]
    fn test_map_xxe_to_tampering() {
        let f = make_finding("8", "semgrep-xxe", "XXE injection", Some("CWE-611"), vec![]);
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Tampering));
    }

    #[test]
    fn test_map_prototype_pollution_to_tampering() {
        let f = make_finding(
            "9",
            "semgrep-proto-pollution",
            "Prototype Pollution vulnerability",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Tampering));
    }

    #[test]
    fn test_map_rate_limit_to_dos() {
        let f = make_finding(
            "10",
            "nuclei-rate-limit",
            "Missing Rate Limiting",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::DenialOfService));
    }

    #[test]
    fn test_map_privilege_escalation() {
        let f = make_finding(
            "11",
            "semgrep-privesc",
            "Privilege Escalation via sudo",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::ElevationOfPrivilege));
    }

    #[test]
    fn test_map_misconfig_to_disclosure() {
        let f = make_finding(
            "12",
            "checkov-s3-public",
            "S3 Bucket public access misconfiguration",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::InformationDisclosure));
    }

    #[test]
    fn test_map_audit_logging_to_repudiation() {
        let f = make_finding(
            "13",
            "semgrep-audit",
            "Missing audit logging in sensitive operations",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Repudiation));
    }

    #[test]
    fn test_map_csrf_to_spoofing() {
        let f = make_finding(
            "14",
            "semgrep-csrf",
            "Cross-Site Request Forgery",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Spoofing));
    }

    #[test]
    fn test_map_container_escape_to_eop() {
        let f = make_finding(
            "15",
            "trivy-container-escape",
            "Container escape via capability",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::ElevationOfPrivilege));
    }

    #[test]
    fn test_map_injection_to_multiple_categories() {
        let f = make_finding(
            "16",
            "semgrep-command-injection",
            "OS Command Injection",
            Some("CWE-77"),
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(
            cats.contains(&StrideCategory::Tampering),
            "injection → Tampering"
        );
        assert!(
            cats.contains(&StrideCategory::ElevationOfPrivilege),
            "CWE-77 → EoP"
        );
    }

    #[test]
    fn test_map_deserialization_to_tampering_and_eop() {
        let f = make_finding(
            "17",
            "semgrep-deserialization",
            "Insecure Deserialization",
            Some("CWE-502"),
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.contains(&StrideCategory::Tampering));
        assert!(cats.contains(&StrideCategory::ElevationOfPrivilege));
    }

    #[test]
    fn test_map_no_keywords_returns_empty() {
        let f = make_finding(
            "18",
            "unknown-rule",
            "Some harmless info message",
            None,
            vec![],
        );
        let cats = map_finding_to_stride(&f);
        assert!(cats.is_empty());
    }

    // ------------------------------------------------------------------
    // Coverage analysis tests
    // ------------------------------------------------------------------

    #[test]
    fn test_empty_findings_no_coverage() {
        let result = analyze_stride_coverage(&[], 0.05);
        assert_eq!(result.total_findings, 0);
        assert_eq!(result.covered_categories, 0);
        assert_eq!(result.gaps.len(), 6);
        assert_eq!(result.coverage_score, 0.0);
    }

    #[test]
    fn test_single_finding_single_category() {
        let f = make_finding(
            "1",
            "gitleaks-aws-key",
            "AWS Secret Key Found",
            None,
            vec![],
        );
        let result = analyze_stride_coverage(&[f], 0.05);
        assert_eq!(result.total_findings, 1);
        // Secret maps to Spoofing + InformationDisclosure = 2 categories
        assert_eq!(result.covered_categories, 2);
        assert_eq!(result.gaps.len(), 4);
        assert!((result.coverage_score - 2.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_mixed_findings_coverage() {
        let findings = vec![
            make_finding(
                "1",
                "gitleaks-aws-key",
                "AWS Secret Key Found",
                None,
                vec![],
            ),
            make_finding("2", "semgrep-sqli", "SQL Injection", Some("CWE-89"), vec![]),
            make_finding(
                "3",
                "semgrep-rce",
                "Remote Code Execution",
                Some("CWE-94"),
                vec![],
            ),
            make_finding("4", "semgrep-audit", "Missing audit logging", None, vec![]),
        ];
        let result = analyze_stride_coverage(&findings, 0.01);
        assert_eq!(result.total_findings, 4);
        // Should cover: Spoofing, Tampering, Repudiation, InfoDisclosure, EoP
        assert_eq!(result.covered_categories, 5);
        assert_eq!(result.gaps, vec![StrideCategory::DenialOfService]);
    }

    #[test]
    fn test_all_categories_covered() {
        let findings = vec![
            make_finding("S", "jwt-bypass", "JWT authentication bypass", None, vec![]),
            make_finding("T", "semgrep-sqli", "SQL Injection", None, vec![]),
            make_finding("R", "missing-audit", "Missing audit trail", None, vec![]),
            make_finding("I", "secret-leak", "API key leaked", None, vec![]),
            make_finding("D", "rate-limit", "No rate limiting on API", None, vec![]),
            make_finding("E", "rce-vuln", "Remote Code Execution", None, vec![]),
        ];
        let result = analyze_stride_coverage(&findings, 0.01);
        assert_eq!(result.total_findings, 6);
        assert_eq!(result.covered_categories, 6);
        assert!(result.gaps.is_empty());
    }

    #[test]
    fn test_threshold_filtering() {
        // Many findings of one type, one finding of another
        let mut findings = Vec::new();
        for i in 0..20 {
            findings.push(make_finding(
                &format!("s{}", i),
                "gitleaks-aws-key",
                "AWS Secret Key Found",
                None,
                vec![],
            ));
        }
        // One SQLi finding — 1/21 ≈ 4.8% → below 5% threshold
        findings.push(make_finding(
            "sqli",
            "semgrep-sqli",
            "SQL Injection",
            None,
            vec![],
        ));

        // With 5% threshold, Tampering should be a gap
        let result = analyze_stride_coverage(&findings, 0.05);
        assert_eq!(result.total_findings, 21);
        assert!(result.gaps.contains(&StrideCategory::Tampering));

        // With 1% threshold, Tampering should be covered
        let result2 = analyze_stride_coverage(&findings, 0.01);
        assert!(!result2.gaps.contains(&StrideCategory::Tampering));
    }

    #[test]
    fn test_coverage_ratio_calculation() {
        let findings = vec![
            make_finding(
                "1",
                "gitleaks-aws-key",
                "AWS Secret Key Found",
                None,
                vec![],
            ),
            make_finding("2", "semgrep-sqli", "SQL Injection", None, vec![]),
        ];
        let result = analyze_stride_coverage(&findings, 0.01);
        // 2 findings. Secret → Spoofing + InfoDisclosure = 2.
        // SQLi → Tampering = 1.
        // Repudiation, DoS, EoP = 0.
        let spoof = result
            .coverage
            .iter()
            .find(|c| c.category == StrideCategory::Spoofing)
            .unwrap();
        let tamper = result
            .coverage
            .iter()
            .find(|c| c.category == StrideCategory::Tampering)
            .unwrap();
        let repud = result
            .coverage
            .iter()
            .find(|c| c.category == StrideCategory::Repudiation)
            .unwrap();

        assert!((spoof.coverage_ratio - 1.0 / 2.0).abs() < 1e-10); // 1/2
        assert!((tamper.coverage_ratio - 1.0 / 2.0).abs() < 1e-10); // 1/2
        assert!((repud.coverage_ratio).abs() < 1e-10); // 0
    }

    // ------------------------------------------------------------------
    // Formatting tests
    // ------------------------------------------------------------------

    #[test]
    fn test_format_stride_table() {
        let result = analyze_stride_coverage(&[], 0.05);
        let table = format_stride_table(&result);
        assert!(table.contains("Coverage Score"));
        assert!(table.contains("0%"));
        assert!(table.contains("Gap"));
    }

    #[test]
    fn test_format_stride_table_with_findings() {
        let f = make_finding(
            "1",
            "gitleaks-aws-key",
            "AWS Secret Key Found",
            None,
            vec![],
        );
        let result = analyze_stride_coverage(&[f], 0.05);
        let table = format_stride_table(&result);
        assert!(table.contains("Spoofing"));
        assert!(table.contains("Information Disclosure"));
        assert!(table.contains("Coverage Score"));
    }

    // ------------------------------------------------------------------
    // StrideConfig tests
    // ------------------------------------------------------------------

    #[test]
    fn test_stride_config_default() {
        let cfg = StrideConfig::default();
        assert!(!cfg.enabled);
        assert!((cfg.coverage_threshold - 0.05).abs() < 1e-10);
    }
}

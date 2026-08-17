// Canonical Finding Schema
// All scanner output is normalized into this structure for analysis.
//
// Zero Trust types in this module (ZeroTrustScorecard, PillarScore, MaturityTier,
// GapAnalysis) implement the Unified Zero Trust Framework (UZTF) v2.0 — a 12-pillar
// maturity model that builds on the CISA Zero Trust Maturity Model.
// See: https://github.com/pirateape/unified-zero-trust-framework
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Normalized finding produced by any scanner.
///
/// All scanner output is converted into this canonical structure
/// for unified analysis, deduplication, scoring, and reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalFinding {
    pub id: String,
    pub scanner: ScannerType,
    pub scanner_version: Option<String>,
    pub rule_id: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub title: String,
    pub description: String,
    pub location: FindingLocation,
    pub cwe: Option<String>,
    pub cvss: Option<f32>,
    pub remediation: Option<String>,
    pub fix_effort: Option<String>,
    pub evidence: Option<String>,
    pub tags: Vec<String>,
    pub zt_pillars: Vec<String>,
    pub cross_refs: Vec<CrossReference>,
    pub grade: Option<GradeVerdict>,
    pub risk_score: Option<UnifiedRiskScore>,
    /// Whether the finding's source file is transitively reachable from an entry point.
    /// `None` = not yet analyzed, `Some(true)` = reachable, `Some(false)` = unreachable (dead code).
    #[serde(default)]
    pub reachable: Option<bool>,
}

/// Location of a finding within the target codebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingLocation {
    /// File path where the finding was detected
    pub file: PathBuf,
    /// Line number within the file
    pub line: Option<u32>,
    /// Column within the line
    pub column: Option<u32>,
    /// Git commit hash (for git-sourced findings)
    pub commit: Option<String>,
    /// Author of the commit
    pub author: Option<String>,
    /// Code snippet around the finding
    pub snippet: Option<String>,
}

/// Links a finding to a corresponding finding from another scanner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// The scanner that produced the related finding
    pub scanner: ScannerType,
    /// Rule ID of the related finding
    pub rule_id: String,
}

/** Reason a finding was rejected by the adversarial AI grader. */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RejectReason {
    TestCode,
    Unreachable,
    FalsePositive,
    AlreadyMitigated,
    SeverityInflated,
}

/** Verdict from the adversarial AI verification grader. */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradeVerdict {
    Confirmed {
        confidence: f32,
        reasoning: String,
    },
    Rejected {
        reasoning: String,
        reason_category: RejectReason,
    },
    NeedsReview {
        reasoning: String,
        open_questions: Vec<String>,
    },
}

/// Per-dimension risk scores for a single finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDimensions {
    pub severity: f32,
    pub confidence: f32,
    pub context: f32,
    pub chain: f32,
    pub zt_pillars: f32,
    pub grade: f32,
}

/// Unified risk score for a single finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRiskScore {
    /// Overall risk score (0.0 = no risk, 1.0 = maximum risk)
    pub overall: f32,
    /// Per-dimension breakdown
    pub dimensions: RiskDimensions,
    /// Confidence in this score (0.0-1.0)
    pub score_confidence: f32,
}

/// Identifies which scanner produced a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScannerType {
    Gitleaks,
    Trufflehog,
    Semgrep,
    TrivyVuln,
    TrivySecret,
    TrivyMisconfig,
    TrivyContainer,
    Checkov,
    Syft,
    Nuclei,
    Zap,
    Architecture,
    ContextDrift,
    McpSecurity,    // Layer 9: MCP security scanning
    TerraformIaC,   // Layer 10: Terraform IaC scanning
    AwsS3,          // Layer 11: AWS S3 bucket permission scanning
    TlsCertificate, // Layer 12: TLS certificate auditing
    Custom(String),
}

impl std::fmt::Display for ScannerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerType::Gitleaks => write!(f, "Gitleaks"),
            ScannerType::Trufflehog => write!(f, "TruffleHog"),
            ScannerType::Semgrep => write!(f, "Semgrep"),
            ScannerType::TrivyVuln => write!(f, "Trivy"),
            ScannerType::TrivySecret => write!(f, "Trivy"),
            ScannerType::TrivyMisconfig => write!(f, "Trivy"),
            ScannerType::TrivyContainer => write!(f, "TrivyContainer"),
            ScannerType::Checkov => write!(f, "Checkov"),
            ScannerType::Syft => write!(f, "Syft"),
            ScannerType::Nuclei => write!(f, "Nuclei"),
            ScannerType::Zap => write!(f, "ZAP"),
            ScannerType::Architecture => write!(f, "Architecture"),
            ScannerType::ContextDrift => write!(f, "ContextDrift"),
            ScannerType::McpSecurity => write!(f, "McpSecurity"),
            ScannerType::TerraformIaC => write!(f, "TerraformIaC"),
            ScannerType::AwsS3 => write!(f, "AwsS3"),
            ScannerType::TlsCertificate => write!(f, "TlsCertificate"),
            ScannerType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Security severity level for a finding.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Severity {
    /// Informational — no direct security impact
    Info,
    /// Low severity — minor issue
    Low,
    /// Medium severity — moderate risk
    Medium,
    /// High severity — significant risk
    High,
    /// Critical severity — immediate action required
    Critical,
}

/// Confidence level in the accuracy of a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    /// Low confidence — possible false positive
    Tentative,
    /// Moderate confidence — likely correct
    Firm,
    /// High confidence — verified or unambiguous
    Certain,
}

/// Overall posture classification for the entire scorecard (UZTF v2.0 §4.3).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostureClassification {
    /// 0–240: significant gaps across all pillars
    Initial,
    /// 241–600: foundational controls, manual processes
    MostlyBaseline,
    /// 601–960: proactive posture, partial automation
    MostlyAdvanced,
    /// 961–1200: strong posture, real-time capabilities
    MostlyAdaptive,
}

/// Zero Trust maturity scorecard across all 12 UZTF pillars.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTrustScorecard {
    /// Total maturity score
    pub overall_score: u32,
    /// Maximum possible score
    pub max_score: u32,
    /// Per-pillar scores
    pub pillars: Vec<PillarScore>,
    /// Pillars at Advanced or Adaptive maturity
    pub pillars_at_advanced_or_higher: u32,
    /// Target maturity level
    pub target_maturity: MaturityTier,
    /// Overall posture classification
    pub classification: PostureClassification,
    /// Gaps between current and target maturity
    pub gap_analysis: Vec<GapAnalysis>,
}

/// Identifies a maturity gap for a single ZT pillar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    /// ZT pillar name
    pub pillar: String,
    /// Current measured maturity
    pub current_maturity: MaturityTier,
    /// Target maturity level
    pub target_maturity: MaturityTier,
    /// Severity of the gap
    pub gap: GapLevel,
    /// Number of findings blocking progress
    pub blocking_findings: u32,
    /// Total severity-weighted deduction for this pillar (UZTF v2.0 §4.1)
    pub total_deduction: u32,
    /// Breakdown of blocking findings by severity
    pub findings_by_severity: FindingsBySeverity,
    /// Remediation recommendations
    pub recommendations: Vec<String>,
}

/// Severity of a maturity gap.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GapLevel {
    /// No gap — target met
    None,
    /// Minor gap, easy to close
    Small,
    /// Moderate gap
    Medium,
    /// Significant gap requiring substantial effort
    Large,
}

/// Score and maturity level for a single ZT pillar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PillarScore {
    /// ZT pillar name
    pub name: String,
    /// Current maturity tier
    pub maturity: MaturityTier,
    /// Number of open gaps for this pillar
    pub gap_count: u32,
    /// Numeric score (0-100)
    pub score: u32,
}

/// Maturity level in the Unified Zero Trust Framework.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaturityTier {
    /// Initial stage — minimal controls in place
    Baseline,
    /// Intermediate stage — proactive monitoring and enforcement
    Advanced,
    /// Highest stage — real-time adaptation and automation
    Adaptive,
}

/// A chain of findings that together form an attack path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackChain {
    /// Unique chain identifier
    pub id: String,
    /// Aggregated risk score for this chain
    pub risk_score: f32,
    /// Human-readable description of the attack path
    pub description: String,
    /// Ordered steps in the attack chain
    pub steps: Vec<String>,
    /// Finding IDs that comprise this chain
    pub finding_ids: Vec<String>,
    /// Remediation recommendation
    pub recommendation: String,
}

/// Summary of a complete scan run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    /// Unique scan identifier
    pub scan_id: String,
    /// ISO-8601 timestamp of scan start
    pub timestamp: String,
    /// Target that was scanned
    pub target: String,
    /// Hash of the target path/configuration
    pub target_hash: String,
    /// Total scan duration in seconds
    pub duration_seconds: f64,
    /// Total number of findings
    pub total_findings: u32,
    /// Breakdown of findings by severity
    pub findings_by_severity: FindingsBySeverity,
    /// Names of scanners used
    pub scanners_used: Vec<String>,
    /// Zero Trust scorecard (if enabled)
    pub zt_scorecard: Option<ZeroTrustScorecard>,
    /// Attack chains identified
    pub attack_chains: Vec<AttackChain>,
}

/// Count of findings grouped by severity level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingsBySeverity {
    /// Number of critical-severity findings
    pub critical: u32,
    /// Number of high-severity findings
    pub high: u32,
    /// Number of medium-severity findings
    pub medium: u32,
    /// Number of low-severity findings
    pub low: u32,
    /// Number of informational findings
    pub info: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Info);
    }

    #[test]
    fn test_canonical_finding_defaults() {
        let f = CanonicalFinding {
            id: "test-1".into(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "test-rule".into(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: "Test".into(),
            description: "Desc".into(),
            location: FindingLocation {
                file: PathBuf::from("test.py"),
                line: Some(42),
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
        };
        assert_eq!(f.id, "test-1");
        assert!(f.location.file.ends_with("test.py"));
    }

    #[test]
    fn test_attack_chain_creation() {
        let chain = AttackChain {
            id: "AC-001".into(),
            risk_score: 8.5,
            description: "Chain from secret to RCE".into(),
            steps: vec!["Find API key".into(), "Access internal endpoint".into()],
            finding_ids: vec!["F-001".into(), "F-002".into()],
            recommendation: "Rotate keys and add WAF".into(),
        };
        assert_eq!(chain.risk_score, 8.5);
        assert_eq!(chain.steps.len(), 2);
    }

    #[test]
    fn test_zt_scorecard_defaults() {
        let sc = ZeroTrustScorecard {
            overall_score: 75,
            max_score: 1200, // 12 pillars × 100
            pillars: vec![],
            pillars_at_advanced_or_higher: 0,
            target_maturity: MaturityTier::Advanced,
            classification: PostureClassification::MostlyBaseline,
            gap_analysis: vec![],
        };
        assert_eq!(sc.overall_score, 75);
        assert_eq!(sc.max_score, 1200);
    }

    #[test]
    fn test_gap_analysis_struct() {
        let ga = GapAnalysis {
            pillar: "identity".into(),
            current_maturity: MaturityTier::Baseline,
            target_maturity: MaturityTier::Advanced,
            gap: GapLevel::Medium,
            blocking_findings: 4,
            total_deduction: 75,
            findings_by_severity: FindingsBySeverity {
                critical: 3,
                high: 1,
                medium: 0,
                low: 0,
                info: 0,
            },
            recommendations: vec!["Rotate secrets".into()],
        };
        assert_eq!(ga.pillar, "identity");
        assert_eq!(ga.gap, GapLevel::Medium);
        assert_eq!(ga.blocking_findings, 4);
        assert_eq!(ga.total_deduction, 75);
    }

    #[test]
    fn test_findings_by_severity() {
        let fbs = FindingsBySeverity {
            critical: 3,
            high: 7,
            medium: 12,
            low: 25,
            info: 50,
        };
        assert_eq!(
            fbs.critical + fbs.high + fbs.medium + fbs.low + fbs.info,
            97
        );
    }

    #[test]
    fn test_maturity_tier_debug() {
        let baseline = MaturityTier::Baseline;
        let advanced = MaturityTier::Advanced;
        let adaptive = MaturityTier::Adaptive;

        // Just verify they exist and can be compared
        assert_ne!(baseline, advanced);
        assert_ne!(advanced, adaptive);
    }

    #[test]
    fn test_cross_reference_default() {
        let cr = CrossReference {
            scanner: ScannerType::Semgrep,
            rule_id: "rule-1".into(),
        };
        assert_eq!(cr.scanner, ScannerType::Semgrep);
    }
}

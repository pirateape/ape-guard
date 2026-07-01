// Canonical Finding Schema
// All scanner output is normalized into this structure for analysis.
//
// Zero Trust types in this module (ZeroTrustScorecard, PillarScore, MaturityTier,
// GapAnalysis) implement the Unified Zero Trust Framework (UZTF) — an 8-pillar
// maturity model that builds on the CISA Zero Trust Maturity Model.
// See: https://github.com/pirateape/unified-zero-trust-framework
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingLocation {
    pub file: PathBuf,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub commit: Option<String>,
    pub author: Option<String>,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub scanner: ScannerType,
    pub rule_id: String,
}

/// Why a finding was rejected by the adversarial grader
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RejectReason {
    TestCode,
    Unreachable,
    FalsePositive,
    AlreadyMitigated,
    SeverityInflated,
}

/// Verdict from the adversarial verification grader
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
            ScannerType::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    Tentative,
    Firm,
    Certain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTrustScorecard {
    pub overall_score: u32,
    pub max_score: u32,
    pub pillars: Vec<PillarScore>,
    pub pillars_at_advanced_or_higher: u32,
    pub target_maturity: MaturityTier,
    pub gap_analysis: Vec<GapAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub pillar: String,
    pub current_maturity: MaturityTier,
    pub target_maturity: MaturityTier,
    pub gap: GapLevel,
    pub blocking_findings: u32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GapLevel {
    None,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PillarScore {
    pub name: String,
    pub maturity: MaturityTier,
    pub gap_count: u32,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaturityTier {
    Baseline,
    Advanced,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackChain {
    pub id: String,
    pub risk_score: f32,
    pub description: String,
    pub steps: Vec<String>,
    pub finding_ids: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub scan_id: String,
    pub timestamp: String,
    pub target: String,
    pub target_hash: String,
    pub duration_seconds: f64,
    pub total_findings: u32,
    pub findings_by_severity: FindingsBySeverity,
    pub scanners_used: Vec<String>,
    pub zt_scorecard: Option<ZeroTrustScorecard>,
    pub attack_chains: Vec<AttackChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingsBySeverity {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
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
            max_score: 800,
            pillars: vec![],
            pillars_at_advanced_or_higher: 0,
            target_maturity: MaturityTier::Advanced,
            gap_analysis: vec![],
        };
        assert_eq!(sc.overall_score, 75);
    }

    #[test]
    fn test_gap_analysis_struct() {
        let ga = GapAnalysis {
            pillar: "identity".into(),
            current_maturity: MaturityTier::Baseline,
            target_maturity: MaturityTier::Advanced,
            gap: GapLevel::Medium,
            blocking_findings: 4,
            recommendations: vec!["Rotate secrets".into()],
        };
        assert_eq!(ga.pillar, "identity");
        assert_eq!(ga.gap, GapLevel::Medium);
        assert_eq!(ga.blocking_findings, 4);
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

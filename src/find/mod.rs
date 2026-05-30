// Canonical Finding Schema
// All scanner output is normalized into this structure for analysis.
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScannerType {
    Gitleaks,
    Semgrep,
    TrivyVuln,
    TrivySecret,
    TrivyMisconfig,
    Nuclei,
    Zap,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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

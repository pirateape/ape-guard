// ApeGuard Multi-Dimension Scoring Pipeline
//
// Computes a unified risk score per finding by combining 6 dimensions:
// severity, confidence, context, attack chain position, ZT pillar impact, and AI grade.
//
// Architecture: docs/scoring-pipeline.md
// Pattern: defending-code-reference-harness — judge agent considers all context holistically
use crate::find::{
    AttackChain, CanonicalFinding, GradeVerdict, RiskDimensions, Severity, UnifiedRiskScore,
};

// ─── Configuration ────────────────────────────────────────────────────────────

/// Weights for each scoring dimension.
/// Defaults calibrated for security audit tooling.
#[derive(Debug, Clone)]
pub struct ScoreWeights {
    pub severity: f32,
    pub confidence: f32,
    pub context: f32,
    pub chain: f32,
    pub zt_pillars: f32,
    pub grade: f32,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            severity: 0.25,
            confidence: 0.20,
            context: 0.15,
            chain: 0.15,
            zt_pillars: 0.10,
            grade: 0.15,
        }
    }
}

impl ScoreWeights {
    /// Sum of all weights for normalization
    fn total(&self) -> f32 {
        self.severity + self.confidence + self.context + self.chain + self.zt_pillars + self.grade
    }
}

// ─── Context for Scoring ──────────────────────────────────────────────────────

/// Context needed to compute finding risk scores.
#[derive(Debug, Clone)]
pub struct ScoringContext<'a> {
    pub chains: &'a [AttackChain],
}

// ─── Orchestrator ─────────────────────────────────────────────────────────────

/// Compute risk scores for all findings in a batch.
///
/// This is deterministic math only — no LLM calls, no external dependencies.
/// Runs efficiently across thousands of findings.
pub fn score_all_findings(
    findings: &mut [CanonicalFinding],
    chains: &[AttackChain],
    weights: &ScoreWeights,
) {
    let context = ScoringContext { chains };

    for finding in findings.iter_mut() {
        let score = compute_finding_risk(finding, &context, weights);
        finding.risk_score = Some(score);
    }
}

// ─── Per-Finding Scoring ──────────────────────────────────────────────────────

/// Compute the unified risk score for a single finding.
fn compute_finding_risk(
    finding: &CanonicalFinding,
    context: &ScoringContext,
    weights: &ScoreWeights,
) -> UnifiedRiskScore {
    let sev = dimension_severity(&finding.severity);
    let conf = dimension_confidence(finding, context);
    let ctx = dimension_context(finding);
    let chain = dimension_chain(finding, context);
    let zt = dimension_zt_pillars(finding);
    let grade = dimension_grade(&finding.grade);

    let weighted_sum = sev * weights.severity
        + conf * weights.confidence
        + ctx * weights.context
        + chain * weights.chain
        + zt * weights.zt_pillars
        + grade * weights.grade;

    let total_weight = weights.total();
    let overall = if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    };

    UnifiedRiskScore {
        overall: (overall * 100.0).round() / 100.0, // Round to 2 decimal places
        dimensions: RiskDimensions {
            severity: sev,
            confidence: conf,
            context: ctx,
            chain,
            zt_pillars: zt,
            grade,
        },
        score_confidence: compute_score_confidence(finding),
    }
}

// ─── Dimension: Severity (0.0–1.0) ────────────────────────────────────────────

fn dimension_severity(severity: &Severity) -> f32 {
    match severity {
        Severity::Critical => 1.0,
        Severity::High => 0.75,
        Severity::Medium => 0.5,
        Severity::Low => 0.25,
        Severity::Info => 0.1,
    }
}

// ─── Dimension: Confidence (0.0–1.0) ──────────────────────────────────────────

fn dimension_confidence(finding: &CanonicalFinding, _context: &ScoringContext) -> f32 {
    // Base score from cross-references (how many scanners agree)
    let cross_ref_score: f32 = match finding.cross_refs.len() {
        0 => 0.4,
        1 => 0.6,
        2 => 0.8,
        _ => 0.95,
    };

    // Adjust for AI grade verdict
    match &finding.grade {
        Some(GradeVerdict::Confirmed { .. }) => (cross_ref_score + 0.3).min(1.0),
        Some(GradeVerdict::Rejected { .. }) => 0.05,
        _ => cross_ref_score,
    }
}

// ─── Dimension: Context (0.0–1.0) ─────────────────────────────────────────────

fn dimension_context(finding: &CanonicalFinding) -> f32 {
    let path = finding.location.file.to_string_lossy().to_lowercase();

    // Test/documentation code → very low risk
    if path.contains("test")
        || path.contains("spec")
        || path.contains("mock")
        || path.contains("fixture")
        || path.contains("example")
        || path.contains("__tests__")
        || path.contains("documentation")
    {
        return 0.1;
    }

    // Security-critical crypto/TLS → very high risk
    if path.contains("crypto")
        || path.contains("tls")
        || path.contains("ssl")
        || path.contains("certificate")
        || path.contains("encrypt")
        || path.contains("decrypt")
        || path.contains("cipher")
    {
        return 0.95;
    }

    // Authentication/authorization → high risk
    if path.contains("auth")
        || path.contains("login")
        || path.contains("password")
        || path.contains("token")
        || path.contains("oauth")
        || path.contains("session")
        || path.contains("permission")
        || path.contains("role")
    {
        return 0.8;
    }

    // Production/infrastructure → medium-high risk
    if path.contains("prod")
        || path.contains("production")
        || path.contains("deploy")
        || path.contains("infrastructure")
        || path.contains("config")
        || path.contains("secret")
    {
        return 0.7;
    }

    // API handlers/controllers → medium risk
    if path.contains("api")
        || path.contains("handler")
        || path.contains("controller")
        || path.contains("route")
        || path.contains("endpoint")
        || path.contains("middleware")
        || path.contains("gateway")
    {
        return 0.6;
    }

    // Utility/library code → low-medium risk
    if path.contains("util")
        || path.contains("helper")
        || path.contains("lib")
        || path.contains("shared")
        || path.contains("common")
    {
        return 0.3;
    }

    // Default / unknown context
    0.5
}

// ─── Dimension: Chain Position (0.0–1.0) ──────────────────────────────────────

fn dimension_chain(finding: &CanonicalFinding, context: &ScoringContext) -> f32 {
    let chain_count = context
        .chains
        .iter()
        .filter(|c| c.finding_ids.contains(&finding.id))
        .count();

    match chain_count {
        0 => 0.3,
        1 => 0.7,
        _ => 0.9, // 2+ chains
    }
}

// ─── Dimension: ZT Pillar Impact (0.0–1.0) ────────────────────────────────────

fn dimension_zt_pillars(finding: &CanonicalFinding) -> f32 {
    match finding.zt_pillars.len() {
        0 => 0.1,
        1 => 0.4,
        2 => 0.6,
        _ => 0.85, // 3+
    }
}

// ─── Dimension: AI Grade (0.0–1.0) ────────────────────────────────────────────

fn dimension_grade(grade: &Option<GradeVerdict>) -> f32 {
    match grade {
        Some(GradeVerdict::Confirmed { confidence, .. }) if *confidence >= 0.8 => 0.9,
        Some(GradeVerdict::Confirmed { .. }) => 0.7,
        Some(GradeVerdict::NeedsReview { .. }) => 0.5,
        Some(GradeVerdict::Rejected { .. }) => 0.1,
        None => 0.5, // Not graded = neutral
    }
}

// ─── Score Confidence (how reliable is this overall score) ───────────────────

fn compute_score_confidence(finding: &CanonicalFinding) -> f32 {
    // More data dimensions = higher confidence in the score
    let mut factors: Vec<f32> = Vec::new();

    // Severity is always known
    factors.push(0.9);

    // Cross-references boost confidence
    factors.push(0.5 + (finding.cross_refs.len() as f32).min(3.0) * 0.15);

    // AI grade boosts if present
    if finding.grade.is_some() {
        factors.push(0.85);
    }

    // ZT pillars known
    if !finding.zt_pillars.is_empty() {
        factors.push(0.8);
    }

    // Context is always computable
    factors.push(0.7);

    // Average all factors
    let sum: f32 = factors.iter().sum();
    (sum / factors.len() as f32).min(1.0)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::*;
    use std::path::PathBuf;

    fn make_finding(
        id: &str,
        severity: Severity,
        file: &str,
        cross_refs: usize,
        zt_count: usize,
        grade: Option<GradeVerdict>,
    ) -> CanonicalFinding {
        let mut cross_ref_list = Vec::new();
        for i in 0..cross_refs {
            cross_ref_list.push(CrossReference {
                scanner: ScannerType::Semgrep,
                rule_id: format!("rule-{}", i),
            });
        }

        let mut zt_pillars = Vec::new();
        for i in 0..zt_count {
            zt_pillars.push(format!("pillar-{}", i));
        }

        CanonicalFinding {
            id: id.into(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "test-rule".into(),
            severity,
            confidence: Confidence::Firm,
            title: format!("Test {}", id),
            description: format!("Description for {}", id),
            location: FindingLocation {
                file: PathBuf::from(file),
                line: Some(10),
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
            zt_pillars,
            cross_refs: cross_ref_list,
            grade,
            risk_score: None,
            reachable: None,
        }
    }

    fn make_chain(finding_ids: Vec<String>) -> AttackChain {
        AttackChain {
            id: "AC-001".into(),
            risk_score: 7.5,
            description: "Test chain".into(),
            steps: vec!["Step 1".into()],
            finding_ids,
            recommendation: "Fix it".into(),
        }
    }

    // ─── Severity Dimension Tests ────────────────────────────────────────

    #[test]
    fn test_severity_critical() {
        assert_eq!(dimension_severity(&Severity::Critical), 1.0);
    }

    #[test]
    fn test_severity_high() {
        assert!((dimension_severity(&Severity::High) - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_severity_medium() {
        assert!((dimension_severity(&Severity::Medium) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_severity_low() {
        assert!((dimension_severity(&Severity::Low) - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_severity_info() {
        assert!((dimension_severity(&Severity::Info) - 0.1).abs() < 0.01);
    }

    // ─── Confidence Dimension Tests ──────────────────────────────────────

    #[test]
    fn test_confidence_no_cross_refs() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let ctx = ScoringContext { chains: &[] };
        let score = dimension_confidence(&finding, &ctx);
        assert!((score - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_confidence_with_cross_refs() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 2, 0, None);
        let ctx = ScoringContext { chains: &[] };
        let score = dimension_confidence(&finding, &ctx);
        assert!((score - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_confidence_rejected_by_ai() {
        let finding = make_finding(
            "F-1",
            Severity::High,
            "src/main.rs",
            2,
            0,
            Some(GradeVerdict::Rejected {
                reasoning: "fp".into(),
                reason_category: RejectReason::TestCode,
            }),
        );
        let ctx = ScoringContext { chains: &[] };
        let score = dimension_confidence(&finding, &ctx);
        assert!((score - 0.05).abs() < 0.01);
    }

    #[test]
    fn test_confidence_confirmed_by_ai() {
        let finding = make_finding(
            "F-1",
            Severity::High,
            "src/main.rs",
            0,
            0,
            Some(GradeVerdict::Confirmed {
                confidence: 0.95,
                reasoning: "real".into(),
            }),
        );
        let ctx = ScoringContext { chains: &[] };
        let score = dimension_confidence(&finding, &ctx);
        // Base 0.4 + boost 0.3 = 0.7
        assert!((score - 0.7).abs() < 0.01);
    }

    // ─── Context Dimension Tests ──────────────────────────────────────────

    #[test]
    fn test_context_test_dir() {
        let finding = make_finding("F-1", Severity::High, "src/test/main.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_context_crypto_dir() {
        let finding = make_finding("F-1", Severity::High, "src/crypto/aes.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_context_auth_dir() {
        let finding = make_finding("F-1", Severity::High, "src/auth/login.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_context_api_dir() {
        let finding = make_finding("F-1", Severity::High, "src/api/handler.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_context_util_dir() {
        let finding = make_finding("F-1", Severity::High, "src/util/helpers.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_context_config_dir() {
        let finding = make_finding("F-1", Severity::High, "config/secrets.yaml", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_context_default() {
        let finding = make_finding("F-1", Severity::High, "src/unknown_module.rs", 0, 0, None);
        let score = dimension_context(&finding);
        assert!((score - 0.5).abs() < 0.01);
    }

    // ─── Chain Dimension Tests ────────────────────────────────────────────

    #[test]
    fn test_chain_not_in_any() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let ctx = ScoringContext { chains: &[] };
        let score = dimension_chain(&finding, &ctx);
        assert!((score - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_chain_in_one() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let chains = vec![make_chain(vec!["F-1".into(), "F-2".into()])];
        let ctx = ScoringContext { chains: &chains };
        let score = dimension_chain(&finding, &ctx);
        assert!((score - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_chain_in_two() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let chains = vec![
            make_chain(vec!["F-1".into(), "F-2".into()]),
            make_chain(vec!["F-1".into(), "F-3".into()]),
        ];
        let ctx = ScoringContext { chains: &chains };
        let score = dimension_chain(&finding, &ctx);
        assert!((score - 0.9).abs() < 0.01);
    }

    // ─── ZT Pillar Dimension Tests ────────────────────────────────────────

    #[test]
    fn test_zt_zero_pillars() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let score = dimension_zt_pillars(&finding);
        assert!((score - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_zt_one_pillar() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 1, None);
        let score = dimension_zt_pillars(&finding);
        assert!((score - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_zt_two_pillars() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 2, None);
        let score = dimension_zt_pillars(&finding);
        assert!((score - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_zt_three_pillars() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 3, None);
        let score = dimension_zt_pillars(&finding);
        assert!((score - 0.85).abs() < 0.01);
    }

    // ─── Grade Dimension Tests ────────────────────────────────────────────

    #[test]
    fn test_grade_not_graded() {
        let score = dimension_grade(&None);
        assert!((score - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_grade_confirmed_high_confidence() {
        let grade = Some(GradeVerdict::Confirmed {
            confidence: 0.9,
            reasoning: "real".into(),
        });
        let score = dimension_grade(&grade);
        assert!((score - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_grade_confirmed_low_confidence() {
        let grade = Some(GradeVerdict::Confirmed {
            confidence: 0.6,
            reasoning: "maybe".into(),
        });
        let score = dimension_grade(&grade);
        assert!((score - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_grade_needs_review() {
        let grade = Some(GradeVerdict::NeedsReview {
            reasoning: "unclear".into(),
            open_questions: vec![],
        });
        let score = dimension_grade(&grade);
        assert!((score - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_grade_rejected() {
        let grade = Some(GradeVerdict::Rejected {
            reasoning: "fp".into(),
            reason_category: RejectReason::TestCode,
        });
        let score = dimension_grade(&grade);
        assert!((score - 0.1).abs() < 0.01);
    }

    // ─── Integrated Score Tests ───────────────────────────────────────────

    #[test]
    fn test_critical_finding_scores_high() {
        let finding = make_finding("F-1", Severity::Critical, "src/auth/login.rs", 1, 1, None);
        let ctx = ScoringContext { chains: &[] };
        let weights = ScoreWeights::default();
        let score = compute_finding_risk(&finding, &ctx, &weights);
        assert!(
            score.overall > 0.5,
            "Critical finding should score >0.5, got {}",
            score.overall
        );
        assert!((score.dimensions.severity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rejected_low_finding_scores_low() {
        let finding = make_finding(
            "F-1",
            Severity::Low,
            "src/test/mock_test.rs",
            0,
            0,
            Some(GradeVerdict::Rejected {
                reasoning: "test code".into(),
                reason_category: RejectReason::TestCode,
            }),
        );
        let ctx = ScoringContext { chains: &[] };
        let weights = ScoreWeights::default();
        let score = compute_finding_risk(&finding, &ctx, &weights);
        assert!(
            score.overall < 0.3,
            "Rejected Low finding should score <0.3, got {}",
            score.overall
        );
    }

    #[test]
    fn test_chain_finding_boosted() {
        let finding = make_finding("F-1", Severity::High, "src/main.rs", 0, 0, None);
        let chains = vec![make_chain(vec!["F-1".into()])];
        let ctx = ScoringContext { chains: &chains };
        let weights = ScoreWeights::default();
        let score_no_chain =
            compute_finding_risk(&finding, &ScoringContext { chains: &[] }, &weights);
        let score_with_chain = compute_finding_risk(&finding, &ctx, &weights);
        assert!(
            score_with_chain.overall > score_no_chain.overall,
            "Finding in chain should score higher than not in chain"
        );
    }

    #[test]
    fn test_score_all_findings_batch() {
        let mut findings = vec![
            make_finding("F-1", Severity::Critical, "src/auth/login.rs", 2, 2, None),
            make_finding(
                "F-2",
                Severity::Low,
                "src/test/mock_test.rs",
                0,
                0,
                Some(GradeVerdict::Rejected {
                    reasoning: "test".into(),
                    reason_category: RejectReason::TestCode,
                }),
            ),
            make_finding("F-3", Severity::Medium, "src/util/helper.rs", 1, 1, None),
        ];
        let chains = vec![make_chain(vec!["F-1".into()])];
        let weights = ScoreWeights::default();

        score_all_findings(&mut findings, &chains, &weights);

        assert!(findings[0].risk_score.is_some());
        assert!(findings[1].risk_score.is_some());
        assert!(findings[2].risk_score.is_some());

        // Critical in auth with cross-refs should score highest
        assert!(
            findings[0]
                .risk_score
                .as_ref()
                .expect("score 0 should be scored")
                .overall
                > findings[1]
                    .risk_score
                    .as_ref()
                    .expect("score 1 should be scored")
                    .overall
        );
        // Rejected low in test should score lowest
        assert!(
            findings[1]
                .risk_score
                .as_ref()
                .expect("score 1 should be scored")
                .overall
                < findings[2]
                    .risk_score
                    .as_ref()
                    .expect("score 2 should be scored")
                    .overall
        );
    }

    // ─── Score Weight Tests ───────────────────────────────────────────────

    #[test]
    fn test_score_weights_default_total() {
        let w = ScoreWeights::default();
        // Total should be 1.0
        assert!((w.total() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_score_weights_custom() {
        let w = ScoreWeights {
            severity: 0.5,
            confidence: 0.5,
            ..ScoreWeights::default()
        };
        assert!((w.severity - 0.5).abs() < 0.01);
        assert!((w.confidence - 0.5).abs() < 0.01);
    }
}

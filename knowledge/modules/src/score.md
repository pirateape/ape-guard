---
type: Rust Module
title: score
resource: src/score.rs#L1-L923
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-attackchain-canonicalfinding-gradeverdict-riskdimensions-severity-unifiedriskscore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ScoreWeights](../../classes/src/score/ScoreWeights.md)
- [default](../../functions/src/score/ScoreWeights/default/default.md)
- [total](../../functions/src/score/ScoreWeights/total.md)
- [ScoringContext](../../classes/src/score/ScoringContext.md)
- [score_all_findings](../../functions/src/score/score_all_findings.md)
- [compute_finding_risk](../../functions/src/score/compute_finding_risk.md)
- [dimension_severity](../../functions/src/score/dimension_severity.md)
- [dimension_confidence](../../functions/src/score/dimension_confidence.md)
- [dimension_context](../../functions/src/score/dimension_context.md)
- [dimension_chain](../../functions/src/score/dimension_chain.md)
- [dimension_zt_pillars](../../functions/src/score/dimension_zt_pillars.md)
- [dimension_grade](../../functions/src/score/dimension_grade.md)
- [compute_score_confidence](../../functions/src/score/compute_score_confidence.md)
- [ScanHealthScore](../../classes/src/score/ScanHealthScore.md)
- [ScanHealthDimensions](../../classes/src/score/ScanHealthDimensions.md)
- [ScoreTrend](../../classes/src/score/ScoreTrend.md)
- [compute_scan_health](../../functions/src/score/compute_scan_health.md)
- [make_finding](../../functions/src/score/make_finding.md)
- [make_chain](../../functions/src/score/make_chain.md)
- [test_severity_critical](../../functions/src/score/test_severity_critical.md)
- [test_severity_high](../../functions/src/score/test_severity_high.md)
- [test_severity_medium](../../functions/src/score/test_severity_medium.md)
- [test_severity_low](../../functions/src/score/test_severity_low.md)
- [test_severity_info](../../functions/src/score/test_severity_info.md)
- [test_confidence_no_cross_refs](../../functions/src/score/test_confidence_no_cross_refs.md)
- [test_confidence_with_cross_refs](../../functions/src/score/test_confidence_with_cross_refs.md)
- [test_confidence_rejected_by_ai](../../functions/src/score/test_confidence_rejected_by_ai.md)
- [test_confidence_confirmed_by_ai](../../functions/src/score/test_confidence_confirmed_by_ai.md)
- [test_context_test_dir](../../functions/src/score/test_context_test_dir.md)
- [test_context_crypto_dir](../../functions/src/score/test_context_crypto_dir.md)
- [test_context_auth_dir](../../functions/src/score/test_context_auth_dir.md)
- [test_context_api_dir](../../functions/src/score/test_context_api_dir.md)
- [test_context_util_dir](../../functions/src/score/test_context_util_dir.md)
- [test_context_config_dir](../../functions/src/score/test_context_config_dir.md)
- [test_context_default](../../functions/src/score/test_context_default.md)
- [test_chain_not_in_any](../../functions/src/score/test_chain_not_in_any.md)
- [test_chain_in_one](../../functions/src/score/test_chain_in_one.md)
- [test_chain_in_two](../../functions/src/score/test_chain_in_two.md)
- [test_zt_zero_pillars](../../functions/src/score/test_zt_zero_pillars.md)
- [test_zt_one_pillar](../../functions/src/score/test_zt_one_pillar.md)
- [test_zt_two_pillars](../../functions/src/score/test_zt_two_pillars.md)
- [test_zt_three_pillars](../../functions/src/score/test_zt_three_pillars.md)
- [test_grade_not_graded](../../functions/src/score/test_grade_not_graded.md)
- [test_grade_confirmed_high_confidence](../../functions/src/score/test_grade_confirmed_high_confidence.md)
- [test_grade_confirmed_low_confidence](../../functions/src/score/test_grade_confirmed_low_confidence.md)
- [test_grade_needs_review](../../functions/src/score/test_grade_needs_review.md)
- [test_grade_rejected](../../functions/src/score/test_grade_rejected.md)
- [test_critical_finding_scores_high](../../functions/src/score/test_critical_finding_scores_high.md)
- [test_rejected_low_finding_scores_low](../../functions/src/score/test_rejected_low_finding_scores_low.md)
- [test_chain_finding_boosted](../../functions/src/score/test_chain_finding_boosted.md)
- [test_score_all_findings_batch](../../functions/src/score/test_score_all_findings_batch.md)
- [test_scan_health_no_findings](../../functions/src/score/test_scan_health_no_findings.md)
- [test_scan_health_with_findings](../../functions/src/score/test_scan_health_with_findings.md)
- [test_scan_health_bounds](../../functions/src/score/test_scan_health_bounds.md)
- [test_scanner_risk_per_scanner](../../functions/src/score/test_scanner_risk_per_scanner.md)
- [test_score_weights_default_total](../../functions/src/score/test_score_weights_default_total.md)
- [test_score_weights_custom](../../functions/src/score/test_score_weights_custom.md)

# Imports

- `crate::find::{
    AttackChain, CanonicalFinding, GradeVerdict, RiskDimensions, Severity, UnifiedRiskScore,
}`
- `serde::Serialize`
- `std::collections::HashMap`
- `super::*`
- `crate::find::*`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
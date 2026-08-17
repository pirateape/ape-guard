---
type: Rust Function
title: compute_finding_risk
resource: src/score.rs#L78-L116
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/dimension_severity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/dimension_confidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/dimension_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/dimension_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/dimension_zt_pillars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/dimension_grade
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/ScoreWeights/total
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/compute_score_confidence
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/score/score_all_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_critical_finding_scores_high
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_rejected_low_finding_scores_low
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_chain_finding_boosted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn compute_finding_risk( finding: &CanonicalFinding, context: &ScoringContext, weights: &ScoreWeights, ) -> UnifiedRiskScore`

# Calls

- [dimension_severity](../../../functions/src/score/dimension_severity.md)
- [dimension_confidence](../../../functions/src/score/dimension_confidence.md)
- [dimension_context](../../../functions/src/score/dimension_context.md)
- [dimension_chain](../../../functions/src/score/dimension_chain.md)
- [dimension_zt_pillars](../../../functions/src/score/dimension_zt_pillars.md)
- [dimension_grade](../../../functions/src/score/dimension_grade.md)
- [total](../../../functions/src/score/ScoreWeights/total.md)
- [compute_score_confidence](../../../functions/src/score/compute_score_confidence.md)

# Called by

- [score_all_findings](../../../functions/src/score/score_all_findings.md)
- [test_critical_finding_scores_high](../../../functions/src/score/test_critical_finding_scores_high.md)
- [test_rejected_low_finding_scores_low](../../../functions/src/score/test_rejected_low_finding_scores_low.md)
- [test_chain_finding_boosted](../../../functions/src/score/test_chain_finding_boosted.md)
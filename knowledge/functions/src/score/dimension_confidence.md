---
type: Rust Function
title: dimension_confidence
resource: src/score.rs#L132-L147
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_confidence_no_cross_refs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_confidence_with_cross_refs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_confidence_rejected_by_ai
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_confidence_confirmed_by_ai
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dimension_confidence(finding: &CanonicalFinding, _context: &ScoringContext) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [test_confidence_no_cross_refs](../../../functions/src/score/test_confidence_no_cross_refs.md)
- [test_confidence_with_cross_refs](../../../functions/src/score/test_confidence_with_cross_refs.md)
- [test_confidence_rejected_by_ai](../../../functions/src/score/test_confidence_rejected_by_ai.md)
- [test_confidence_confirmed_by_ai](../../../functions/src/score/test_confidence_confirmed_by_ai.md)
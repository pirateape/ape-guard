---
type: Rust Function
title: dimension_chain
resource: src/score.rs#L230-L242
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_chain_not_in_any
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_chain_in_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_chain_in_two
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dimension_chain(finding: &CanonicalFinding, context: &ScoringContext) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [test_chain_not_in_any](../../../functions/src/score/test_chain_not_in_any.md)
- [test_chain_in_one](../../../functions/src/score/test_chain_in_one.md)
- [test_chain_in_two](../../../functions/src/score/test_chain_in_two.md)
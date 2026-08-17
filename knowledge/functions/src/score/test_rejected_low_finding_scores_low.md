---
type: Rust Function
title: test_rejected_low_finding_scores_low
resource: src/score.rs#L748-L768
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/ScoreWeights/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_rejected_low_finding_scores_low()`

# Calls

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [make_finding](../../../functions/src/score/make_finding.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
---
type: Rust Function
title: test_critical_finding_scores_high
resource: src/score.rs#L734-L745
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

`fn test_critical_finding_scores_high()`

# Calls

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [make_finding](../../../functions/src/score/make_finding.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
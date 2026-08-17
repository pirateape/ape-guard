---
type: Rust Function
title: test_chain_finding_boosted
resource: src/score.rs#L771-L783
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

`fn test_chain_finding_boosted()`

# Calls

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [make_finding](../../../functions/src/score/make_finding.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
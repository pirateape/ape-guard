---
type: Rust Function
title: test_scan_health_bounds
resource: src/score.rs#L864-L879
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/score_all_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/compute_scan_health
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/ScoreWeights/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_scan_health_bounds()`

# Calls

- [score_all_findings](../../../functions/src/score/score_all_findings.md)
- [compute_scan_health](../../../functions/src/score/compute_scan_health.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
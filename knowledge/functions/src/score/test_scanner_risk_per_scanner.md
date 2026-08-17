---
type: Rust Function
title: test_scanner_risk_per_scanner
resource: src/score.rs#L882-L902
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

`fn test_scanner_risk_per_scanner()`

# Calls

- [score_all_findings](../../../functions/src/score/score_all_findings.md)
- [compute_scan_health](../../../functions/src/score/compute_scan_health.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
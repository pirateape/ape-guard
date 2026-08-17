---
type: Rust Function
title: test_gap_analysis_with_secrets
resource: src/normalize.rs#L675-L690
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/normalize/normalize_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_gap_analysis_with_secrets()`

# Calls

- [normalize_findings](../../../functions/src/normalize/normalize_findings.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)
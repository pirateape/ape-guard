---
type: Rust Function
title: test_scorecard_multiple_findings_capped
resource: src/normalize.rs#L579-L597
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

`fn test_scorecard_multiple_findings_capped()`

# Calls

- [normalize_findings](../../../functions/src/normalize/normalize_findings.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)
---
type: Rust Function
title: count_verdicts
resource: src/grade.rs#L222-L240
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/grade/test_count_verdicts_all_types
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn count_verdicts(findings: &[CanonicalFinding]) -> GradeCounts`

# Called by

- [test_count_verdicts_all_types](../../../functions/src/grade/test_count_verdicts_all_types.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
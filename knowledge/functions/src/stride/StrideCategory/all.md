---
type: Rust Method
title: all
resource: src/stride.rs#L91-L100
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/evaluate_rules_on_group
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/analyze_stride_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_all_categories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn all() -> [StrideCategory; 6]`

# Called by

- [evaluate_rules_on_group](../../../../functions/src/chain/evaluate_rules_on_group.md)
- [analyze_stride_coverage](../../../../functions/src/stride/analyze_stride_coverage.md)
- [test_all_categories](../../../../functions/src/stride/test_all_categories.md)
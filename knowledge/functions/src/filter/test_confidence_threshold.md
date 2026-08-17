---
type: Rust Function
title: test_confidence_threshold
resource: src/filter.rs#L472-L490
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/filter/default_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/apply_fp_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_confidence_threshold()`

# Calls

- [default_config](../../../functions/src/filter/default_config.md)
- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
- [make_finding](../../../functions/src/filter/make_finding.md)
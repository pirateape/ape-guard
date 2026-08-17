---
type: Rust Function
title: test_excludes_vendor_paths
resource: src/filter.rs#L331-L353
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
---

# Signature

`fn test_excludes_vendor_paths()`

# Calls

- [default_config](../../../functions/src/filter/default_config.md)
- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
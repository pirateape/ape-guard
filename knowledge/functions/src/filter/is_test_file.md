---
type: Rust Function
title: is_test_file
resource: src/filter.rs#L261-L264
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/filter/apply_fp_filters
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_test_file(path: &Path) -> bool`

# Called by

- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
---
type: Rust Function
title: is_excluded_path
resource: src/filter.rs#L224-L258
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

`fn is_excluded_path(path: &Path, config: &FilterConfig) -> bool`

# Called by

- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
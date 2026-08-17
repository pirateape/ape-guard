---
type: Rust Function
title: partition_by
resource: src/filter.rs#L213-L221
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

`fn partition_by<F>( findings: Vec<CanonicalFinding>, mut pred: F, ) -> (Vec<CanonicalFinding>, Vec<CanonicalFinding>) where F: FnMut(&CanonicalFinding) -> bool,`

# Called by

- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
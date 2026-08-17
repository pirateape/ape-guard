---
type: Rust Function
title: test_merge_empty_layers_does_not_override
resource: src/config.rs#L457-L467
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/merge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_merge_empty_layers_does_not_override()`

# Calls

- [merge](../../../functions/src/config/merge.md)
- [default](../../../functions/src/config/Config/default/default.md)
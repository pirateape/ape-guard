---
type: Rust Function
title: merge
resource: src/config.rs#L332-L378
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_merge_overlay
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_merge_empty_layers_does_not_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_merge_report_formats
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge(base: &mut Config, overlay: Config)`

# Called by

- [load](../../../functions/src/config/load.md)
- [test_merge_overlay](../../../functions/src/config/test_merge_overlay.md)
- [test_merge_empty_layers_does_not_override](../../../functions/src/config/test_merge_empty_layers_does_not_override.md)
- [test_merge_report_formats](../../../functions/src/config/test_merge_report_formats.md)
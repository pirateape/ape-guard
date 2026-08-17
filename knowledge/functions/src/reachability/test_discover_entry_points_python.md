---
type: Rust Function
title: test_discover_entry_points_python
resource: src/reachability.rs#L884-L891
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/create_temp_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/write_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/discover_entry_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_discover_entry_points_python()`

# Calls

- [create_temp_dir](../../../functions/src/reachability/create_temp_dir.md)
- [write_file](../../../functions/src/reachability/write_file.md)
- [discover_entry_points](../../../functions/src/reachability/discover_entry_points.md)
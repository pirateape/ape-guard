---
type: Rust Function
title: discover_entry_points
resource: src/reachability.rs#L91-L152
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_discover_entry_points_rust
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_discover_entry_points_python
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_discover_entry_points_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn discover_entry_points(target: &Path, user_entries: &[String]) -> Vec<PathBuf>`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
- [test_discover_entry_points_rust](../../../functions/src/reachability/test_discover_entry_points_rust.md)
- [test_discover_entry_points_python](../../../functions/src/reachability/test_discover_entry_points_python.md)
- [test_discover_entry_points_none](../../../functions/src/reachability/test_discover_entry_points_none.md)
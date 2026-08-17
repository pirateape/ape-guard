---
type: Rust Function
title: test_canonicalize_path_relative
resource: src/reachability.rs#L1291-L1299
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
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_canonicalize_path_relative()`

# Calls

- [create_temp_dir](../../../functions/src/reachability/create_temp_dir.md)
- [write_file](../../../functions/src/reachability/write_file.md)
- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)
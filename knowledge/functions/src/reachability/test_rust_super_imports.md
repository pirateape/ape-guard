---
type: Rust Function
title: test_rust_super_imports
resource: src/reachability.rs#L1320-L1334
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
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_rust_super_imports()`

# Calls

- [create_temp_dir](../../../functions/src/reachability/create_temp_dir.md)
- [write_file](../../../functions/src/reachability/write_file.md)
- [extract_imports](../../../functions/src/reachability/extract_imports.md)
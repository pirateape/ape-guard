---
type: Rust Function
title: test_collect_source_files_excludes_common_dirs
resource: src/reachability.rs#L918-L935
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
  - target: functions/src/reachability/collect_source_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_collect_source_files_excludes_common_dirs()`

# Calls

- [create_temp_dir](../../../functions/src/reachability/create_temp_dir.md)
- [write_file](../../../functions/src/reachability/write_file.md)
- [collect_source_files](../../../functions/src/reachability/collect_source_files.md)
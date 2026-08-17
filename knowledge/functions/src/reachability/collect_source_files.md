---
type: Rust Function
title: collect_source_files
resource: src/reachability.rs#L158-L169
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/collect_files_recursive
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_collect_source_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_collect_source_files_excludes_common_dirs
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn collect_source_files( target: &Path, extensions: &[String], exclude_dirs: &[String], ) -> Vec<PathBuf>`

# Calls

- [collect_files_recursive](../../../functions/src/reachability/collect_files_recursive.md)

# Called by

- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
- [test_collect_source_files](../../../functions/src/reachability/test_collect_source_files.md)
- [test_collect_source_files_excludes_common_dirs](../../../functions/src/reachability/test_collect_source_files_excludes_common_dirs.md)
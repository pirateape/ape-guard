---
type: Rust Function
title: collect_files_recursive
resource: src/reachability.rs#L172-L206
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/collect_source_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn collect_files_recursive( root: &Path, dir: &Path, files: &mut Vec<PathBuf>, extensions: &[String], exclude_dirs: &HashSet<&str>, )`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [collect_source_files](../../../functions/src/reachability/collect_source_files.md)
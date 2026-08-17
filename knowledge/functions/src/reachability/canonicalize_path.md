---
type: Rust Function
title: canonicalize_path
resource: src/reachability.rs#L834-L846
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/discover_entry_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/collect_files_recursive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_rust_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_relative_use
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_python_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_python_absolute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_js_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_c_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/apply_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_apply_reachability_markings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_canonicalize_path_relative
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn canonicalize_path(path: &Path) -> PathBuf`

# Called by

- [discover_entry_points](../../../functions/src/reachability/discover_entry_points.md)
- [collect_files_recursive](../../../functions/src/reachability/collect_files_recursive.md)
- [extract_rust_imports](../../../functions/src/reachability/extract_rust_imports.md)
- [resolve_relative_use](../../../functions/src/reachability/resolve_relative_use.md)
- [resolve_python_relative](../../../functions/src/reachability/resolve_python_relative.md)
- [resolve_python_absolute](../../../functions/src/reachability/resolve_python_absolute.md)
- [resolve_js_path](../../../functions/src/reachability/resolve_js_path.md)
- [extract_c_imports](../../../functions/src/reachability/extract_c_imports.md)
- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
- [apply_reachability](../../../functions/src/reachability/apply_reachability.md)
- [test_apply_reachability_markings](../../../functions/src/reachability/test_apply_reachability_markings.md)
- [test_canonicalize_path_relative](../../../functions/src/reachability/test_canonicalize_path_relative.md)
---
type: Rust Function
title: write_file
resource: src/reachability.rs#L859-L866
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/test_discover_entry_points_rust
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_discover_entry_points_python
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_collect_source_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_collect_source_files_excludes_common_dirs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_rust_mod_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_rust_crate_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_rust_submodule_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_python_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_python_from_import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_python_relative_import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_js_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_js_require
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_js_index_resolution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_c_include
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_apply_reachability_markings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_canonicalize_path_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_no_false_imports_for_std_extern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_rust_super_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_extract_js_tsx_extension
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_file(dir: &Path, rel: &str, content: &str) -> PathBuf`

# Called by

- [test_discover_entry_points_rust](../../../functions/src/reachability/test_discover_entry_points_rust.md)
- [test_discover_entry_points_python](../../../functions/src/reachability/test_discover_entry_points_python.md)
- [test_collect_source_files](../../../functions/src/reachability/test_collect_source_files.md)
- [test_collect_source_files_excludes_common_dirs](../../../functions/src/reachability/test_collect_source_files_excludes_common_dirs.md)
- [test_extract_rust_mod_imports](../../../functions/src/reachability/test_extract_rust_mod_imports.md)
- [test_extract_rust_crate_imports](../../../functions/src/reachability/test_extract_rust_crate_imports.md)
- [test_extract_rust_submodule_imports](../../../functions/src/reachability/test_extract_rust_submodule_imports.md)
- [test_extract_python_imports](../../../functions/src/reachability/test_extract_python_imports.md)
- [test_extract_python_from_import](../../../functions/src/reachability/test_extract_python_from_import.md)
- [test_extract_python_relative_import](../../../functions/src/reachability/test_extract_python_relative_import.md)
- [test_extract_js_imports](../../../functions/src/reachability/test_extract_js_imports.md)
- [test_extract_js_require](../../../functions/src/reachability/test_extract_js_require.md)
- [test_extract_js_index_resolution](../../../functions/src/reachability/test_extract_js_index_resolution.md)
- [test_extract_c_include](../../../functions/src/reachability/test_extract_c_include.md)
- [test_apply_reachability_markings](../../../functions/src/reachability/test_apply_reachability_markings.md)
- [test_canonicalize_path_relative](../../../functions/src/reachability/test_canonicalize_path_relative.md)
- [test_no_false_imports_for_std_extern](../../../functions/src/reachability/test_no_false_imports_for_std_extern.md)
- [test_rust_super_imports](../../../functions/src/reachability/test_rust_super_imports.md)
- [test_extract_js_tsx_extension](../../../functions/src/reachability/test_extract_js_tsx_extension.md)
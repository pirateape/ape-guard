---
type: Rust Function
title: extract_imports
resource: src/reachability.rs#L212-L234
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/extract_rust_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_python_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_js_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_go_imports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/extract_c_imports
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/build_import_graph
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

`fn extract_imports(file_path: &Path, target_root: &Path) -> Vec<PathBuf>`

# Calls

- [extract_rust_imports](../../../functions/src/reachability/extract_rust_imports.md)
- [extract_python_imports](../../../functions/src/reachability/extract_python_imports.md)
- [extract_js_imports](../../../functions/src/reachability/extract_js_imports.md)
- [extract_go_imports](../../../functions/src/reachability/extract_go_imports.md)
- [extract_c_imports](../../../functions/src/reachability/extract_c_imports.md)

# Called by

- [build_import_graph](../../../functions/src/reachability/build_import_graph.md)
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
- [test_no_false_imports_for_std_extern](../../../functions/src/reachability/test_no_false_imports_for_std_extern.md)
- [test_rust_super_imports](../../../functions/src/reachability/test_rust_super_imports.md)
- [test_extract_js_tsx_extension](../../../functions/src/reachability/test_extract_js_tsx_extension.md)
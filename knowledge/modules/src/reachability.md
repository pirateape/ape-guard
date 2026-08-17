---
type: Rust Module
title: reachability
resource: src/reachability.rs#L1-L1358
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-hashset-vecdeque
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-rc-rc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ReachabilityResult](../../classes/src/reachability/ReachabilityResult.md)
- [ReachabilityConfig](../../classes/src/reachability/ReachabilityConfig.md)
- [default](../../functions/src/reachability/ReachabilityConfig/default/default.md)
- [discover_entry_points](../../functions/src/reachability/discover_entry_points.md)
- [collect_source_files](../../functions/src/reachability/collect_source_files.md)
- [collect_files_recursive](../../functions/src/reachability/collect_files_recursive.md)
- [extract_imports](../../functions/src/reachability/extract_imports.md)
- [extract_rust_imports](../../functions/src/reachability/extract_rust_imports.md)
- [resolve_rust_crate_path](../../functions/src/reachability/resolve_rust_crate_path.md)
- [resolve_relative_use](../../functions/src/reachability/resolve_relative_use.md)
- [extract_python_imports](../../functions/src/reachability/extract_python_imports.md)
- [resolve_python_relative](../../functions/src/reachability/resolve_python_relative.md)
- [resolve_python_absolute](../../functions/src/reachability/resolve_python_absolute.md)
- [extract_js_imports](../../functions/src/reachability/extract_js_imports.md)
- [extract_quoted_string](../../functions/src/reachability/extract_quoted_string.md)
- [resolve_js_path](../../functions/src/reachability/resolve_js_path.md)
- [extract_go_imports](../../functions/src/reachability/extract_go_imports.md)
- [extract_c_imports](../../functions/src/reachability/extract_c_imports.md)
- [build_import_graph](../../functions/src/reachability/build_import_graph.md)
- [bfs_reachable_files](../../functions/src/reachability/bfs_reachable_files.md)
- [analyze_reachability](../../functions/src/reachability/analyze_reachability.md)
- [apply_reachability](../../functions/src/reachability/apply_reachability.md)
- [canonicalize_path](../../functions/src/reachability/canonicalize_path.md)
- [create_temp_dir](../../functions/src/reachability/create_temp_dir.md)
- [write_file](../../functions/src/reachability/write_file.md)
- [test_discover_entry_points_rust](../../functions/src/reachability/test_discover_entry_points_rust.md)
- [test_discover_entry_points_python](../../functions/src/reachability/test_discover_entry_points_python.md)
- [test_discover_entry_points_none](../../functions/src/reachability/test_discover_entry_points_none.md)
- [test_collect_source_files](../../functions/src/reachability/test_collect_source_files.md)
- [test_collect_source_files_excludes_common_dirs](../../functions/src/reachability/test_collect_source_files_excludes_common_dirs.md)
- [test_extract_rust_mod_imports](../../functions/src/reachability/test_extract_rust_mod_imports.md)
- [test_extract_rust_crate_imports](../../functions/src/reachability/test_extract_rust_crate_imports.md)
- [test_extract_rust_submodule_imports](../../functions/src/reachability/test_extract_rust_submodule_imports.md)
- [test_extract_python_imports](../../functions/src/reachability/test_extract_python_imports.md)
- [test_extract_python_from_import](../../functions/src/reachability/test_extract_python_from_import.md)
- [test_extract_python_relative_import](../../functions/src/reachability/test_extract_python_relative_import.md)
- [test_extract_js_imports](../../functions/src/reachability/test_extract_js_imports.md)
- [test_extract_js_require](../../functions/src/reachability/test_extract_js_require.md)
- [test_extract_js_index_resolution](../../functions/src/reachability/test_extract_js_index_resolution.md)
- [test_extract_c_include](../../functions/src/reachability/test_extract_c_include.md)
- [test_bfs_simple](../../functions/src/reachability/test_bfs_simple.md)
- [test_bfs_with_cycle](../../functions/src/reachability/test_bfs_with_cycle.md)
- [test_bfs_unreachable](../../functions/src/reachability/test_bfs_unreachable.md)
- [test_bfs_multiple_entry_points](../../functions/src/reachability/test_bfs_multiple_entry_points.md)
- [test_analyze_reachability_disabled](../../functions/src/reachability/test_analyze_reachability_disabled.md)
- [test_apply_reachability_markings](../../functions/src/reachability/test_apply_reachability_markings.md)
- [create_minimal_finding](../../functions/src/reachability/create_minimal_finding.md)
- [test_reachability_config_defaults](../../functions/src/reachability/test_reachability_config_defaults.md)
- [test_canonicalize_path_relative](../../functions/src/reachability/test_canonicalize_path_relative.md)
- [test_no_false_imports_for_std_extern](../../functions/src/reachability/test_no_false_imports_for_std_extern.md)
- [test_rust_super_imports](../../functions/src/reachability/test_rust_super_imports.md)
- [test_extract_js_tsx_extension](../../functions/src/reachability/test_extract_js_tsx_extension.md)

# Imports

- `crate::find::CanonicalFinding`
- `std::collections::{HashMap, HashSet, VecDeque}`
- `std::path::{Path, PathBuf}`
- `std::rc::Rc`
- `super::*`
- `std::fs`

# Member of

- [apeguard](../../packages/apeguard.md)
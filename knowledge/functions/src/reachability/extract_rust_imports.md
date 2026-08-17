---
type: Rust Function
title: extract_rust_imports
resource: src/reachability.rs#L237-L285
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_rust_crate_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_relative_use
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_rust_imports( content: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>, )`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)
- [resolve_rust_crate_path](../../../functions/src/reachability/resolve_rust_crate_path.md)
- [resolve_relative_use](../../../functions/src/reachability/resolve_relative_use.md)

# Called by

- [extract_imports](../../../functions/src/reachability/extract_imports.md)
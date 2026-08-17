---
type: Rust Function
title: resolve_relative_use
resource: src/reachability.rs#L324-L382
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_rust_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_relative_use( trimmed: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>, )`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [extract_rust_imports](../../../functions/src/reachability/extract_rust_imports.md)
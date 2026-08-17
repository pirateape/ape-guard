---
type: Rust Function
title: resolve_python_relative
resource: src/reachability.rs#L421-L446
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_python_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_python_relative(module_path: &str, parent: &Path, target_root: &Path) -> Vec<PathBuf>`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [extract_python_imports](../../../functions/src/reachability/extract_python_imports.md)
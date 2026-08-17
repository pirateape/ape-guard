---
type: Rust Function
title: extract_python_imports
resource: src/reachability.rs#L385-L419
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/resolve_python_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_python_absolute
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_python_imports( content: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>, )`

# Calls

- [resolve_python_relative](../../../functions/src/reachability/resolve_python_relative.md)
- [resolve_python_absolute](../../../functions/src/reachability/resolve_python_absolute.md)

# Called by

- [extract_imports](../../../functions/src/reachability/extract_imports.md)
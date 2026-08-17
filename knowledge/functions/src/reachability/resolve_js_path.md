---
type: Rust Function
title: resolve_js_path
resource: src/reachability.rs#L543-L577
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_js_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_js_path(path_str: &str, parent: &Path, _target_root: &Path, imports: &mut Vec<PathBuf>)`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [extract_js_imports](../../../functions/src/reachability/extract_js_imports.md)
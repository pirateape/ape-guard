---
type: Rust Function
title: extract_js_imports
resource: src/reachability.rs#L495-L530
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/extract_quoted_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/resolve_js_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_js_imports( content: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>, )`

# Calls

- [extract_quoted_string](../../../functions/src/reachability/extract_quoted_string.md)
- [resolve_js_path](../../../functions/src/reachability/resolve_js_path.md)

# Called by

- [extract_imports](../../../functions/src/reachability/extract_imports.md)
---
type: Rust Function
title: extract_c_imports
resource: src/reachability.rs#L593-L615
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_c_imports(content: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>)`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [extract_imports](../../../functions/src/reachability/extract_imports.md)
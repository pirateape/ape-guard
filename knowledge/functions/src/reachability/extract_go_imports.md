---
type: Rust Function
title: extract_go_imports
resource: src/reachability.rs#L580-L590
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_go_imports( _content: &str, _parent: &Path, _target_root: &Path, _imports: &mut Vec<PathBuf>, )`

# Called by

- [extract_imports](../../../functions/src/reachability/extract_imports.md)
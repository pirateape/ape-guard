---
type: Rust Function
title: resolve_rust_crate_path
resource: src/reachability.rs#L288-L321
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/extract_rust_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_rust_crate_path(parts: &[&str], target_root: &Path) -> Option<PathBuf>`

# Called by

- [extract_rust_imports](../../../functions/src/reachability/extract_rust_imports.md)
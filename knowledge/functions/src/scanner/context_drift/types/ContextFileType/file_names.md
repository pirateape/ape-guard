---
type: Rust Method
title: file_names
resource: src/scanner/context_drift/types.rs#L19-L25
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/discover/discover_context_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn file_names(&self) -> &[&str]`

# Called by

- [discover_context_files](../../../../../../functions/src/scanner/context_drift/discover/discover_context_files.md)
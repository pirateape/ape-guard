---
type: Rust Function
title: detect_file_type
resource: src/scanner/context_drift/discover.rs#L63-L76
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/parse/parse_all_context_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_file_type(path: &Path) -> Option<ContextFileType>`

# Called by

- [parse_all_context_files](../../../../../functions/src/scanner/context_drift/parse/parse_all_context_files.md)
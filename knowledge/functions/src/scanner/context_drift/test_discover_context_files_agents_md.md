---
type: Rust Function
title: test_discover_context_files_agents_md
resource: src/scanner/context_drift/mod.rs#L256-L266
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/temp_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/discover/discover_context_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_discover_context_files_agents_md()`

# Calls

- [temp_file](../../../../functions/src/scanner/context_drift/temp_file.md)
- [discover_context_files](../../../../functions/src/scanner/context_drift/discover/discover_context_files.md)
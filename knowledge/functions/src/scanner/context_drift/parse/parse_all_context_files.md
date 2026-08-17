---
type: Rust Function
title: parse_all_context_files
resource: src/scanner/context_drift/parse.rs#L346-L358
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/discover_context_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/discover/detect_file_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_context_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/discover_claims
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_all_context_files(root: &Path) -> Vec<ContextClaim>`

# Calls

- [discover_context_files](../../../../../functions/src/scanner/context_drift/discover/discover_context_files.md)
- [detect_file_type](../../../../../functions/src/scanner/context_drift/discover/detect_file_type.md)
- [parse_context_file](../../../../../functions/src/scanner/context_drift/parse/parse_context_file.md)

# Called by

- [discover_claims](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/discover_claims.md)
---
type: Rust Method
title: discover_claims
resource: src/scanner/context_drift/mod.rs#L137-L139
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/parse/parse_all_context_files
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn discover_claims(&self) -> Vec<ContextClaim>`

# Calls

- [parse_all_context_files](../../../../../functions/src/scanner/context_drift/parse/parse_all_context_files.md)

# Called by

- [scan_drift](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
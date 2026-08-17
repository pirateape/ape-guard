---
type: Rust Function
title: test_full_drift_scan_with_claims
resource: src/scanner/context_drift/mod.rs#L526-L557
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/ContextDriftScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_full_drift_scan_with_claims()`

# Calls

- [scan_drift](../../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
- [new](../../../../functions/src/scanner/context_drift/ContextDriftScanner/new.md)
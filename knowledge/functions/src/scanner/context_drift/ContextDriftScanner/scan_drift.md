---
type: Rust Method
title: scan_drift
resource: src/scanner/context_drift/mod.rs#L97-L134
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/discover_context_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/ContextDriftScanner/discover_claims
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_claims
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/count_by_severity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_full_drift_scan_no_context_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_full_drift_scan_with_claims
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn scan_drift(&self) -> DriftScanResult`

# Calls

- [discover_context_files](../../../../../functions/src/scanner/context_drift/discover/discover_context_files.md)
- [discover_claims](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/discover_claims.md)
- [verify_claims](../../../../../functions/src/scanner/context_drift/verify/verify_claims.md)
- [count_by_severity](../../../../../functions/src/scanner/context_drift/count_by_severity.md)

# Called by

- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_full_drift_scan_no_context_files](../../../../../functions/src/scanner/context_drift/test_full_drift_scan_no_context_files.md)
- [test_full_drift_scan_with_claims](../../../../../functions/src/scanner/context_drift/test_full_drift_scan_with_claims.md)
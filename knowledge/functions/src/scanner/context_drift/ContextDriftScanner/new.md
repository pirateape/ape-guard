---
type: Rust Method
title: new
resource: src/scanner/context_drift/mod.rs#L74-L80
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/orchestrate/run_scan
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/ContextDriftScanner/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_full_drift_scan_no_context_files
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_full_drift_scan_with_claims
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_scanner_name_and_type
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new(root: &Path) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [default](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/default/default.md)
- [test_full_drift_scan_no_context_files](../../../../../functions/src/scanner/context_drift/test_full_drift_scan_no_context_files.md)
- [test_full_drift_scan_with_claims](../../../../../functions/src/scanner/context_drift/test_full_drift_scan_with_claims.md)
- [test_scanner_name_and_type](../../../../../functions/src/scanner/context_drift/test_scanner_name_and_type.md)
---
type: Rust Method
title: scan
resource: src/scanner/mod.rs#L58-L77
visibility: private
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
---

# Signature

`async fn scan(&self, path: &Path) -> Result<ScannerResult, ScannerError>`

# Called by

- [handle_scan_tool](../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../functions/src/orchestrate/run_scan.md)
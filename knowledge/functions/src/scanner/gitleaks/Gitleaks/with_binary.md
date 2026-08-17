---
type: Rust Method
title: with_binary
resource: src/scanner/gitleaks.rs#L22-L26
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

`pub fn with_binary(path: Option<String>) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
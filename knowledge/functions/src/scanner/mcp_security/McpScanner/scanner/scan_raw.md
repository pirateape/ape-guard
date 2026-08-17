---
type: Rust Method
title: scan_raw
resource: src/scanner/mcp_security.rs#L62-L93
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/binary_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/run_command_with_timeout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError>`

# Calls

- [binary_exists](../../../../../../functions/src/scanner/binary_exists.md)
- [run_command_with_timeout](../../../../../../functions/src/scanner/run_command_with_timeout.md)
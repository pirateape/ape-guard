---
type: Rust Method
title: scan_raw
resource: src/scanner/container.rs#L55-L61
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/run_command_with_timeout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError>`

# Calls

- [run_command_with_timeout](../../../../../../functions/src/scanner/run_command_with_timeout.md)
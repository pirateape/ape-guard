---
type: Rust Method
title: scan_raw
resource: src/scanner/tls.rs#L61-L109
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/binary_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError>`

# Calls

- [binary_exists](../../../../../../functions/src/scanner/binary_exists.md)
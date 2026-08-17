---
type: Rust Method
title: check_installed
resource: src/scanner/gitleaks.rs#L43-L48
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

`async fn check_installed(&self) -> Result<bool, ScannerError>`

# Calls

- [binary_exists](../../../../../../functions/src/scanner/binary_exists.md)
---
type: Rust Method
title: mode_flag
resource: src/scanner/trivy.rs#L44-L50
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/trivy/Trivy/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mode_flag(&self) -> &'static str`

# Called by

- [scan_raw](../../../../../functions/src/scanner/trivy/Trivy/scanner/scan_raw.md)
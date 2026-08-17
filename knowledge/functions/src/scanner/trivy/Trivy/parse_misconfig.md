---
type: Rust Method
title: parse_misconfig
resource: src/scanner/trivy.rs#L329-L416
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/trivy/Trivy/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_misconfig(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [parse_output](../../../../../functions/src/scanner/trivy/Trivy/scanner/parse_output.md)
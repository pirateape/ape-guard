---
type: Rust Method
title: parse_output
resource: src/scanner/dast.rs#L68-L70
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/dast/DastScanner/parse_nuclei_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Calls

- [parse_nuclei_json](../../../../../../functions/src/scanner/dast/DastScanner/parse_nuclei_json.md)
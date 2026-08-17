---
type: Rust Method
title: parse_secret
resource: src/scanner/trivy.rs#L243-L327
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/trivy/Trivy/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/test_parse_secret_fixture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_secret(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [parse_output](../../../../../functions/src/scanner/trivy/Trivy/scanner/parse_output.md)
- [test_parse_secret_fixture](../../../../../functions/src/scanner/trivy/test_parse_secret_fixture.md)
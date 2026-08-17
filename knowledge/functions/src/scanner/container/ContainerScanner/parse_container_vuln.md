---
type: Rust Method
title: parse_container_vuln
resource: src/scanner/container.rs#L70-L194
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/container/ContainerScanner/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/container/test_parse_container_vuln_real_fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/container/test_parse_container_vuln_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_container_vuln(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [parse_output](../../../../../functions/src/scanner/container/ContainerScanner/scanner/parse_output.md)
- [test_parse_container_vuln_real_fixture](../../../../../functions/src/scanner/container/test_parse_container_vuln_real_fixture.md)
- [test_parse_container_vuln_empty](../../../../../functions/src/scanner/container/test_parse_container_vuln_empty.md)
---
type: Rust Method
title: parse_output
resource: src/scanner/container.rs#L63-L66
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/container/ContainerScanner/parse_container_vuln
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Calls

- [parse_container_vuln](../../../../../../functions/src/scanner/container/ContainerScanner/parse_container_vuln.md)
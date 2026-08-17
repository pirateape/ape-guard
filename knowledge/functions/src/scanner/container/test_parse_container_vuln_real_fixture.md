---
type: Rust Function
title: test_parse_container_vuln_real_fixture
resource: src/scanner/container.rs#L202-L240
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/container/ContainerScanner/parse_container_vuln
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/container/ContainerScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_container_vuln_real_fixture()`

# Calls

- [parse_container_vuln](../../../../functions/src/scanner/container/ContainerScanner/parse_container_vuln.md)
- [new](../../../../functions/src/scanner/container/ContainerScanner/new.md)
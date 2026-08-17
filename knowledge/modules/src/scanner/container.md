---
type: Rust Module
title: container
resource: src/scanner/container.rs#L1-L257
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-scanner-scannererror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ContainerScanner](../../../classes/src/scanner/container/ContainerScanner.md)
- [new](../../../functions/src/scanner/container/ContainerScanner/new.md)
- [name](../../../functions/src/scanner/container/ContainerScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/container/ContainerScanner/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/container/ContainerScanner/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/container/ContainerScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/container/ContainerScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/container/ContainerScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/container/ContainerScanner/scanner/parse_output.md)
- [parse_container_vuln](../../../functions/src/scanner/container/ContainerScanner/parse_container_vuln.md)
- [TrivyReport](../../../classes/src/scanner/container/TrivyReport.md)
- [TrivyResult](../../../classes/src/scanner/container/TrivyResult.md)
- [TrivyVuln](../../../classes/src/scanner/container/TrivyVuln.md)
- [test_parse_container_vuln_real_fixture](../../../functions/src/scanner/container/test_parse_container_vuln_real_fixture.md)
- [test_parse_container_vuln_empty](../../../functions/src/scanner/container/test_parse_container_vuln_empty.md)
- [test_parse_container_vuln_invalid_json](../../../functions/src/scanner/container/test_parse_container_vuln_invalid_json.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
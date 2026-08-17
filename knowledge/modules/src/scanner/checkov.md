---
type: Rust Module
title: checkov
resource: src/scanner/checkov.rs#L1-L248
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

- [Checkov](../../../classes/src/scanner/checkov/Checkov.md)
- [new](../../../functions/src/scanner/checkov/Checkov/new.md)
- [with_binary](../../../functions/src/scanner/checkov/Checkov/with_binary.md)
- [name](../../../functions/src/scanner/checkov/Checkov/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/checkov/Checkov/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/checkov/Checkov/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/checkov/Checkov/scanner/check_installed.md)
- [version](../../../functions/src/scanner/checkov/Checkov/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/checkov/Checkov/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/checkov/Checkov/scanner/parse_output.md)
- [CheckovReport](../../../classes/src/scanner/checkov/CheckovReport.md)
- [CheckovResults](../../../classes/src/scanner/checkov/CheckovResults.md)
- [CheckovFinding](../../../classes/src/scanner/checkov/CheckovFinding.md)
- [test_parse_output_real_fixture](../../../functions/src/scanner/checkov/test_parse_output_real_fixture.md)
- [test_parse_output_empty](../../../functions/src/scanner/checkov/test_parse_output_empty.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
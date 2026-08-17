---
type: Rust Module
title: syft
resource: src/scanner/syft.rs#L1-L201
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

- [Syft](../../../classes/src/scanner/syft/Syft.md)
- [new](../../../functions/src/scanner/syft/Syft/new.md)
- [with_binary](../../../functions/src/scanner/syft/Syft/with_binary.md)
- [name](../../../functions/src/scanner/syft/Syft/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/syft/Syft/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/syft/Syft/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/syft/Syft/scanner/check_installed.md)
- [version](../../../functions/src/scanner/syft/Syft/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/syft/Syft/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/syft/Syft/scanner/parse_output.md)
- [SyftReport](../../../classes/src/scanner/syft/SyftReport.md)
- [SyftArtifact](../../../classes/src/scanner/syft/SyftArtifact.md)
- [SyftLocation](../../../classes/src/scanner/syft/SyftLocation.md)
- [test_parse_output_real_fixture](../../../functions/src/scanner/syft/test_parse_output_real_fixture.md)
- [test_parse_output_empty](../../../functions/src/scanner/syft/test_parse_output_empty.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
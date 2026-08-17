---
type: Rust Module
title: gitleaks
resource: src/scanner/gitleaks.rs#L1-L291
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

- [Gitleaks](../../../classes/src/scanner/gitleaks/Gitleaks.md)
- [new](../../../functions/src/scanner/gitleaks/Gitleaks/new.md)
- [with_binary](../../../functions/src/scanner/gitleaks/Gitleaks/with_binary.md)
- [name](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/check_installed.md)
- [version](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/parse_output.md)
- [GitleaksFinding](../../../classes/src/scanner/gitleaks/GitleaksFinding.md)
- [test_parse_output_real_fixture](../../../functions/src/scanner/gitleaks/test_parse_output_real_fixture.md)
- [test_parse_output_empty_array](../../../functions/src/scanner/gitleaks/test_parse_output_empty_array.md)
- [test_parse_output_empty_bytes](../../../functions/src/scanner/gitleaks/test_parse_output_empty_bytes.md)
- [test_parse_output_single_object_fallback](../../../functions/src/scanner/gitleaks/test_parse_output_single_object_fallback.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
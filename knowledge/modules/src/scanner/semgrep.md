---
type: Rust Module
title: semgrep
resource: src/scanner/semgrep.rs#L1-L282
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

- [Semgrep](../../../classes/src/scanner/semgrep/Semgrep.md)
- [new](../../../functions/src/scanner/semgrep/Semgrep/new.md)
- [with_binary](../../../functions/src/scanner/semgrep/Semgrep/with_binary.md)
- [name](../../../functions/src/scanner/semgrep/Semgrep/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/semgrep/Semgrep/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/semgrep/Semgrep/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/semgrep/Semgrep/scanner/check_installed.md)
- [version](../../../functions/src/scanner/semgrep/Semgrep/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/semgrep/Semgrep/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/semgrep/Semgrep/scanner/parse_output.md)
- [SemgrepResults](../../../classes/src/scanner/semgrep/SemgrepResults.md)
- [SemgrepFinding](../../../classes/src/scanner/semgrep/SemgrepFinding.md)
- [SemgrepLocation](../../../classes/src/scanner/semgrep/SemgrepLocation.md)
- [SemgrepExtra](../../../classes/src/scanner/semgrep/SemgrepExtra.md)
- [SemgrepError](../../../classes/src/scanner/semgrep/SemgrepError.md)
- [test_parse_output_real_fixture](../../../functions/src/scanner/semgrep/test_parse_output_real_fixture.md)
- [test_parse_output_empty_results](../../../functions/src/scanner/semgrep/test_parse_output_empty_results.md)
- [test_parse_output_info_severity](../../../functions/src/scanner/semgrep/test_parse_output_info_severity.md)
- [test_parse_output_invalid_json](../../../functions/src/scanner/semgrep/test_parse_output_invalid_json.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
---
type: Rust Module
title: dast
resource: src/scanner/dast.rs#L1-L315
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

- [DastScanner](../../../classes/src/scanner/dast/DastScanner.md)
- [new](../../../functions/src/scanner/dast/DastScanner/new.md)
- [name](../../../functions/src/scanner/dast/DastScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/dast/DastScanner/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/dast/DastScanner/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/dast/DastScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/dast/DastScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/dast/DastScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/dast/DastScanner/scanner/parse_output.md)
- [parse_nuclei_json](../../../functions/src/scanner/dast/DastScanner/parse_nuclei_json.md)
- [NucleiResult](../../../classes/src/scanner/dast/NucleiResult.md)
- [NucleiInfo](../../../classes/src/scanner/dast/NucleiInfo.md)
- [NucleiCvss](../../../classes/src/scanner/dast/NucleiCvss.md)
- [test_parse_nuclei_json_real_fixture](../../../functions/src/scanner/dast/test_parse_nuclei_json_real_fixture.md)
- [test_parse_nuclei_json_empty](../../../functions/src/scanner/dast/test_parse_nuclei_json_empty.md)
- [test_parse_nuclei_json_skips_malformed_lines](../../../functions/src/scanner/dast/test_parse_nuclei_json_skips_malformed_lines.md)
- [test_parse_nuclei_json_severity_from_info_block](../../../functions/src/scanner/dast/test_parse_nuclei_json_severity_from_info_block.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
---
type: Rust Module
title: tls
resource: src/scanner/tls.rs#L1-L324
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-scanner-scannererror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
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

- [TlsScanner](../../../classes/src/scanner/tls/TlsScanner.md)
- [new](../../../functions/src/scanner/tls/TlsScanner/new.md)
- [name](../../../functions/src/scanner/tls/TlsScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/tls/TlsScanner/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/tls/TlsScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/tls/TlsScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/tls/TlsScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/tls/TlsScanner/scanner/parse_output.md)
- [install_hint](../../../functions/src/scanner/tls/TlsScanner/scanner/install_hint.md)
- [test_tls_scanner_name](../../../functions/src/scanner/tls/test_tls_scanner_name.md)
- [test_tls_scanner_type](../../../functions/src/scanner/tls/test_tls_scanner_type.md)
- [test_tls_scanner_installed](../../../functions/src/scanner/tls/test_tls_scanner_installed.md)
- [test_parse_output_manual_inspection](../../../functions/src/scanner/tls/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../functions/src/scanner/tls/test_parse_output_empty_input.md)
- [test_parse_output_expired_path](../../../functions/src/scanner/tls/test_parse_output_expired_path.md)
- [test_parse_output_cert_found_path](../../../functions/src/scanner/tls/test_parse_output_cert_found_path.md)
- [test_parse_output_not_found_path](../../../functions/src/scanner/tls/test_parse_output_not_found_path.md)
- [test_parse_output_ok_response](../../../functions/src/scanner/tls/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../functions/src/scanner/tls/test_parse_output_no_relevant_lines.md)

# Imports

- `super::{Scanner, ScannerError}`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `async_trait::async_trait`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
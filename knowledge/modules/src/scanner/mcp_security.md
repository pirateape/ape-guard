---
type: Rust Module
title: mcp_security
resource: src/scanner/mcp_security.rs#L1-L296
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

- [McpScanner](../../../classes/src/scanner/mcp_security/McpScanner.md)
- [new](../../../functions/src/scanner/mcp_security/McpScanner/new.md)
- [name](../../../functions/src/scanner/mcp_security/McpScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/mcp_security/McpScanner/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/mcp_security/McpScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/mcp_security/McpScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/mcp_security/McpScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/mcp_security/McpScanner/scanner/parse_output.md)
- [install_hint](../../../functions/src/scanner/mcp_security/McpScanner/scanner/install_hint.md)
- [test_mcp_scanner_name](../../../functions/src/scanner/mcp_security/test_mcp_scanner_name.md)
- [test_mcp_scanner_type](../../../functions/src/scanner/mcp_security/test_mcp_scanner_type.md)
- [test_mcp_scanner_installed](../../../functions/src/scanner/mcp_security/test_mcp_scanner_installed.md)
- [test_parse_output_manual_inspection](../../../functions/src/scanner/mcp_security/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../functions/src/scanner/mcp_security/test_parse_output_empty_input.md)
- [test_parse_output_ssrf_detection](../../../functions/src/scanner/mcp_security/test_parse_output_ssrf_detection.md)
- [test_parse_output_ok_response](../../../functions/src/scanner/mcp_security/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../functions/src/scanner/mcp_security/test_parse_output_no_relevant_lines.md)

# Imports

- `super::{Scanner, ScannerError}`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `async_trait::async_trait`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
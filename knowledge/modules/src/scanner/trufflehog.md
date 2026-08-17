---
type: Rust Module
title: trufflehog
resource: src/scanner/trufflehog.rs#L1-L672
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

- [Trufflehog](../../../classes/src/scanner/trufflehog/Trufflehog.md)
- [new](../../../functions/src/scanner/trufflehog/Trufflehog/new.md)
- [with_binary](../../../functions/src/scanner/trufflehog/Trufflehog/with_binary.md)
- [name](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/check_installed.md)
- [version](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output.md)
- [parse_json_array](../../../functions/src/scanner/trufflehog/Trufflehog/parse_json_array.md)
- [parse_trufflehog_line](../../../functions/src/scanner/trufflehog/parse_trufflehog_line.md)
- [TrufflehogResult](../../../classes/src/scanner/trufflehog/TrufflehogResult.md)
- [SourceMetadata](../../../classes/src/scanner/trufflehog/SourceMetadata.md)
- [SourceData](../../../classes/src/scanner/trufflehog/SourceData.md)
- [FileSource](../../../classes/src/scanner/trufflehog/FileSource.md)
- [GitSource](../../../classes/src/scanner/trufflehog/GitSource.md)
- [detector_type_to_severity](../../../functions/src/scanner/trufflehog/detector_type_to_severity.md)
- [test_parse_output_single_finding](../../../functions/src/scanner/trufflehog/test_parse_output_single_finding.md)
- [test_parse_output_unverified](../../../functions/src/scanner/trufflehog/test_parse_output_unverified.md)
- [test_parse_output_multiple_lines](../../../functions/src/scanner/trufflehog/test_parse_output_multiple_lines.md)
- [test_parse_output_empty](../../../functions/src/scanner/trufflehog/test_parse_output_empty.md)
- [test_parse_output_whitespace_only](../../../functions/src/scanner/trufflehog/test_parse_output_whitespace_only.md)
- [test_parse_output_git_source](../../../functions/src/scanner/trufflehog/test_parse_output_git_source.md)
- [test_parse_output_skip_no_secret](../../../functions/src/scanner/trufflehog/test_parse_output_skip_no_secret.md)
- [test_detector_type_severity](../../../functions/src/scanner/trufflehog/test_detector_type_severity.md)
- [test_scanner_name](../../../functions/src/scanner/trufflehog/test_scanner_name.md)
- [test_scanner_type](../../../functions/src/scanner/trufflehog/test_scanner_type.md)
- [test_install_hint](../../../functions/src/scanner/trufflehog/test_install_hint.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
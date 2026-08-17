---
type: Rust Module
title: aws_s3
resource: src/scanner/aws_s3.rs#L1-L287
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

- [AwsS3Scanner](../../../classes/src/scanner/aws_s3/AwsS3Scanner.md)
- [new](../../../functions/src/scanner/aws_s3/AwsS3Scanner/new.md)
- [name](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/parse_output.md)
- [install_hint](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/install_hint.md)
- [test_aws_s3_scanner_name](../../../functions/src/scanner/aws_s3/test_aws_s3_scanner_name.md)
- [test_aws_s3_scanner_type](../../../functions/src/scanner/aws_s3/test_aws_s3_scanner_type.md)
- [test_aws_s3_scanner_installed](../../../functions/src/scanner/aws_s3/test_aws_s3_scanner_installed.md)
- [test_parse_output_manual_inspection](../../../functions/src/scanner/aws_s3/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../functions/src/scanner/aws_s3/test_parse_output_empty_input.md)
- [test_parse_output_ok_response](../../../functions/src/scanner/aws_s3/test_parse_output_ok_response.md)
- [test_parse_output_aws_cli_json](../../../functions/src/scanner/aws_s3/test_parse_output_aws_cli_json.md)
- [test_parse_output_no_relevant_lines](../../../functions/src/scanner/aws_s3/test_parse_output_no_relevant_lines.md)

# Imports

- `super::{Scanner, ScannerError}`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `async_trait::async_trait`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
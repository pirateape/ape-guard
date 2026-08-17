---
type: Rust Module
title: terraform
resource: src/scanner/terraform.rs#L1-L353
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

- [TerraformScanner](../../../classes/src/scanner/terraform/TerraformScanner.md)
- [new](../../../functions/src/scanner/terraform/TerraformScanner/new.md)
- [name](../../../functions/src/scanner/terraform/TerraformScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/terraform/TerraformScanner/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/terraform/TerraformScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/terraform/TerraformScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/terraform/TerraformScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/terraform/TerraformScanner/scanner/parse_output.md)
- [CheckovReport](../../../classes/src/scanner/terraform/CheckovReport.md)
- [CheckovResults](../../../classes/src/scanner/terraform/CheckovResults.md)
- [CheckovFinding](../../../classes/src/scanner/terraform/CheckovFinding.md)
- [install_hint](../../../functions/src/scanner/terraform/TerraformScanner/scanner/install_hint.md)
- [test_terraform_scanner_name](../../../functions/src/scanner/terraform/test_terraform_scanner_name.md)
- [test_terraform_scanner_type](../../../functions/src/scanner/terraform/test_terraform_scanner_type.md)
- [test_terraform_scanner_installed](../../../functions/src/scanner/terraform/test_terraform_scanner_installed.md)
- [test_parse_output_manual_inspection](../../../functions/src/scanner/terraform/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../functions/src/scanner/terraform/test_parse_output_empty_input.md)
- [test_parse_output_overly_permissive_iam](../../../functions/src/scanner/terraform/test_parse_output_overly_permissive_iam.md)
- [test_parse_output_ok_response](../../../functions/src/scanner/terraform/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../functions/src/scanner/terraform/test_parse_output_no_relevant_lines.md)

# Imports

- `super::{Scanner, ScannerError}`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `async_trait::async_trait`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
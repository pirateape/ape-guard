---
type: Rust Method
title: new
resource: src/scanner/terraform.rs#L15-L19
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/orchestrate/run_scan
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_terraform_scanner_name
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_terraform_scanner_type
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_terraform_scanner_installed
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_parse_output_manual_inspection
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_parse_output_empty_input
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_parse_output_overly_permissive_iam
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_parse_output_ok_response
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/test_parse_output_no_relevant_lines
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new(terraform_dir: &str) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_terraform_scanner_name](../../../../../functions/src/scanner/terraform/test_terraform_scanner_name.md)
- [test_terraform_scanner_type](../../../../../functions/src/scanner/terraform/test_terraform_scanner_type.md)
- [test_terraform_scanner_installed](../../../../../functions/src/scanner/terraform/test_terraform_scanner_installed.md)
- [test_parse_output_manual_inspection](../../../../../functions/src/scanner/terraform/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../../../functions/src/scanner/terraform/test_parse_output_empty_input.md)
- [test_parse_output_overly_permissive_iam](../../../../../functions/src/scanner/terraform/test_parse_output_overly_permissive_iam.md)
- [test_parse_output_ok_response](../../../../../functions/src/scanner/terraform/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../../../functions/src/scanner/terraform/test_parse_output_no_relevant_lines.md)
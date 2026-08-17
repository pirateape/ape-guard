---
type: Rust Method
title: parse_output
resource: src/scanner/terraform.rs#L125-L276
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
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

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_manual_inspection](../../../../../../functions/src/scanner/terraform/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../../../../functions/src/scanner/terraform/test_parse_output_empty_input.md)
- [test_parse_output_overly_permissive_iam](../../../../../../functions/src/scanner/terraform/test_parse_output_overly_permissive_iam.md)
- [test_parse_output_ok_response](../../../../../../functions/src/scanner/terraform/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../../../../functions/src/scanner/terraform/test_parse_output_no_relevant_lines.md)
---
type: Rust Function
title: test_parse_output_no_relevant_lines
resource: src/scanner/terraform.rs#L345-L352
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/terraform/TerraformScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/TerraformScanner/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_no_relevant_lines()`

# Calls

- [new](../../../../functions/src/scanner/terraform/TerraformScanner/new.md)
- [parse_output](../../../../functions/src/scanner/terraform/TerraformScanner/scanner/parse_output.md)
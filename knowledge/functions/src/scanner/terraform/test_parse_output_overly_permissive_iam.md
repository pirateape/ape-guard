---
type: Rust Function
title: test_parse_output_overly_permissive_iam
resource: src/scanner/terraform.rs#L324-L332
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

`fn test_parse_output_overly_permissive_iam()`

# Calls

- [new](../../../../functions/src/scanner/terraform/TerraformScanner/new.md)
- [parse_output](../../../../functions/src/scanner/terraform/TerraformScanner/scanner/parse_output.md)
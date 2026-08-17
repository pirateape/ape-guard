---
type: Rust Function
title: test_parse_output_manual_inspection
resource: src/scanner/aws_s3.rs#L239-L246
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_manual_inspection()`

# Calls

- [new](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/new.md)
- [parse_output](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/parse_output.md)
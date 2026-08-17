---
type: Rust Function
title: test_parse_output_no_relevant_lines
resource: src/scanner/aws_s3.rs#L279-L286
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

`fn test_parse_output_no_relevant_lines()`

# Calls

- [new](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/new.md)
- [parse_output](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/parse_output.md)
---
type: Rust Function
title: test_parse_output_aws_cli_json
resource: src/scanner/aws_s3.rs#L268-L276
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

`fn test_parse_output_aws_cli_json()`

# Calls

- [new](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/new.md)
- [parse_output](../../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/parse_output.md)
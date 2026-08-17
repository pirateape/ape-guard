---
type: Rust Function
title: test_parse_output_skip_no_secret
resource: src/scanner/trufflehog.rs#L604-L626
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trufflehog/Trufflehog/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_skip_no_secret()`

# Calls

- [new](../../../../functions/src/scanner/trufflehog/Trufflehog/new.md)
- [parse_output](../../../../functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output.md)
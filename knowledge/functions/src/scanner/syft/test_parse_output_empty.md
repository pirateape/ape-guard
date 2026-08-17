---
type: Rust Function
title: test_parse_output_empty
resource: src/scanner/syft.rs#L194-L200
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/syft/Syft/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/syft/Syft/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_empty()`

# Calls

- [new](../../../../functions/src/scanner/syft/Syft/new.md)
- [parse_output](../../../../functions/src/scanner/syft/Syft/scanner/parse_output.md)
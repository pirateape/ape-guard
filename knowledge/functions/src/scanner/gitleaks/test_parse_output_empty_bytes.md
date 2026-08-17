---
type: Rust Function
title: test_parse_output_empty_bytes
resource: src/scanner/gitleaks.rs#L262-L268
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/gitleaks/Gitleaks/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/gitleaks/Gitleaks/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_empty_bytes()`

# Calls

- [new](../../../../functions/src/scanner/gitleaks/Gitleaks/new.md)
- [parse_output](../../../../functions/src/scanner/gitleaks/Gitleaks/scanner/parse_output.md)
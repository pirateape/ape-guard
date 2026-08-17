---
type: Rust Function
title: test_parse_secret_fixture
resource: src/scanner/trivy.rs#L526-L550
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trivy/Trivy/with_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/parse_secret
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_parse_secret_fixture()`

# Calls

- [with_mode](../../../../functions/src/scanner/trivy/Trivy/with_mode.md)
- [parse_secret](../../../../functions/src/scanner/trivy/Trivy/parse_secret.md)
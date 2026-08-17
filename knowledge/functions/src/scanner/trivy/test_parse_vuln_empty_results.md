---
type: Rust Function
title: test_parse_vuln_empty_results
resource: src/scanner/trivy.rs#L498-L505
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trivy/make_trivy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/parse_vuln
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_parse_vuln_empty_results()`

# Calls

- [make_trivy](../../../../functions/src/scanner/trivy/make_trivy.md)
- [parse_vuln](../../../../functions/src/scanner/trivy/Trivy/parse_vuln.md)
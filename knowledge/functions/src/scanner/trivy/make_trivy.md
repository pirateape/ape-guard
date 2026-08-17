---
type: Rust Function
title: make_trivy
resource: src/scanner/trivy.rs#L423-L425
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trivy/Trivy/with_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/trivy/test_parse_vuln_real_fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/test_parse_vuln_empty_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/test_parse_vuln_no_vulnerabilities_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/test_parse_vuln_invalid_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_trivy() -> Trivy`

# Calls

- [with_mode](../../../../functions/src/scanner/trivy/Trivy/with_mode.md)

# Called by

- [test_parse_vuln_real_fixture](../../../../functions/src/scanner/trivy/test_parse_vuln_real_fixture.md)
- [test_parse_vuln_empty_results](../../../../functions/src/scanner/trivy/test_parse_vuln_empty_results.md)
- [test_parse_vuln_no_vulnerabilities_key](../../../../functions/src/scanner/trivy/test_parse_vuln_no_vulnerabilities_key.md)
- [test_parse_vuln_invalid_json](../../../../functions/src/scanner/trivy/test_parse_vuln_invalid_json.md)
---
type: Rust Method
title: parse_vuln
resource: src/scanner/trivy.rs#L125-L241
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/trivy/Trivy/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
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

`fn parse_vuln(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [parse_output](../../../../../functions/src/scanner/trivy/Trivy/scanner/parse_output.md)
- [test_parse_vuln_real_fixture](../../../../../functions/src/scanner/trivy/test_parse_vuln_real_fixture.md)
- [test_parse_vuln_empty_results](../../../../../functions/src/scanner/trivy/test_parse_vuln_empty_results.md)
- [test_parse_vuln_no_vulnerabilities_key](../../../../../functions/src/scanner/trivy/test_parse_vuln_no_vulnerabilities_key.md)
- [test_parse_vuln_invalid_json](../../../../../functions/src/scanner/trivy/test_parse_vuln_invalid_json.md)
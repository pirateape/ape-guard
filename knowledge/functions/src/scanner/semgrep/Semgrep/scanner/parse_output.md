---
type: Rust Method
title: parse_output
resource: src/scanner/semgrep.rs#L75-L179
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/semgrep/test_parse_output_real_fixture
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/semgrep/test_parse_output_empty_results
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/semgrep/test_parse_output_info_severity
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_real_fixture](../../../../../../functions/src/scanner/semgrep/test_parse_output_real_fixture.md)
- [test_parse_output_empty_results](../../../../../../functions/src/scanner/semgrep/test_parse_output_empty_results.md)
- [test_parse_output_info_severity](../../../../../../functions/src/scanner/semgrep/test_parse_output_info_severity.md)
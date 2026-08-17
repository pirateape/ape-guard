---
type: Rust Method
title: new
resource: src/scanner/semgrep.rs#L15-L19
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
  - target: functions/src/scanner/semgrep/test_parse_output_invalid_json
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [test_parse_output_real_fixture](../../../../../functions/src/scanner/semgrep/test_parse_output_real_fixture.md)
- [test_parse_output_empty_results](../../../../../functions/src/scanner/semgrep/test_parse_output_empty_results.md)
- [test_parse_output_info_severity](../../../../../functions/src/scanner/semgrep/test_parse_output_info_severity.md)
- [test_parse_output_invalid_json](../../../../../functions/src/scanner/semgrep/test_parse_output_invalid_json.md)
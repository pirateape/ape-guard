---
type: Rust Method
title: parse_nuclei_json
resource: src/scanner/dast.rs#L74-L240
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/dast/DastScanner/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/test_parse_nuclei_json_real_fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/test_parse_nuclei_json_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/test_parse_nuclei_json_skips_malformed_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/test_parse_nuclei_json_severity_from_info_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_nuclei_json(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [parse_output](../../../../../functions/src/scanner/dast/DastScanner/scanner/parse_output.md)
- [test_parse_nuclei_json_real_fixture](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_real_fixture.md)
- [test_parse_nuclei_json_empty](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_empty.md)
- [test_parse_nuclei_json_skips_malformed_lines](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_skips_malformed_lines.md)
- [test_parse_nuclei_json_severity_from_info_block](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_severity_from_info_block.md)
---
type: Rust Method
title: new
resource: src/scanner/dast.rs#L15-L20
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/orchestrate/run_scan
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/dast/test_parse_nuclei_json_real_fixture
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/dast/test_parse_nuclei_json_empty
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/dast/test_parse_nuclei_json_skips_malformed_lines
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/dast/test_parse_nuclei_json_severity_from_info_block
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new(target: &str) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_parse_nuclei_json_real_fixture](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_real_fixture.md)
- [test_parse_nuclei_json_empty](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_empty.md)
- [test_parse_nuclei_json_skips_malformed_lines](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_skips_malformed_lines.md)
- [test_parse_nuclei_json_severity_from_info_block](../../../../../functions/src/scanner/dast/test_parse_nuclei_json_severity_from_info_block.md)
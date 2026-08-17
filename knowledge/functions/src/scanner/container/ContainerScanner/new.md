---
type: Rust Method
title: new
resource: src/scanner/container.rs#L15-L20
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
  - target: functions/src/scanner/container/test_parse_container_vuln_real_fixture
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/container/test_parse_container_vuln_empty
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/container/test_parse_container_vuln_invalid_json
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new(image: &str) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_parse_container_vuln_real_fixture](../../../../../functions/src/scanner/container/test_parse_container_vuln_real_fixture.md)
- [test_parse_container_vuln_empty](../../../../../functions/src/scanner/container/test_parse_container_vuln_empty.md)
- [test_parse_container_vuln_invalid_json](../../../../../functions/src/scanner/container/test_parse_container_vuln_invalid_json.md)
---
type: Rust Method
title: parse_output
resource: src/scanner/mcp_security.rs#L95-L219
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/mcp_security/test_parse_output_manual_inspection
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/test_parse_output_empty_input
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/test_parse_output_ssrf_detection
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/test_parse_output_ok_response
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/test_parse_output_no_relevant_lines
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_manual_inspection](../../../../../../functions/src/scanner/mcp_security/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../../../../functions/src/scanner/mcp_security/test_parse_output_empty_input.md)
- [test_parse_output_ssrf_detection](../../../../../../functions/src/scanner/mcp_security/test_parse_output_ssrf_detection.md)
- [test_parse_output_ok_response](../../../../../../functions/src/scanner/mcp_security/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../../../../functions/src/scanner/mcp_security/test_parse_output_no_relevant_lines.md)
---
type: Rust Function
title: test_parse_output_ssrf_detection
resource: src/scanner/mcp_security.rs#L267-L275
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/mcp_security/McpScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/McpScanner/scanner/parse_output
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_output_ssrf_detection()`

# Calls

- [new](../../../../functions/src/scanner/mcp_security/McpScanner/new.md)
- [parse_output](../../../../functions/src/scanner/mcp_security/McpScanner/scanner/parse_output.md)
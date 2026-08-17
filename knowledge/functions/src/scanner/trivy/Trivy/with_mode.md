---
type: Rust Method
title: with_mode
resource: src/scanner/trivy.rs#L29-L34
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/make_trivy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/test_parse_secret_fixture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_mode(mode: TrivyMode) -> Self`

# Called by

- [handle_scan_tool](../../../../../functions/src/mcp/handle_scan_tool.md)
- [make_trivy](../../../../../functions/src/scanner/trivy/make_trivy.md)
- [test_parse_secret_fixture](../../../../../functions/src/scanner/trivy/test_parse_secret_fixture.md)
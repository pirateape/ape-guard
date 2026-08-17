---
type: Rust Function
title: handle_call_tool
resource: src/mcp.rs#L245-L281
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_findings_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scorecard_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_chains_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_arch_tool
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_call_tool(id: &Value, params: &Value) -> anyhow::Result<Value>`

# Calls

- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [handle_findings_tool](../../../functions/src/mcp/handle_findings_tool.md)
- [handle_scorecard_tool](../../../functions/src/mcp/handle_scorecard_tool.md)
- [handle_chains_tool](../../../functions/src/mcp/handle_chains_tool.md)
- [handle_arch_tool](../../../functions/src/mcp/handle_arch_tool.md)

# Called by

- [handle_request](../../../functions/src/mcp/handle_request.md)
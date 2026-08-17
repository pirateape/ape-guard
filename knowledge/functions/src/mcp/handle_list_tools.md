---
type: Rust Function
title: handle_list_tools
resource: src/mcp.rs#L153-L242
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_list_tools_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_list_tools(id: &Value) -> Value`

# Called by

- [handle_request](../../../functions/src/mcp/handle_request.md)
- [test_list_tools_response](../../../functions/src/mcp/test_list_tools_response.md)
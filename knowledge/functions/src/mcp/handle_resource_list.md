---
type: Rust Function
title: handle_resource_list
resource: src/mcp.rs#L684-L705
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_resource_list
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_resource_list(id: &Value) -> Value`

# Called by

- [handle_request](../../../functions/src/mcp/handle_request.md)
- [test_resource_list](../../../functions/src/mcp/test_resource_list.md)
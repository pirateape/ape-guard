---
type: Rust Function
title: handle_initialize
resource: src/mcp.rs#L126-L150
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_initialize_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_initialize(id: &Value, params: &Value) -> Value`

# Called by

- [handle_request](../../../functions/src/mcp/handle_request.md)
- [test_initialize_response](../../../functions/src/mcp/test_initialize_response.md)
---
type: Rust Function
title: handle_request
resource: src/mcp.rs#L93-L123
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/handle_initialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_list_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_call_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_resource_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_resource_read
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/serve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_handle_resources_read_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_handle_initialize_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_handle_list_tools_valid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_handle_unknown_method
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_handle_invalid_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_request(line: &str) -> anyhow::Result<Value>`

# Calls

- [handle_initialize](../../../functions/src/mcp/handle_initialize.md)
- [handle_list_tools](../../../functions/src/mcp/handle_list_tools.md)
- [handle_call_tool](../../../functions/src/mcp/handle_call_tool.md)
- [handle_resource_list](../../../functions/src/mcp/handle_resource_list.md)
- [handle_resource_read](../../../functions/src/mcp/handle_resource_read.md)

# Called by

- [serve](../../../functions/src/mcp/serve.md)
- [test_handle_resources_read_valid](../../../functions/src/mcp/test_handle_resources_read_valid.md)
- [test_handle_initialize_valid](../../../functions/src/mcp/test_handle_initialize_valid.md)
- [test_handle_list_tools_valid](../../../functions/src/mcp/test_handle_list_tools_valid.md)
- [test_handle_unknown_method](../../../functions/src/mcp/test_handle_unknown_method.md)
- [test_handle_invalid_json](../../../functions/src/mcp/test_handle_invalid_json.md)
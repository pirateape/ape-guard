---
type: Rust Function
title: serve
resource: src/mcp.rs#L58-L90
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn serve() -> anyhow::Result<()>`

# Calls

- [handle_request](../../../functions/src/mcp/handle_request.md)

# Called by

- [main](../../../functions/src/main.md)
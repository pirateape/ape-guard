---
type: Rust Function
title: handle_scorecard_tool
resource: src/mcp.rs#L569-L603
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/handle_call_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_scorecard_tool() -> anyhow::Result<Value>`

# Calls

- [load_cached_findings](../../../functions/src/mcp/load_cached_findings.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)

# Called by

- [handle_call_tool](../../../functions/src/mcp/handle_call_tool.md)
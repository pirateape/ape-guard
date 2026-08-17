---
type: Rust Function
title: load_effective_config
resource: src/mcp.rs#L11-L22
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_effective_config() -> anyhow::Result<crate::config::Config>`

# Calls

- [load](../../../functions/src/config/load.md)

# Called by

- [load_cached_findings](../../../functions/src/mcp/load_cached_findings.md)
- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
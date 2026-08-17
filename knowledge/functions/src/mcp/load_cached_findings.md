---
type: Rust Function
title: load_cached_findings
resource: src/mcp.rs#L47-L55
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/get_latest_scan_findings
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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
  - target: functions/src/mcp/handle_resource_read
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_cached_findings() -> anyhow::Result<Option<(String, Vec<CanonicalFinding>)>>`

# Calls

- [load_effective_config](../../../functions/src/mcp/load_effective_config.md)
- [open](../../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../../functions/src/cache/ScanCache/enforce_ttl.md)
- [get_latest_scan_findings](../../../functions/src/cache/ScanCache/get_latest_scan_findings.md)

# Called by

- [handle_findings_tool](../../../functions/src/mcp/handle_findings_tool.md)
- [handle_scorecard_tool](../../../functions/src/mcp/handle_scorecard_tool.md)
- [handle_chains_tool](../../../functions/src/mcp/handle_chains_tool.md)
- [handle_arch_tool](../../../functions/src/mcp/handle_arch_tool.md)
- [handle_resource_read](../../../functions/src/mcp/handle_resource_read.md)
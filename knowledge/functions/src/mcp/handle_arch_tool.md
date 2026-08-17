---
type: Rust Function
title: handle_arch_tool
resource: src/mcp.rs#L634-L681
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/discover_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/assess_component_risks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/generate_mermaid_diagram
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/handle_call_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_arch_tool(args: &Value) -> anyhow::Result<Value>`

# Calls

- [discover_artifacts](../../../functions/src/arch/discover_artifacts.md)
- [load_cached_findings](../../../functions/src/mcp/load_cached_findings.md)
- [assess_component_risks](../../../functions/src/arch/assess_component_risks.md)
- [generate_mermaid_diagram](../../../functions/src/arch/generate_mermaid_diagram.md)

# Called by

- [handle_call_tool](../../../functions/src/mcp/handle_call_tool.md)
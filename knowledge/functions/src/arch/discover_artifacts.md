---
type: Rust Function
title: discover_artifacts
resource: src/arch.rs#L92-L131
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_arch_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/ArchitectureAnalyzer/discover
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn discover_artifacts(root: &Path) -> Vec<ArchitectureArtifact>`

# Calls

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)

# Called by

- [run_report](../../../functions/src/run_report.md)
- [handle_arch_tool](../../../functions/src/mcp/handle_arch_tool.md)
- [discover](../../../functions/src/scanner/arch/ArchitectureAnalyzer/discover.md)
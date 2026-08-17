---
type: Rust Function
title: generate_mermaid_diagram
resource: src/arch.rs#L623-L678
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/sanitize_mermaid_id
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_arch_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/ArchitectureAnalyzer/generate_diagram
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_mermaid_diagram( artifacts: &[ArchitectureArtifact], risks: &[ComponentRisk], ) -> String`

# Calls

- [sanitize_mermaid_id](../../../functions/src/arch/sanitize_mermaid_id.md)

# Called by

- [run_report](../../../functions/src/run_report.md)
- [handle_arch_tool](../../../functions/src/mcp/handle_arch_tool.md)
- [generate_diagram](../../../functions/src/scanner/arch/ArchitectureAnalyzer/generate_diagram.md)
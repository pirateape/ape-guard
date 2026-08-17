---
type: Rust Function
title: assess_component_risks
resource: src/arch.rs#L513-L588
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/generate_component_recommendations
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/test_component_risk_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_arch_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/ArchitectureScanner/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn assess_component_risks( findings: &[CanonicalFinding], artifacts: &[ArchitectureArtifact], ) -> Vec<ComponentRisk>`

# Calls

- [generate_component_recommendations](../../../functions/src/arch/generate_component_recommendations.md)

# Called by

- [test_component_risk_generation](../../../functions/src/arch/test_component_risk_generation.md)
- [run_report](../../../functions/src/run_report.md)
- [handle_arch_tool](../../../functions/src/mcp/handle_arch_tool.md)
- [scan](../../../functions/src/scanner/arch/ArchitectureScanner/scan.md)
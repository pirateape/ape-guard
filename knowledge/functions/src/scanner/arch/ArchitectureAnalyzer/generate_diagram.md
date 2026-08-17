---
type: Rust Method
title: generate_diagram
resource: src/scanner/arch.rs#L36-L42
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/generate_mermaid_diagram
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/arch/ArchitectureScanner/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_diagram( &self, artifacts: &[ArchitectureArtifact], risks: &[ComponentRisk], ) -> String`

# Calls

- [generate_mermaid_diagram](../../../../../functions/src/arch/generate_mermaid_diagram.md)

# Called by

- [scan](../../../../../functions/src/scanner/arch/ArchitectureScanner/scan.md)
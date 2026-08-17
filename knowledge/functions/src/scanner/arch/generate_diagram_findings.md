---
type: Rust Function
title: generate_diagram_findings
resource: src/scanner/arch.rs#L342-L417
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/arch/generate_architecture_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_diagram_findings(artifact: &ArchitectureArtifact) -> Vec<CanonicalFinding>`

# Called by

- [generate_architecture_findings](../../../../functions/src/scanner/arch/generate_architecture_findings.md)
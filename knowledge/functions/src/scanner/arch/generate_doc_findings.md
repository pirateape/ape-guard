---
type: Rust Function
title: generate_doc_findings
resource: src/scanner/arch.rs#L198-L285
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

`fn generate_doc_findings(artifact: &ArchitectureArtifact) -> Vec<CanonicalFinding>`

# Called by

- [generate_architecture_findings](../../../../functions/src/scanner/arch/generate_architecture_findings.md)
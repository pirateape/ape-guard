---
type: Rust Function
title: generate_architecture_findings
resource: src/scanner/arch.rs#L137-L195
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/arch/generate_doc_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/generate_adr_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/generate_diagram_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_architecture_findings(artifacts: &[ArchitectureArtifact]) -> Vec<CanonicalFinding>`

# Calls

- [generate_doc_findings](../../../../functions/src/scanner/arch/generate_doc_findings.md)
- [generate_adr_findings](../../../../functions/src/scanner/arch/generate_adr_findings.md)
- [generate_diagram_findings](../../../../functions/src/scanner/arch/generate_diagram_findings.md)
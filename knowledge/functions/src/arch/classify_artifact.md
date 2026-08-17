---
type: Rust Function
title: classify_artifact
resource: src/arch.rs#L163-L191
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_classify_mermaid_diagram
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_classify_adr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_classify_architecture_doc
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn classify_artifact(path: &Path, filename: &str, dirname: &str) -> ArtifactType`

# Called by

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)
- [test_classify_mermaid_diagram](../../../functions/src/arch/test_classify_mermaid_diagram.md)
- [test_classify_adr](../../../functions/src/arch/test_classify_adr.md)
- [test_classify_architecture_doc](../../../functions/src/arch/test_classify_architecture_doc.md)
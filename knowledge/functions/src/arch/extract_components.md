---
type: Rust Function
title: extract_components
resource: src/arch.rs#L194-L200
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/extract_mermaid_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_markdown_components
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_components(content: &str, artifact_type: &ArtifactType) -> Vec<String>`

# Calls

- [extract_mermaid_components](../../../functions/src/arch/extract_mermaid_components.md)
- [extract_markdown_components](../../../functions/src/arch/extract_markdown_components.md)

# Called by

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)
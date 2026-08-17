---
type: Rust Function
title: parse_artifact
resource: src/arch.rs#L134-L160
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/classify_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/parse_adr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/summarize_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/discover_artifacts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_artifact(path: &Path) -> Option<ArchitectureArtifact>`

# Calls

- [classify_artifact](../../../functions/src/arch/classify_artifact.md)
- [extract_components](../../../functions/src/arch/extract_components.md)
- [extract_dependencies](../../../functions/src/arch/extract_dependencies.md)
- [parse_adr](../../../functions/src/arch/parse_adr.md)
- [summarize_content](../../../functions/src/arch/summarize_content.md)

# Called by

- [discover_artifacts](../../../functions/src/arch/discover_artifacts.md)
---
type: Rust Function
title: extract_mermaid_subgraph
resource: src/arch.rs#L312-L325
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/extract_mermaid_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_mermaid_subgraph
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_mermaid_subgraph(line: &str) -> Option<String>`

# Called by

- [extract_mermaid_components](../../../functions/src/arch/extract_mermaid_components.md)
- [test_extract_mermaid_subgraph](../../../functions/src/arch/test_extract_mermaid_subgraph.md)
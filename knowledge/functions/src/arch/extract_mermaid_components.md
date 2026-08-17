---
type: Rust Function
title: extract_mermaid_components
resource: src/arch.rs#L203-L228
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/extract_mermaid_node
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_mermaid_subgraph
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/extract_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_mermaid_components
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_mermaid_components(content: &str) -> Vec<String>`

# Calls

- [extract_mermaid_node](../../../functions/src/arch/extract_mermaid_node.md)
- [extract_mermaid_subgraph](../../../functions/src/arch/extract_mermaid_subgraph.md)

# Called by

- [extract_components](../../../functions/src/arch/extract_components.md)
- [test_extract_mermaid_components](../../../functions/src/arch/test_extract_mermaid_components.md)
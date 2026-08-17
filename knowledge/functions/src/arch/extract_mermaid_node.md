---
type: Rust Function
title: extract_mermaid_node
resource: src/arch.rs#L231-L256
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/extract_bracket_label
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/extract_mermaid_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_mermaid_node_square
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_mermaid_node_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_mermaid_node_quoted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_mermaid_node(line: &str) -> Option<String>`

# Calls

- [extract_bracket_label](../../../functions/src/arch/extract_bracket_label.md)

# Called by

- [extract_mermaid_components](../../../functions/src/arch/extract_mermaid_components.md)
- [test_extract_mermaid_node_square](../../../functions/src/arch/test_extract_mermaid_node_square.md)
- [test_extract_mermaid_node_round](../../../functions/src/arch/test_extract_mermaid_node_round.md)
- [test_extract_mermaid_node_quoted](../../../functions/src/arch/test_extract_mermaid_node_quoted.md)
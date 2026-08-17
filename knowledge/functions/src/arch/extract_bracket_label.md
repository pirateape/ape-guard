---
type: Rust Function
title: extract_bracket_label
resource: src/arch.rs#L259-L309
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/extract_mermaid_node
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_bracket_label(text: &str) -> Option<String>`

# Called by

- [extract_mermaid_node](../../../functions/src/arch/extract_mermaid_node.md)
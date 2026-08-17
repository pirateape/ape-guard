---
type: Rust Function
title: sanitize_mermaid_id
resource: src/arch.rs#L681-L685
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/generate_mermaid_diagram
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sanitize_mermaid_id(name: &str) -> String`

# Called by

- [generate_mermaid_diagram](../../../functions/src/arch/generate_mermaid_diagram.md)
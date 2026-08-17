---
type: Rust Function
title: extract_adr_context
resource: src/arch.rs#L470-L497
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_adr
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_adr_context(content: &str) -> String`

# Called by

- [parse_adr](../../../functions/src/arch/parse_adr.md)
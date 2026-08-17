---
type: Rust Function
title: summarize_content
resource: src/arch.rs#L500-L510
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/parse_adr
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn summarize_content(content: &str) -> String`

# Called by

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)
- [parse_adr](../../../functions/src/arch/parse_adr.md)
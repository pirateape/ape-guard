---
type: Rust Function
title: parse_adr_title
resource: src/arch.rs#L443-L449
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_adr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_parse_adr_title
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_adr_title(line: &str) -> Option<(String, String)>`

# Called by

- [parse_adr](../../../functions/src/arch/parse_adr.md)
- [test_parse_adr_title](../../../functions/src/arch/test_parse_adr_title.md)
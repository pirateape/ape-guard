---
type: Rust Function
title: extract_status
resource: src/arch.rs#L452-L467
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_adr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_status_accepted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_status(content: &str) -> DecisionStatus`

# Called by

- [parse_adr](../../../functions/src/arch/parse_adr.md)
- [test_extract_status_accepted](../../../functions/src/arch/test_extract_status_accepted.md)
---
type: Rust Function
title: is_misconfig
resource: src/chain.rs#L103-L107
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/chain/has_any_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/scanner_is
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/keyword_fallback
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_misconfig(f: &CanonicalFinding) -> bool`

# Calls

- [has_any_tag](../../../functions/src/chain/has_any_tag.md)
- [scanner_is](../../../functions/src/chain/scanner_is.md)
- [keyword_fallback](../../../functions/src/chain/keyword_fallback.md)
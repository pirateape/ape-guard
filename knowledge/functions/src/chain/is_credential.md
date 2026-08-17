---
type: Rust Function
title: is_credential
resource: src/chain.rs#L76-L80
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/chain/has_any_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/cwe_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/keyword_fallback
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_credential(f: &CanonicalFinding) -> bool`

# Calls

- [has_any_tag](../../../functions/src/chain/has_any_tag.md)
- [cwe_prefix](../../../functions/src/chain/cwe_prefix.md)
- [keyword_fallback](../../../functions/src/chain/keyword_fallback.md)
---
type: Rust Function
title: has_any_tag
resource: src/chain.rs#L39-L41
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/is_credential
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/is_injection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/is_vulnerability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/is_misconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/is_xss
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn has_any_tag(tags: &'static [&'static str]) -> impl Fn(&CanonicalFinding) -> bool`

# Called by

- [is_credential](../../../functions/src/chain/is_credential.md)
- [is_injection](../../../functions/src/chain/is_injection.md)
- [is_vulnerability](../../../functions/src/chain/is_vulnerability.md)
- [is_misconfig](../../../functions/src/chain/is_misconfig.md)
- [is_xss](../../../functions/src/chain/is_xss.md)
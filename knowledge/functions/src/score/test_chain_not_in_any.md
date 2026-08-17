---
type: Rust Function
title: test_chain_not_in_any
resource: src/score.rs#L625-L630
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/dimension_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_chain_not_in_any()`

# Calls

- [dimension_chain](../../../functions/src/score/dimension_chain.md)
- [make_finding](../../../functions/src/score/make_finding.md)
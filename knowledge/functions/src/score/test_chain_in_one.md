---
type: Rust Function
title: test_chain_in_one
resource: src/score.rs#L633-L639
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

`fn test_chain_in_one()`

# Calls

- [dimension_chain](../../../functions/src/score/dimension_chain.md)
- [make_finding](../../../functions/src/score/make_finding.md)
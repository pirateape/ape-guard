---
type: Rust Function
title: test_confidence_confirmed_by_ai
resource: src/score.rs#L553-L569
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/dimension_confidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_confidence_confirmed_by_ai()`

# Calls

- [dimension_confidence](../../../functions/src/score/dimension_confidence.md)
- [make_finding](../../../functions/src/score/make_finding.md)
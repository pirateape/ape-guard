---
type: Rust Function
title: parse_reject_reason
resource: src/grade.rs#L210-L219
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/grade/parse_grade_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_reject_reason(reason: &Option<String>) -> RejectReason`

# Called by

- [parse_grade_response](../../../functions/src/grade/parse_grade_response.md)
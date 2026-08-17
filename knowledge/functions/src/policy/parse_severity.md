---
type: Rust Function
title: parse_severity
resource: src/policy.rs#L268-L277
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/policy/apply_actions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_severity(s: &str) -> Option<Severity>`

# Called by

- [apply_actions](../../../functions/src/policy/apply_actions.md)
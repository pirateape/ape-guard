---
type: Rust Function
title: test_evaluate_policies_disabled
resource: src/policy.rs#L818-L832
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/policy/evaluate_policies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/PolicyConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_evaluate_policies_disabled()`

# Calls

- [evaluate_policies](../../../functions/src/policy/evaluate_policies.md)
- [default](../../../functions/src/policy/PolicyConfig/default/default.md)
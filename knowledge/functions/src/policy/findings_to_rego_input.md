---
type: Rust Function
title: findings_to_rego_input
resource: src/policy.rs#L150-L186
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/policy/evaluate_policy_actions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_findings_to_rego_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn findings_to_rego_input(findings: &[CanonicalFinding]) -> serde_json::Value`

# Called by

- [evaluate_policy_actions](../../../functions/src/policy/evaluate_policy_actions.md)
- [test_findings_to_rego_input](../../../functions/src/policy/test_findings_to_rego_input.md)
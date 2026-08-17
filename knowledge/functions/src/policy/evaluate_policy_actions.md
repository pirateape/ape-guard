---
type: Rust Function
title: evaluate_policy_actions
resource: src/policy.rs#L190-L265
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/policy/findings_to_rego_input
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/policy/evaluate_policies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_empty_findings_no_actions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn evaluate_policy_actions( policies: &[(String, String)], findings: &[CanonicalFinding], ) -> Result<(Vec<PolicyActionEntry>, Vec<String>), String>`

# Calls

- [findings_to_rego_input](../../../functions/src/policy/findings_to_rego_input.md)

# Called by

- [evaluate_policies](../../../functions/src/policy/evaluate_policies.md)
- [test_empty_findings_no_actions](../../../functions/src/policy/test_empty_findings_no_actions.md)
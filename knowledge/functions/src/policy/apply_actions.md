---
type: Rust Function
title: apply_actions
resource: src/policy.rs#L281-L382
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/policy/parse_severity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/policy/evaluate_policies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_block_action
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_escalate_action
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_downgrade_action
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_tag_action
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_flag_action
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_escalate_only_if_higher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_apply_downgrade_only_if_lower
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_action_applied_but_finding_removed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_multiple_actions_same_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_unknown_finding_actions_logged
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_actions( findings: Vec<CanonicalFinding>, actions: &[PolicyActionEntry], ) -> ( Vec<CanonicalFinding>, Vec<PolicyActionEntry>, PolicyActionStats, )`

# Calls

- [parse_severity](../../../functions/src/policy/parse_severity.md)

# Called by

- [evaluate_policies](../../../functions/src/policy/evaluate_policies.md)
- [test_apply_block_action](../../../functions/src/policy/test_apply_block_action.md)
- [test_apply_escalate_action](../../../functions/src/policy/test_apply_escalate_action.md)
- [test_apply_downgrade_action](../../../functions/src/policy/test_apply_downgrade_action.md)
- [test_apply_tag_action](../../../functions/src/policy/test_apply_tag_action.md)
- [test_apply_flag_action](../../../functions/src/policy/test_apply_flag_action.md)
- [test_apply_escalate_only_if_higher](../../../functions/src/policy/test_apply_escalate_only_if_higher.md)
- [test_apply_downgrade_only_if_lower](../../../functions/src/policy/test_apply_downgrade_only_if_lower.md)
- [test_action_applied_but_finding_removed](../../../functions/src/policy/test_action_applied_but_finding_removed.md)
- [test_multiple_actions_same_finding](../../../functions/src/policy/test_multiple_actions_same_finding.md)
- [test_unknown_finding_actions_logged](../../../functions/src/policy/test_unknown_finding_actions_logged.md)
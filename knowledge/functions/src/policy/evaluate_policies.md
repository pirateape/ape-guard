---
type: Rust Function
title: evaluate_policies
resource: src/policy.rs#L402-L512
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/policy/load_policy_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/evaluate_policy_actions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/apply_actions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/PolicyActionStats/total_affected
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_evaluate_policies_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_evaluate_policies_no_policy_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn evaluate_policies( findings: Vec<CanonicalFinding>, config: &PolicyConfig, ) -> (Vec<CanonicalFinding>, PolicyResult)`

# Calls

- [load_policy_files](../../../functions/src/policy/load_policy_files.md)
- [evaluate_policy_actions](../../../functions/src/policy/evaluate_policy_actions.md)
- [apply_actions](../../../functions/src/policy/apply_actions.md)
- [total_affected](../../../functions/src/policy/PolicyActionStats/total_affected.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
- [test_evaluate_policies_disabled](../../../functions/src/policy/test_evaluate_policies_disabled.md)
- [test_evaluate_policies_no_policy_dir](../../../functions/src/policy/test_evaluate_policies_no_policy_dir.md)
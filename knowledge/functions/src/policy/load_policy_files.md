---
type: Rust Function
title: load_policy_files
resource: src/policy.rs#L108-L147
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/policy/evaluate_policies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_load_policy_files_nonexistent_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_policy_files(policy_dir: &Path) -> Vec<(String, String)>`

# Called by

- [evaluate_policies](../../../functions/src/policy/evaluate_policies.md)
- [test_load_policy_files_nonexistent_dir](../../../functions/src/policy/test_load_policy_files_nonexistent_dir.md)
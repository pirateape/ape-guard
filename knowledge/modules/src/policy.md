---
type: Rust Module
title: policy
resource: src/policy.rs#L1-L1021
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-scannertype
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [PolicyConfig](../../classes/src/policy/PolicyConfig.md)
- [default](../../functions/src/policy/PolicyConfig/default/default.md)
- [PolicyAction](../../classes/src/policy/PolicyAction.md)
- [PolicyActionEntry](../../classes/src/policy/PolicyActionEntry.md)
- [PolicyResult](../../classes/src/policy/PolicyResult.md)
- [load_policy_files](../../functions/src/policy/load_policy_files.md)
- [findings_to_rego_input](../../functions/src/policy/findings_to_rego_input.md)
- [evaluate_policy_actions](../../functions/src/policy/evaluate_policy_actions.md)
- [parse_severity](../../functions/src/policy/parse_severity.md)
- [apply_actions](../../functions/src/policy/apply_actions.md)
- [PolicyActionStats](../../classes/src/policy/PolicyActionStats.md)
- [total_affected](../../functions/src/policy/PolicyActionStats/total_affected.md)
- [evaluate_policies](../../functions/src/policy/evaluate_policies.md)
- [format_policy_summary](../../functions/src/policy/format_policy_summary.md)
- [format_policy_actions_table](../../functions/src/policy/format_policy_actions_table.md)
- [make_test_finding](../../functions/src/policy/make_test_finding.md)
- [test_load_policy_files_nonexistent_dir](../../functions/src/policy/test_load_policy_files_nonexistent_dir.md)
- [test_findings_to_rego_input](../../functions/src/policy/test_findings_to_rego_input.md)
- [test_parse_severity](../../functions/src/policy/test_parse_severity.md)
- [test_apply_block_action](../../functions/src/policy/test_apply_block_action.md)
- [test_apply_escalate_action](../../functions/src/policy/test_apply_escalate_action.md)
- [test_apply_downgrade_action](../../functions/src/policy/test_apply_downgrade_action.md)
- [test_apply_tag_action](../../functions/src/policy/test_apply_tag_action.md)
- [test_apply_flag_action](../../functions/src/policy/test_apply_flag_action.md)
- [test_apply_escalate_only_if_higher](../../functions/src/policy/test_apply_escalate_only_if_higher.md)
- [test_apply_downgrade_only_if_lower](../../functions/src/policy/test_apply_downgrade_only_if_lower.md)
- [test_evaluate_policies_disabled](../../functions/src/policy/test_evaluate_policies_disabled.md)
- [test_evaluate_policies_no_policy_dir](../../functions/src/policy/test_evaluate_policies_no_policy_dir.md)
- [test_format_policy_summary_disabled](../../functions/src/policy/test_format_policy_summary_disabled.md)
- [test_format_policy_summary_active](../../functions/src/policy/test_format_policy_summary_active.md)
- [test_format_policy_actions_table_empty](../../functions/src/policy/test_format_policy_actions_table_empty.md)
- [test_format_policy_actions_table_with_actions](../../functions/src/policy/test_format_policy_actions_table_with_actions.md)
- [test_policy_action_stats](../../functions/src/policy/test_policy_action_stats.md)
- [test_empty_findings_no_actions](../../functions/src/policy/test_empty_findings_no_actions.md)
- [test_regorus_serde_roundtrip](../../functions/src/policy/test_regorus_serde_roundtrip.md)
- [test_action_applied_but_finding_removed](../../functions/src/policy/test_action_applied_but_finding_removed.md)
- [test_multiple_actions_same_finding](../../functions/src/policy/test_multiple_actions_same_finding.md)
- [test_unknown_finding_actions_logged](../../functions/src/policy/test_unknown_finding_actions_logged.md)

# Imports

- `crate::find::{CanonicalFinding, Severity}`
- `serde::{Deserialize, Serialize}`
- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `super::*`
- `crate::find::ScannerType`

# Member of

- [apeguard](../../packages/apeguard.md)
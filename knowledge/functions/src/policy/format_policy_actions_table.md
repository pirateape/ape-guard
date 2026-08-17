---
type: Rust Function
title: format_policy_actions_table
resource: src/policy.rs#L545-L577
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/policy/test_format_policy_actions_table_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_format_policy_actions_table_with_actions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_html_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_policy_actions_table(actions: &[PolicyActionEntry]) -> String`

# Called by

- [test_format_policy_actions_table_empty](../../../functions/src/policy/test_format_policy_actions_table_empty.md)
- [test_format_policy_actions_table_with_actions](../../../functions/src/policy/test_format_policy_actions_table_with_actions.md)
- [generate_report](../../../functions/src/report/generate_report.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)
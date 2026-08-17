---
type: Rust Function
title: format_policy_summary
resource: src/policy.rs#L515-L542
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/policy/test_format_policy_summary_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/test_format_policy_summary_active
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

`pub fn format_policy_summary(result: &PolicyResult) -> String`

# Called by

- [test_format_policy_summary_disabled](../../../functions/src/policy/test_format_policy_summary_disabled.md)
- [test_format_policy_summary_active](../../../functions/src/policy/test_format_policy_summary_active.md)
- [generate_report](../../../functions/src/report/generate_report.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)
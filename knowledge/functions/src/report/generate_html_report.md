---
type: Rust Function
title: generate_html_report
resource: src/report/mod.rs#L720-L835
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/normalize/mitre_mapping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/format_stride_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/format_policy_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/format_policy_actions_table
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/generate_summary_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_html_report_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_html_report_with_arch_diagram
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_html_report_empty_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_html_report( summary: &ScanSummary, findings: &[CanonicalFinding], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path, arch_diagram: Option<&str>, stride_result: Option<&crate::stride::StrideResult>, policy_result: Option<&crate::policy::PolicyResult>, ) -> anyhow::Result<std::path::PathBuf>`

# Calls

- [mitre_mapping](../../../functions/src/normalize/mitre_mapping.md)
- [format_stride_table](../../../functions/src/stride/format_stride_table.md)
- [format_policy_summary](../../../functions/src/policy/format_policy_summary.md)
- [format_policy_actions_table](../../../functions/src/policy/format_policy_actions_table.md)

# Called by

- [run_report](../../../functions/src/run_report.md)
- [generate_summary_report](../../../functions/src/orchestrate/generate_summary_report.md)
- [test_generate_html_report_format](../../../functions/src/report/test_generate_html_report_format.md)
- [test_generate_html_report_with_arch_diagram](../../../functions/src/report/test_generate_html_report_with_arch_diagram.md)
- [test_generate_html_report_empty_findings](../../../functions/src/report/test_generate_html_report_empty_findings.md)
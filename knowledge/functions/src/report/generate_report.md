---
type: Rust Function
title: generate_report
resource: src/report/mod.rs#L77-L222
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/get_template
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/report/generate_all_reports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_report_technical_contains_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_report_executive_contains_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_report_roadmap_contains_remediation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_report_with_arch_diagram_appears
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_report( report_type: &ReportType, summary: &ScanSummary, findings: &[CanonicalFinding], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path, arch_diagram: Option<&str>, stride_result: Option<&crate::stride::StrideResult>, policy_result: Option<&crate::policy::PolicyResult>, ) -> anyhow::Result<std::path::PathBuf>`

# Calls

- [get_template](../../../functions/src/report/get_template.md)
- [mitre_mapping](../../../functions/src/normalize/mitre_mapping.md)
- [format_stride_table](../../../functions/src/stride/format_stride_table.md)
- [format_policy_summary](../../../functions/src/policy/format_policy_summary.md)
- [format_policy_actions_table](../../../functions/src/policy/format_policy_actions_table.md)

# Called by

- [generate_all_reports](../../../functions/src/report/generate_all_reports.md)
- [test_generate_report_technical_contains_findings](../../../functions/src/report/test_generate_report_technical_contains_findings.md)
- [test_generate_report_executive_contains_summary](../../../functions/src/report/test_generate_report_executive_contains_summary.md)
- [test_generate_report_roadmap_contains_remediation](../../../functions/src/report/test_generate_report_roadmap_contains_remediation.md)
- [test_generate_report_with_arch_diagram_appears](../../../functions/src/report/test_generate_report_with_arch_diagram_appears.md)
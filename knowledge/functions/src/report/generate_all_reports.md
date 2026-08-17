---
type: Rust Function
title: generate_all_reports
resource: src/report/mod.rs#L35-L73
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/generate_summary_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_all_reports_creates_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_selected_report_types
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_all_reports( summary: &ScanSummary, findings: &[CanonicalFinding], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path, arch_diagram: Option<&str>, report_types: &[ReportType], stride_result: Option<&crate::stride::StrideResult>, policy_result: Option<&crate::policy::PolicyResult>, ) -> anyhow::Result<Vec<std::path::PathBuf>>`

# Calls

- [generate_report](../../../functions/src/report/generate_report.md)

# Called by

- [run_report](../../../functions/src/run_report.md)
- [generate_summary_report](../../../functions/src/orchestrate/generate_summary_report.md)
- [test_generate_all_reports_creates_files](../../../functions/src/report/test_generate_all_reports_creates_files.md)
- [test_generate_selected_report_types](../../../functions/src/report/test_generate_selected_report_types.md)
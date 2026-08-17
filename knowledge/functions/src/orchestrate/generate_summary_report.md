---
type: Rust Function
title: generate_summary_report
resource: src/orchestrate.rs#L161-L254
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/generate_all_reports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_json_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_sarif_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_html_report
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_summary_report( findings: &[find::CanonicalFinding], target: &str, started_at: &str, duration_secs: f64, by_severity: &FindingsBySeverity, scanners_used: &[String], attack_chains: &[AttackChain], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path, report_types: &[cli::ReportType], formats: &[cli::OutputFormat], stride_result: Option<&stride::StrideResult>, policy_result: &policy::PolicyResult, ) -> anyhow::Result<Vec<std::path::PathBuf>>`

# Calls

- [generate_all_reports](../../../functions/src/report/generate_all_reports.md)
- [generate_json_report](../../../functions/src/report/generate_json_report.md)
- [generate_sarif_report](../../../functions/src/report/generate_sarif_report.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
---
type: Rust Function
title: generate_sarif_report
resource: src/report/mod.rs#L515-L717
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/generate_summary_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_sarif_report_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_sarif_report_with_arch_diagram
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_sarif_report( summary: &ScanSummary, findings: &[CanonicalFinding], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path, arch_diagram: Option<&str>, _stride_result: Option<&crate::stride::StrideResult>, _policy_result: Option<&crate::policy::PolicyResult>, ) -> anyhow::Result<std::path::PathBuf>`

# Called by

- [run_report](../../../functions/src/run_report.md)
- [generate_summary_report](../../../functions/src/orchestrate/generate_summary_report.md)
- [test_generate_sarif_report_format](../../../functions/src/report/test_generate_sarif_report_format.md)
- [test_generate_sarif_report_with_arch_diagram](../../../functions/src/report/test_generate_sarif_report_with_arch_diagram.md)
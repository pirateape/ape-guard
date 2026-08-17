---
type: Rust Function
title: run_report
resource: src/main.rs#L211-L405
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/get_latest_scan_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/get_scan_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/get_latest_scan_record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/discover_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/assess_component_risks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/generate_mermaid_diagram
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run_report( target: &str, snapshot: Option<&str>, output_dir: &str, cfg: &config::Config, quiet: bool, selected_reports: &[cli::ReportType], formats: &[cli::OutputFormat], ) -> anyhow::Result<()>`

# Calls

- [open](../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../functions/src/cache/ScanCache/enforce_ttl.md)
- [get_latest_scan_findings](../../functions/src/cache/ScanCache/get_latest_scan_findings.md)
- [get_scan_findings](../../functions/src/cache/ScanCache/get_scan_findings.md)
- [get_latest_scan_record](../../functions/src/cache/ScanCache/get_latest_scan_record.md)
- [compute_zt_scorecard](../../functions/src/normalize/compute_zt_scorecard.md)
- [build_attack_chains](../../functions/src/chain/build_attack_chains.md)
- [discover_artifacts](../../functions/src/arch/discover_artifacts.md)
- [assess_component_risks](../../functions/src/arch/assess_component_risks.md)
- [generate_mermaid_diagram](../../functions/src/arch/generate_mermaid_diagram.md)
- [generate_all_reports](../../functions/src/report/generate_all_reports.md)
- [generate_json_report](../../functions/src/report/generate_json_report.md)
- [generate_sarif_report](../../functions/src/report/generate_sarif_report.md)
- [generate_html_report](../../functions/src/report/generate_html_report.md)

# Called by

- [main](../../functions/src/main.md)
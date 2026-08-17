---
type: Rust Function
title: create_test_context
resource: src/report/mod.rs#L1109-L1212
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/report/test_generate_all_reports_creates_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_selected_report_types
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_json_report_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_json_report_with_arch_diagram
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_sarif_report_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_sarif_report_with_arch_diagram
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
  - target: functions/src/report/test_generate_html_report_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/test_generate_html_report_with_arch_diagram
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_test_context() -> (ScanSummary, ZeroTrustScorecard, Vec<CanonicalFinding>)`

# Called by

- [test_generate_all_reports_creates_files](../../../functions/src/report/test_generate_all_reports_creates_files.md)
- [test_generate_selected_report_types](../../../functions/src/report/test_generate_selected_report_types.md)
- [test_generate_json_report_format](../../../functions/src/report/test_generate_json_report_format.md)
- [test_generate_json_report_with_arch_diagram](../../../functions/src/report/test_generate_json_report_with_arch_diagram.md)
- [test_generate_sarif_report_format](../../../functions/src/report/test_generate_sarif_report_format.md)
- [test_generate_sarif_report_with_arch_diagram](../../../functions/src/report/test_generate_sarif_report_with_arch_diagram.md)
- [test_generate_report_technical_contains_findings](../../../functions/src/report/test_generate_report_technical_contains_findings.md)
- [test_generate_report_executive_contains_summary](../../../functions/src/report/test_generate_report_executive_contains_summary.md)
- [test_generate_report_roadmap_contains_remediation](../../../functions/src/report/test_generate_report_roadmap_contains_remediation.md)
- [test_generate_report_with_arch_diagram_appears](../../../functions/src/report/test_generate_report_with_arch_diagram_appears.md)
- [test_generate_html_report_format](../../../functions/src/report/test_generate_html_report_format.md)
- [test_generate_html_report_with_arch_diagram](../../../functions/src/report/test_generate_html_report_with_arch_diagram.md)
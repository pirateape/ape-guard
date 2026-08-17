---
type: Rust Function
title: test_generate_html_report_empty_findings
resource: src/report/mod.rs#L1560-L1599
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/empty_scorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_html_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_generate_html_report_empty_findings()`

# Calls

- [empty_scorecard](../../../functions/src/report/empty_scorecard.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)
---
type: Rust Function
title: format_stride_table
resource: src/stride.rs#L479-L513
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_html_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_format_stride_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_format_stride_table_with_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_stride_table(result: &StrideResult) -> String`

# Called by

- [generate_report](../../../functions/src/report/generate_report.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)
- [test_format_stride_table](../../../functions/src/stride/test_format_stride_table.md)
- [test_format_stride_table_with_findings](../../../functions/src/stride/test_format_stride_table_with_findings.md)
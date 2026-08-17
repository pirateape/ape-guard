---
type: Rust Function
title: test_generate_selected_report_types
resource: src/report/mod.rs#L1255-L1278
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/create_test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_all_reports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_generate_selected_report_types()`

# Calls

- [create_test_context](../../../functions/src/report/create_test_context.md)
- [generate_all_reports](../../../functions/src/report/generate_all_reports.md)
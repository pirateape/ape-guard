---
type: Rust Function
title: test_generate_json_report_with_arch_diagram
resource: src/report/mod.rs#L1309-L1330
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/create_test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_json_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_generate_json_report_with_arch_diagram()`

# Calls

- [create_test_context](../../../functions/src/report/create_test_context.md)
- [generate_json_report](../../../functions/src/report/generate_json_report.md)
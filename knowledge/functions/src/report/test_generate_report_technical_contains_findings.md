---
type: Rust Function
title: test_generate_report_technical_contains_findings
resource: src/report/mod.rs#L1385-L1408
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/report/create_test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_generate_report_technical_contains_findings()`

# Calls

- [create_test_context](../../../functions/src/report/create_test_context.md)
- [generate_report](../../../functions/src/report/generate_report.md)
---
type: Rust Function
title: test_format_stride_table_with_findings
resource: src/stride.rs#L1007-L1020
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/analyze_stride_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/format_stride_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_format_stride_table_with_findings()`

# Calls

- [analyze_stride_coverage](../../../functions/src/stride/analyze_stride_coverage.md)
- [format_stride_table](../../../functions/src/stride/format_stride_table.md)
- [make_finding](../../../functions/src/stride/make_finding.md)
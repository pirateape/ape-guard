---
type: Rust Function
title: test_single_finding_single_category
resource: src/stride.rs#L865-L879
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/analyze_stride_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_single_finding_single_category()`

# Calls

- [analyze_stride_coverage](../../../functions/src/stride/analyze_stride_coverage.md)
- [make_finding](../../../functions/src/stride/make_finding.md)
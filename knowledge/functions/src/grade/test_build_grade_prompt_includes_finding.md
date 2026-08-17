---
type: Rust Function
title: test_build_grade_prompt_includes_finding
resource: src/grade.rs#L290-L302
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/grade/build_grade_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_build_grade_prompt_includes_finding()`

# Calls

- [build_grade_prompt](../../../functions/src/grade/build_grade_prompt.md)
- [make_finding](../../../functions/src/grade/make_finding.md)
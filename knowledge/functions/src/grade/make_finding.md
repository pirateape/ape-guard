---
type: Rust Function
title: make_finding
resource: src/grade.rs#L257-L287
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/grade/test_build_grade_prompt_includes_finding
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/grade/test_grade_field_on_canonical_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn make_finding(id: &str) -> CanonicalFinding`

# Called by

- [test_build_grade_prompt_includes_finding](../../../functions/src/grade/test_build_grade_prompt_includes_finding.md)
- [test_grade_field_on_canonical_finding](../../../functions/src/grade/test_grade_field_on_canonical_finding.md)
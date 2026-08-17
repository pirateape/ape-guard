---
type: Rust Function
title: dimension_grade
resource: src/score.rs#L257-L265
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_grade_not_graded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_grade_confirmed_high_confidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_grade_confirmed_low_confidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_grade_needs_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_grade_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dimension_grade(grade: &Option<GradeVerdict>) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [test_grade_not_graded](../../../functions/src/score/test_grade_not_graded.md)
- [test_grade_confirmed_high_confidence](../../../functions/src/score/test_grade_confirmed_high_confidence.md)
- [test_grade_confirmed_low_confidence](../../../functions/src/score/test_grade_confirmed_low_confidence.md)
- [test_grade_needs_review](../../../functions/src/score/test_grade_needs_review.md)
- [test_grade_rejected](../../../functions/src/score/test_grade_rejected.md)
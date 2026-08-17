---
type: Rust Function
title: dimension_zt_pillars
resource: src/score.rs#L246-L253
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_zt_zero_pillars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_zt_one_pillar
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_zt_two_pillars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_zt_three_pillars
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dimension_zt_pillars(finding: &CanonicalFinding) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [test_zt_zero_pillars](../../../functions/src/score/test_zt_zero_pillars.md)
- [test_zt_one_pillar](../../../functions/src/score/test_zt_one_pillar.md)
- [test_zt_two_pillars](../../../functions/src/score/test_zt_two_pillars.md)
- [test_zt_three_pillars](../../../functions/src/score/test_zt_three_pillars.md)
---
type: Rust Function
title: compute_score_confidence
resource: src/score.rs#L269-L295
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn compute_score_confidence(finding: &CanonicalFinding) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
---
type: Rust Function
title: dimension_severity
resource: src/score.rs#L120-L128
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

`fn dimension_severity(severity: &Severity) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
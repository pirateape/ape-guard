---
type: Rust Function
title: make_finding
resource: src/filter.rs#L276-L312
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/filter/test_confidence_threshold
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn make_finding( id: &str, severity: Severity, file: &str, cross_refs: Vec<crate::find::CrossReference>, grade: Option<GradeVerdict>, ) -> CanonicalFinding`

# Called by

- [test_confidence_threshold](../../../functions/src/filter/test_confidence_threshold.md)
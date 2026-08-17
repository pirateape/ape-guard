---
type: Rust Function
title: count_by_sev
resource: src/main.rs#L469-L487
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/run_compare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_by_sev(findings: &[&find::CanonicalFinding]) -> find::FindingsBySeverity`

# Called by

- [run_compare](../../functions/src/run_compare.md)
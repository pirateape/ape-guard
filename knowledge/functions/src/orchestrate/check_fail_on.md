---
type: Rust Function
title: check_fail_on
resource: src/orchestrate.rs#L147-L157
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_fail_on(findings: &[find::CanonicalFinding], threshold: &cli::FailOnThreshold) -> bool`

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
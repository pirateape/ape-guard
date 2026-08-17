---
type: Rust Function
title: filter_by_severity
resource: src/orchestrate.rs#L83-L116
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn filter_by_severity( findings: Vec<find::CanonicalFinding>, filter: &cli::SeverityFilter, ) -> Vec<find::CanonicalFinding>`

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
---
type: Rust Function
title: load_completed_scanners
resource: src/orchestrate.rs#L119-L144
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn load_completed_scanners( path: &std::path::Path, ) -> anyhow::Result<std::collections::HashSet<String>>`

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
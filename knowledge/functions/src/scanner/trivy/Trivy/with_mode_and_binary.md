---
type: Rust Method
title: with_mode_and_binary
resource: src/scanner/trivy.rs#L37-L42
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_mode_and_binary(mode: TrivyMode, path: Option<String>) -> Self`

# Called by

- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
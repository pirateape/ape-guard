---
type: Rust Function
title: count_by_severity
resource: src/scanner/context_drift/mod.rs#L174-L186
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_by_severity(findings: &[DriftFinding]) -> DriftCounts`

# Called by

- [scan_drift](../../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
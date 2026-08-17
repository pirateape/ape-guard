---
type: Rust Method
title: get_scan_findings
resource: src/cache.rs#L81-L102
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_compare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_scan_findings( &self, scan_id: &str, ) -> anyhow::Result<Option<Vec<CanonicalFinding>>>`

# Called by

- [run_report](../../../../functions/src/run_report.md)
- [run_compare](../../../../functions/src/run_compare.md)
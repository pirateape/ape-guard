---
type: Rust Method
title: get_latest_scan_record
resource: src/cache.rs#L130-L154
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_latest_scan_record(&self) -> anyhow::Result<Option<ScanRecord>>`

# Called by

- [run_report](../../../../functions/src/run_report.md)
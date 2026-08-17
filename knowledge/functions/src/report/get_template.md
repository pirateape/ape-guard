---
type: Rust Function
title: get_template
resource: src/report/mod.rs#L242-L248
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn get_template(rtype: &ReportType) -> &'static str`

# Called by

- [generate_report](../../../functions/src/report/generate_report.md)
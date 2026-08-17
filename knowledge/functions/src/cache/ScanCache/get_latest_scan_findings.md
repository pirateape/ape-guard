---
type: Rust Method
title: get_latest_scan_findings
resource: src/cache.rs#L105-L127
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_latest_scan_findings( &self, ) -> anyhow::Result<Option<(String, Vec<CanonicalFinding>)>>`

# Called by

- [run_report](../../../../functions/src/run_report.md)
- [load_cached_findings](../../../../functions/src/mcp/load_cached_findings.md)
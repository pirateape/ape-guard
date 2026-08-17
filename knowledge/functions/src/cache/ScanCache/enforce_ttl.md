---
type: Rust Method
title: enforce_ttl
resource: src/cache.rs#L232-L254
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/cache/test_enforce_ttl_prunes_old_scan_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_compare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/handle_cache
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn enforce_ttl(&self, ttl_hours: u32) -> anyhow::Result<u64>`

# Called by

- [test_enforce_ttl_prunes_old_scan_history](../../../../functions/src/cache/test_enforce_ttl_prunes_old_scan_history.md)
- [run_report](../../../../functions/src/run_report.md)
- [run_compare](../../../../functions/src/run_compare.md)
- [handle_cache](../../../../functions/src/handle_cache.md)
- [load_cached_findings](../../../../functions/src/mcp/load_cached_findings.md)
- [handle_scan_tool](../../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../../functions/src/orchestrate/run_scan.md)
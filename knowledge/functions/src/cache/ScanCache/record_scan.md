---
type: Rust Method
title: record_scan
resource: src/cache.rs#L257-L280
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/cache/test_record_scan_persists_timestamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn record_scan(&self, input: RecordScanInput<'_>) -> anyhow::Result<()>`

# Called by

- [test_record_scan_persists_timestamps](../../../../functions/src/cache/test_record_scan_persists_timestamps.md)
- [main](../../../../functions/src/main.md)
- [handle_scan_tool](../../../../functions/src/mcp/handle_scan_tool.md)
---
type: Rust Function
title: handle_resource_read
resource: src/mcp.rs#L708-L832
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/load_cached_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/summarize_findings_by_severity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/handle_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_resource_read_missing_uri
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/test_resource_read_unknown_uri
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_resource_read(id: &Value, params: &Value) -> Value`

# Calls

- [load_cached_findings](../../../functions/src/mcp/load_cached_findings.md)
- [summarize_findings_by_severity](../../../functions/src/mcp/summarize_findings_by_severity.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)

# Called by

- [handle_request](../../../functions/src/mcp/handle_request.md)
- [test_resource_read_missing_uri](../../../functions/src/mcp/test_resource_read_missing_uri.md)
- [test_resource_read_unknown_uri](../../../functions/src/mcp/test_resource_read_unknown_uri.md)
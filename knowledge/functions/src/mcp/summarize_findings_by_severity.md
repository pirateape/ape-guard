---
type: Rust Function
title: summarize_findings_by_severity
resource: src/mcp.rs#L24-L44
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_resource_read
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn summarize_findings_by_severity(findings: &[CanonicalFinding]) -> FindingsBySeverity`

# Called by

- [handle_resource_read](../../../functions/src/mcp/handle_resource_read.md)
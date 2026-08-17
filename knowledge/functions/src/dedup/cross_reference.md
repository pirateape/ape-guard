---
type: Rust Function
title: cross_reference
resource: src/dedup.rs#L40-L78
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/dedup/test_cross_reference_links_related
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_cross_reference_three_scanners
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_cross_reference_no_self_ref
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_cross_reference_different_location_no_link
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

`pub fn cross_reference(findings: &mut [CanonicalFinding])`

# Called by

- [test_cross_reference_links_related](../../../functions/src/dedup/test_cross_reference_links_related.md)
- [test_cross_reference_three_scanners](../../../functions/src/dedup/test_cross_reference_three_scanners.md)
- [test_cross_reference_no_self_ref](../../../functions/src/dedup/test_cross_reference_no_self_ref.md)
- [test_cross_reference_different_location_no_link](../../../functions/src/dedup/test_cross_reference_different_location_no_link.md)
- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
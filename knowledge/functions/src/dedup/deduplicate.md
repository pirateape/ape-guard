---
type: Rust Function
title: deduplicate
resource: src/dedup.rs#L10-L35
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/dedup/test_dedup_removes_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_dedup_different_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_dedup_same_file_different_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_dedup_no_line_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/test_empty_findings
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

`pub fn deduplicate(findings: Vec<CanonicalFinding>) -> Vec<CanonicalFinding>`

# Called by

- [test_dedup_removes_duplicates](../../../functions/src/dedup/test_dedup_removes_duplicates.md)
- [test_dedup_different_files](../../../functions/src/dedup/test_dedup_different_files.md)
- [test_dedup_same_file_different_lines](../../../functions/src/dedup/test_dedup_same_file_different_lines.md)
- [test_dedup_no_line_number](../../../functions/src/dedup/test_dedup_no_line_number.md)
- [test_empty_findings](../../../functions/src/dedup/test_empty_findings.md)
- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
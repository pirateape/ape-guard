---
type: Rust Function
title: analyze_stride_coverage
resource: src/stride.rs#L413-L472
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/StrideCategory/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/map_finding_to_stride
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_empty_findings_no_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_single_finding_single_category
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_mixed_findings_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_all_categories_covered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_threshold_filtering
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_coverage_ratio_calculation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_format_stride_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_format_stride_table_with_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn analyze_stride_coverage(findings: &[CanonicalFinding], threshold: f64) -> StrideResult`

# Calls

- [all](../../../functions/src/stride/StrideCategory/all.md)
- [map_finding_to_stride](../../../functions/src/stride/map_finding_to_stride.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
- [test_empty_findings_no_coverage](../../../functions/src/stride/test_empty_findings_no_coverage.md)
- [test_single_finding_single_category](../../../functions/src/stride/test_single_finding_single_category.md)
- [test_mixed_findings_coverage](../../../functions/src/stride/test_mixed_findings_coverage.md)
- [test_all_categories_covered](../../../functions/src/stride/test_all_categories_covered.md)
- [test_threshold_filtering](../../../functions/src/stride/test_threshold_filtering.md)
- [test_coverage_ratio_calculation](../../../functions/src/stride/test_coverage_ratio_calculation.md)
- [test_format_stride_table](../../../functions/src/stride/test_format_stride_table.md)
- [test_format_stride_table_with_findings](../../../functions/src/stride/test_format_stride_table_with_findings.md)
---
type: Rust Function
title: apply_fp_filters
resource: src/filter.rs#L105-L210
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/filter/partition_by
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/is_excluded_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/is_test_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/filter/test_excludes_vendor_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_excludes_test_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_excludes_examples
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_custom_exclude_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_suppress_low_severity_in_test_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_cross_scanner_requirement_for_low
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_grade_rejected_filtered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_confidence_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_severity_floor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_no_filtering_when_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_empty_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_stats_total_removed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn apply_fp_filters( findings: Vec<CanonicalFinding>, config: &FilterConfig, ) -> (Vec<CanonicalFinding>, FilterStats)`

# Calls

- [partition_by](../../../functions/src/filter/partition_by.md)
- [is_excluded_path](../../../functions/src/filter/is_excluded_path.md)
- [is_test_file](../../../functions/src/filter/is_test_file.md)

# Called by

- [test_excludes_vendor_paths](../../../functions/src/filter/test_excludes_vendor_paths.md)
- [test_excludes_test_files](../../../functions/src/filter/test_excludes_test_files.md)
- [test_excludes_examples](../../../functions/src/filter/test_excludes_examples.md)
- [test_custom_exclude_paths](../../../functions/src/filter/test_custom_exclude_paths.md)
- [test_suppress_low_severity_in_test_files](../../../functions/src/filter/test_suppress_low_severity_in_test_files.md)
- [test_cross_scanner_requirement_for_low](../../../functions/src/filter/test_cross_scanner_requirement_for_low.md)
- [test_grade_rejected_filtered](../../../functions/src/filter/test_grade_rejected_filtered.md)
- [test_confidence_threshold](../../../functions/src/filter/test_confidence_threshold.md)
- [test_severity_floor](../../../functions/src/filter/test_severity_floor.md)
- [test_no_filtering_when_disabled](../../../functions/src/filter/test_no_filtering_when_disabled.md)
- [test_empty_findings](../../../functions/src/filter/test_empty_findings.md)
- [test_stats_total_removed](../../../functions/src/filter/test_stats_total_removed.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
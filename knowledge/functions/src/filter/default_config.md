---
type: Rust Function
title: default_config
resource: src/filter.rs#L314-L326
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
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
  - target: functions/src/filter/test_empty_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/test_stats_total_removed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_config() -> FilterConfig`

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
- [test_empty_findings](../../../functions/src/filter/test_empty_findings.md)
- [test_stats_total_removed](../../../functions/src/filter/test_stats_total_removed.md)
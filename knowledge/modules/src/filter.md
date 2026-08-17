---
type: Rust Module
title: filter
resource: src/filter.rs#L1-L562
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-config-filterconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-gradeverdict-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FilterStats](../../classes/src/filter/FilterStats.md)
- [total_removed](../../functions/src/filter/FilterStats/total_removed.md)
- [apply_fp_filters](../../functions/src/filter/apply_fp_filters.md)
- [partition_by](../../functions/src/filter/partition_by.md)
- [is_excluded_path](../../functions/src/filter/is_excluded_path.md)
- [is_test_file](../../functions/src/filter/is_test_file.md)
- [make_finding](../../functions/src/filter/make_finding.md)
- [default_config](../../functions/src/filter/default_config.md)
- [test_excludes_vendor_paths](../../functions/src/filter/test_excludes_vendor_paths.md)
- [test_excludes_test_files](../../functions/src/filter/test_excludes_test_files.md)
- [test_excludes_examples](../../functions/src/filter/test_excludes_examples.md)
- [test_custom_exclude_paths](../../functions/src/filter/test_custom_exclude_paths.md)
- [test_suppress_low_severity_in_test_files](../../functions/src/filter/test_suppress_low_severity_in_test_files.md)
- [test_cross_scanner_requirement_for_low](../../functions/src/filter/test_cross_scanner_requirement_for_low.md)
- [test_grade_rejected_filtered](../../functions/src/filter/test_grade_rejected_filtered.md)
- [test_confidence_threshold](../../functions/src/filter/test_confidence_threshold.md)
- [test_severity_floor](../../functions/src/filter/test_severity_floor.md)
- [test_no_filtering_when_disabled](../../functions/src/filter/test_no_filtering_when_disabled.md)
- [test_empty_findings](../../functions/src/filter/test_empty_findings.md)
- [test_stats_total_removed](../../functions/src/filter/test_stats_total_removed.md)

# Imports

- `crate::config::FilterConfig`
- `crate::find::{CanonicalFinding, Confidence, GradeVerdict, Severity}`
- `std::path::Path`
- `super::*`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
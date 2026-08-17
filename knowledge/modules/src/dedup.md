---
type: Rust Module
title: dedup
resource: src/dedup.rs#L1-L238
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [deduplicate](../../functions/src/dedup/deduplicate.md)
- [cross_reference](../../functions/src/dedup/cross_reference.md)
- [make_finding](../../functions/src/dedup/make_finding.md)
- [test_dedup_removes_duplicates](../../functions/src/dedup/test_dedup_removes_duplicates.md)
- [test_dedup_different_files](../../functions/src/dedup/test_dedup_different_files.md)
- [test_dedup_same_file_different_lines](../../functions/src/dedup/test_dedup_same_file_different_lines.md)
- [test_dedup_no_line_number](../../functions/src/dedup/test_dedup_no_line_number.md)
- [test_empty_findings](../../functions/src/dedup/test_empty_findings.md)
- [test_cross_reference_links_related](../../functions/src/dedup/test_cross_reference_links_related.md)
- [test_cross_reference_three_scanners](../../functions/src/dedup/test_cross_reference_three_scanners.md)
- [test_cross_reference_no_self_ref](../../functions/src/dedup/test_cross_reference_no_self_ref.md)
- [test_cross_reference_different_location_no_link](../../functions/src/dedup/test_cross_reference_different_location_no_link.md)

# Imports

- `crate::find::CanonicalFinding`
- `std::collections::HashSet`
- `std::collections::HashMap`
- `super::*`
- `crate::find::*`

# Member of

- [apeguard](../../packages/apeguard.md)
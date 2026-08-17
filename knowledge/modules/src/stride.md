---
type: Rust Module
title: stride
resource: src/stride.rs#L1-L1032
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
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

- [StrideCategory](../../classes/src/stride/StrideCategory.md)
- [label](../../functions/src/stride/StrideCategory/label.md)
- [id](../../functions/src/stride/StrideCategory/id.md)
- [description](../../functions/src/stride/StrideCategory/description.md)
- [all](../../functions/src/stride/StrideCategory/all.md)
- [StrideCoverage](../../classes/src/stride/StrideCoverage.md)
- [StrideResult](../../classes/src/stride/StrideResult.md)
- [StrideConfig](../../classes/src/stride/StrideConfig.md)
- [default](../../functions/src/stride/StrideConfig/default/default.md)
- [map_finding_to_stride](../../functions/src/stride/map_finding_to_stride.md)
- [analyze_stride_coverage](../../functions/src/stride/analyze_stride_coverage.md)
- [format_stride_table](../../functions/src/stride/format_stride_table.md)
- [make_finding](../../functions/src/stride/make_finding.md)
- [test_category_labels](../../functions/src/stride/test_category_labels.md)
- [test_category_ids](../../functions/src/stride/test_category_ids.md)
- [test_all_categories](../../functions/src/stride/test_all_categories.md)
- [test_map_secret_to_spoofing_and_disclosure](../../functions/src/stride/test_map_secret_to_spoofing_and_disclosure.md)
- [test_map_sqli_to_tampering](../../functions/src/stride/test_map_sqli_to_tampering.md)
- [test_map_xss_to_tampering](../../functions/src/stride/test_map_xss_to_tampering.md)
- [test_map_rce_to_elevation_of_privilege](../../functions/src/stride/test_map_rce_to_elevation_of_privilege.md)
- [test_map_ssrf_to_information_disclosure](../../functions/src/stride/test_map_ssrf_to_information_disclosure.md)
- [test_map_idor_to_information_disclosure](../../functions/src/stride/test_map_idor_to_information_disclosure.md)
- [test_map_jwt_to_spoofing](../../functions/src/stride/test_map_jwt_to_spoofing.md)
- [test_map_xxe_to_tampering](../../functions/src/stride/test_map_xxe_to_tampering.md)
- [test_map_prototype_pollution_to_tampering](../../functions/src/stride/test_map_prototype_pollution_to_tampering.md)
- [test_map_rate_limit_to_dos](../../functions/src/stride/test_map_rate_limit_to_dos.md)
- [test_map_privilege_escalation](../../functions/src/stride/test_map_privilege_escalation.md)
- [test_map_misconfig_to_disclosure](../../functions/src/stride/test_map_misconfig_to_disclosure.md)
- [test_map_audit_logging_to_repudiation](../../functions/src/stride/test_map_audit_logging_to_repudiation.md)
- [test_map_csrf_to_spoofing](../../functions/src/stride/test_map_csrf_to_spoofing.md)
- [test_map_container_escape_to_eop](../../functions/src/stride/test_map_container_escape_to_eop.md)
- [test_map_injection_to_multiple_categories](../../functions/src/stride/test_map_injection_to_multiple_categories.md)
- [test_map_deserialization_to_tampering_and_eop](../../functions/src/stride/test_map_deserialization_to_tampering_and_eop.md)
- [test_map_no_keywords_returns_empty](../../functions/src/stride/test_map_no_keywords_returns_empty.md)
- [test_empty_findings_no_coverage](../../functions/src/stride/test_empty_findings_no_coverage.md)
- [test_single_finding_single_category](../../functions/src/stride/test_single_finding_single_category.md)
- [test_mixed_findings_coverage](../../functions/src/stride/test_mixed_findings_coverage.md)
- [test_all_categories_covered](../../functions/src/stride/test_all_categories_covered.md)
- [test_threshold_filtering](../../functions/src/stride/test_threshold_filtering.md)
- [test_coverage_ratio_calculation](../../functions/src/stride/test_coverage_ratio_calculation.md)
- [test_format_stride_table](../../functions/src/stride/test_format_stride_table.md)
- [test_format_stride_table_with_findings](../../functions/src/stride/test_format_stride_table_with_findings.md)
- [test_stride_config_default](../../functions/src/stride/test_stride_config_default.md)

# Imports

- `crate::find::CanonicalFinding`
- `serde::{Deserialize, Serialize}`
- `super::*`
- `crate::find::*`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
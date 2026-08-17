---
type: Rust Function
title: map_finding_to_stride
resource: src/stride.rs#L172-L397
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stride/analyze_stride_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_secret_to_spoofing_and_disclosure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_sqli_to_tampering
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_xss_to_tampering
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_rce_to_elevation_of_privilege
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_ssrf_to_information_disclosure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_idor_to_information_disclosure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_jwt_to_spoofing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_xxe_to_tampering
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_prototype_pollution_to_tampering
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_rate_limit_to_dos
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_privilege_escalation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_misconfig_to_disclosure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_audit_logging_to_repudiation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_csrf_to_spoofing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_container_escape_to_eop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_injection_to_multiple_categories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_deserialization_to_tampering_and_eop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/test_map_no_keywords_returns_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn map_finding_to_stride(finding: &CanonicalFinding) -> Vec<StrideCategory>`

# Called by

- [analyze_stride_coverage](../../../functions/src/stride/analyze_stride_coverage.md)
- [test_map_secret_to_spoofing_and_disclosure](../../../functions/src/stride/test_map_secret_to_spoofing_and_disclosure.md)
- [test_map_sqli_to_tampering](../../../functions/src/stride/test_map_sqli_to_tampering.md)
- [test_map_xss_to_tampering](../../../functions/src/stride/test_map_xss_to_tampering.md)
- [test_map_rce_to_elevation_of_privilege](../../../functions/src/stride/test_map_rce_to_elevation_of_privilege.md)
- [test_map_ssrf_to_information_disclosure](../../../functions/src/stride/test_map_ssrf_to_information_disclosure.md)
- [test_map_idor_to_information_disclosure](../../../functions/src/stride/test_map_idor_to_information_disclosure.md)
- [test_map_jwt_to_spoofing](../../../functions/src/stride/test_map_jwt_to_spoofing.md)
- [test_map_xxe_to_tampering](../../../functions/src/stride/test_map_xxe_to_tampering.md)
- [test_map_prototype_pollution_to_tampering](../../../functions/src/stride/test_map_prototype_pollution_to_tampering.md)
- [test_map_rate_limit_to_dos](../../../functions/src/stride/test_map_rate_limit_to_dos.md)
- [test_map_privilege_escalation](../../../functions/src/stride/test_map_privilege_escalation.md)
- [test_map_misconfig_to_disclosure](../../../functions/src/stride/test_map_misconfig_to_disclosure.md)
- [test_map_audit_logging_to_repudiation](../../../functions/src/stride/test_map_audit_logging_to_repudiation.md)
- [test_map_csrf_to_spoofing](../../../functions/src/stride/test_map_csrf_to_spoofing.md)
- [test_map_container_escape_to_eop](../../../functions/src/stride/test_map_container_escape_to_eop.md)
- [test_map_injection_to_multiple_categories](../../../functions/src/stride/test_map_injection_to_multiple_categories.md)
- [test_map_deserialization_to_tampering_and_eop](../../../functions/src/stride/test_map_deserialization_to_tampering_and_eop.md)
- [test_map_no_keywords_returns_empty](../../../functions/src/stride/test_map_no_keywords_returns_empty.md)
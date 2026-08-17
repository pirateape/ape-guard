---
type: Rust Function
title: test_map_ssrf_to_information_disclosure
resource: src/stride.rs#L668-L678
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/map_finding_to_stride
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_map_ssrf_to_information_disclosure()`

# Calls

- [map_finding_to_stride](../../../functions/src/stride/map_finding_to_stride.md)
- [make_finding](../../../functions/src/stride/make_finding.md)
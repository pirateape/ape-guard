---
type: Rust Function
title: test_map_rce_to_elevation_of_privilege
resource: src/stride.rs#L654-L665
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

`fn test_map_rce_to_elevation_of_privilege()`

# Calls

- [map_finding_to_stride](../../../functions/src/stride/map_finding_to_stride.md)
- [make_finding](../../../functions/src/stride/make_finding.md)
---
type: Rust Function
title: test_map_jwt_to_spoofing
resource: src/stride.rs#L694-L704
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

`fn test_map_jwt_to_spoofing()`

# Calls

- [map_finding_to_stride](../../../functions/src/stride/map_finding_to_stride.md)
- [make_finding](../../../functions/src/stride/make_finding.md)
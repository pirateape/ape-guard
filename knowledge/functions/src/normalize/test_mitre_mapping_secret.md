---
type: Rust Function
title: test_mitre_mapping_secret
resource: src/normalize.rs#L630-L634
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/normalize/mitre_mapping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_mitre_mapping_secret()`

# Calls

- [mitre_mapping](../../../functions/src/normalize/mitre_mapping.md)
- [make_finding](../../../functions/src/normalize/make_finding.md)
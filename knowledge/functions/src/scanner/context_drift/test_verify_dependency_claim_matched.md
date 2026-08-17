---
type: Rust Function
title: test_verify_dependency_claim_matched
resource: src/scanner/context_drift/mod.rs#L391-L414
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/verify_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/new
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_verify_dependency_claim_matched()`

# Calls

- [verify_dependency_claim](../../../../functions/src/scanner/context_drift/verify/verify_dependency_claim.md)
- [new](../../../../functions/src/scanner/context_drift/verify/DependencyCache/new.md)
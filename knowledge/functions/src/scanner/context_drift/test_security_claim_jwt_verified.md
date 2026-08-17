---
type: Rust Function
title: test_security_claim_jwt_verified
resource: src/scanner/context_drift/mod.rs#L490-L512
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/new
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_security_claim_jwt_verified()`

# Calls

- [verify_single_claim](../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
- [new](../../../../functions/src/scanner/context_drift/verify/DependencyCache/new.md)
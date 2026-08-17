---
type: Rust Function
title: verify_semantic_claim
resource: src/scanner/context_drift/verify.rs#L631-L651
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verify_semantic_claim(claim: &ContextClaim, _root: &Path) -> VerificationResult`

# Called by

- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
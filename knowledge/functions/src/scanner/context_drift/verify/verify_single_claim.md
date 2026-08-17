---
type: Rust Function
title: verify_single_claim
resource: src/scanner/context_drift/verify.rs#L51-L65
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/verify_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_path_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_architecture_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_convention_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_security_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_command_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_semantic_claim
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_security_claim_jwt_verified
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_claims
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn verify_single_claim( claim: &ContextClaim, root: &Path, dep_cache: &DependencyCache, ) -> VerificationResult`

# Calls

- [verify_dependency_claim](../../../../../functions/src/scanner/context_drift/verify/verify_dependency_claim.md)
- [verify_path_claim](../../../../../functions/src/scanner/context_drift/verify/verify_path_claim.md)
- [verify_architecture_claim](../../../../../functions/src/scanner/context_drift/verify/verify_architecture_claim.md)
- [verify_convention_claim](../../../../../functions/src/scanner/context_drift/verify/verify_convention_claim.md)
- [verify_security_claim](../../../../../functions/src/scanner/context_drift/verify/verify_security_claim.md)
- [verify_command_claim](../../../../../functions/src/scanner/context_drift/verify/verify_command_claim.md)
- [verify_semantic_claim](../../../../../functions/src/scanner/context_drift/verify/verify_semantic_claim.md)

# Called by

- [test_security_claim_jwt_verified](../../../../../functions/src/scanner/context_drift/test_security_claim_jwt_verified.md)
- [verify_claims](../../../../../functions/src/scanner/context_drift/verify/verify_claims.md)
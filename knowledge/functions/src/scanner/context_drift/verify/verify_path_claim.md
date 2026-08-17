---
type: Rust Function
title: verify_path_claim
resource: src/scanner/context_drift/verify.rs#L284-L328
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/path_pattern
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_verify_path_claim_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_verify_path_claim_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn verify_path_claim(claim: &ContextClaim, root: &Path) -> VerificationResult`

# Calls

- [path_pattern](../../../../../functions/src/scanner/context_drift/discover/path_pattern.md)

# Called by

- [test_verify_path_claim_exists](../../../../../functions/src/scanner/context_drift/test_verify_path_claim_exists.md)
- [test_verify_path_claim_missing](../../../../../functions/src/scanner/context_drift/test_verify_path_claim_missing.md)
- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
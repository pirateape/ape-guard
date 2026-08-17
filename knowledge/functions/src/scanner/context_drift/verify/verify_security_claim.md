---
type: Rust Function
title: verify_security_claim
resource: src/scanner/context_drift/verify.rs#L528-L593
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/read_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verify_security_claim(claim: &ContextClaim, root: &Path) -> VerificationResult`

# Calls

- [read_file](../../../../../functions/src/scanner/context_drift/discover/read_file.md)

# Called by

- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
---
type: Rust Function
title: drift_severity
resource: src/scanner/context_drift/verify.rs#L34-L48
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/types/ClaimCategory/default_severity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_claims
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn drift_severity(category: &ClaimCategory, result: &VerificationResult) -> Severity`

# Calls

- [default_severity](../../../../../functions/src/scanner/context_drift/types/ClaimCategory/default_severity.md)

# Called by

- [verify_claims](../../../../../functions/src/scanner/context_drift/verify/verify_claims.md)
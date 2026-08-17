---
type: Rust Function
title: verify_claims
resource: src/scanner/context_drift/verify.rs#L10-L31
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/drift_severity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/new
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn verify_claims(claims: &[ContextClaim], root: &Path) -> Vec<DriftFinding>`

# Calls

- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
- [drift_severity](../../../../../functions/src/scanner/context_drift/verify/drift_severity.md)
- [new](../../../../../functions/src/scanner/context_drift/verify/DependencyCache/new.md)

# Called by

- [scan_drift](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
---
type: Rust Function
title: verify_dependency_claim
resource: src/scanner/context_drift/verify.rs#L198-L280
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/extract_dep_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/has_dependency
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_verify_dependency_claim_matched
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_verify_dependency_claim_drifted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn verify_dependency_claim( claim: &ContextClaim, _root: &Path, dep_cache: &DependencyCache, ) -> VerificationResult`

# Calls

- [extract_dep_name](../../../../../functions/src/scanner/context_drift/verify/extract_dep_name.md)
- [has_dependency](../../../../../functions/src/scanner/context_drift/verify/DependencyCache/has_dependency.md)

# Called by

- [test_verify_dependency_claim_matched](../../../../../functions/src/scanner/context_drift/test_verify_dependency_claim_matched.md)
- [test_verify_dependency_claim_drifted](../../../../../functions/src/scanner/context_drift/test_verify_dependency_claim_drifted.md)
- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
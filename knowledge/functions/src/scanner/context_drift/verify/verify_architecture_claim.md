---
type: Rust Function
title: verify_architecture_claim
resource: src/scanner/context_drift/verify.rs#L332-L386
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/technology_keyword_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/search_for_technology_usage
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_single_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verify_architecture_claim(claim: &ContextClaim, root: &Path) -> VerificationResult`

# Calls

- [technology_keyword_pattern](../../../../../functions/src/scanner/context_drift/discover/technology_keyword_pattern.md)
- [search_for_technology_usage](../../../../../functions/src/scanner/context_drift/verify/search_for_technology_usage.md)

# Called by

- [verify_single_claim](../../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
---
type: Rust Method
title: new
resource: src/scanner/context_drift/verify.rs#L78-L85
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/read_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_dependency_cache_has_dependency
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_package_json_dependency_check
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_verify_dependency_claim_matched
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_verify_dependency_claim_drifted
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_security_claim_jwt_verified
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/test_go_mod_dependency_check
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/verify/verify_claims
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub(crate) fn new(root: &Path) -> Self`

# Calls

- [read_file](../../../../../../functions/src/scanner/context_drift/discover/read_file.md)

# Called by

- [test_dependency_cache_has_dependency](../../../../../../functions/src/scanner/context_drift/test_dependency_cache_has_dependency.md)
- [test_package_json_dependency_check](../../../../../../functions/src/scanner/context_drift/test_package_json_dependency_check.md)
- [test_verify_dependency_claim_matched](../../../../../../functions/src/scanner/context_drift/test_verify_dependency_claim_matched.md)
- [test_verify_dependency_claim_drifted](../../../../../../functions/src/scanner/context_drift/test_verify_dependency_claim_drifted.md)
- [test_security_claim_jwt_verified](../../../../../../functions/src/scanner/context_drift/test_security_claim_jwt_verified.md)
- [test_go_mod_dependency_check](../../../../../../functions/src/scanner/context_drift/test_go_mod_dependency_check.md)
- [verify_claims](../../../../../../functions/src/scanner/context_drift/verify/verify_claims.md)
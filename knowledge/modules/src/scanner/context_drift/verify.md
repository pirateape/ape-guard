---
type: Rust Module
title: verify
resource: src/scanner/context_drift/verify.rs#L1-L720
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-discover-path-pattern-read-file-technology-keyword-pattern-version-pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-types-claimcategory-contextclaim-driftfinding-verificationresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [verify_claims](../../../../functions/src/scanner/context_drift/verify/verify_claims.md)
- [drift_severity](../../../../functions/src/scanner/context_drift/verify/drift_severity.md)
- [verify_single_claim](../../../../functions/src/scanner/context_drift/verify/verify_single_claim.md)
- [DependencyCache](../../../../classes/src/scanner/context_drift/verify/DependencyCache.md)
- [new](../../../../functions/src/scanner/context_drift/verify/DependencyCache/new.md)
- [has_dependency](../../../../functions/src/scanner/context_drift/verify/DependencyCache/has_dependency.md)
- [extract_json_section](../../../../functions/src/scanner/context_drift/verify/extract_json_section.md)
- [extract_dep_name](../../../../functions/src/scanner/context_drift/verify/extract_dep_name.md)
- [verify_dependency_claim](../../../../functions/src/scanner/context_drift/verify/verify_dependency_claim.md)
- [verify_path_claim](../../../../functions/src/scanner/context_drift/verify/verify_path_claim.md)
- [verify_architecture_claim](../../../../functions/src/scanner/context_drift/verify/verify_architecture_claim.md)
- [search_for_technology_usage](../../../../functions/src/scanner/context_drift/verify/search_for_technology_usage.md)
- [verify_convention_claim](../../../../functions/src/scanner/context_drift/verify/verify_convention_claim.md)
- [verify_security_claim](../../../../functions/src/scanner/context_drift/verify/verify_security_claim.md)
- [verify_command_claim](../../../../functions/src/scanner/context_drift/verify/verify_command_claim.md)
- [verify_semantic_claim](../../../../functions/src/scanner/context_drift/verify/verify_semantic_claim.md)
- [drift_findings_to_canonical](../../../../functions/src/scanner/context_drift/verify/drift_findings_to_canonical.md)

# Imports

- `super::discover::{path_pattern, read_file, technology_keyword_pattern, version_pattern}`
- `super::types::{ClaimCategory, ContextClaim, DriftFinding, VerificationResult}`
- `crate::find::{CanonicalFinding, FindingLocation, ScannerType, Severity}`
- `regex::Regex`
- `std::path::Path`

# Member of

- [apeguard](../../../../packages/apeguard.md)
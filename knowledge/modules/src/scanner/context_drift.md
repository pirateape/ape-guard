---
type: Rust Module
title: context_drift
resource: src/scanner/context_drift/mod.rs#L1-L746
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/pub-use-types-contextclaim-driftfinding-verificationresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-verify-drift-findings-to-canonical
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-discover-discover-context-files
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-parse-parse-all-context-files
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-verify-verify-claims
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-discover-detect-file-type
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-parse-classify-claim-parse-agents-md-parse-claude-md-parse-cursor-rules
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-types-claimcategory-contextfileref-contextfiletype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-crate-use-verify-drift-severity-extract-dep-name-extract-json-section-search-for-technology-usage-verify-dependency-claim-verify-path-claim-verify-single-claim-dependencycache
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-scanner-scannererror-scannertype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-confidence-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ContextDriftScanner](../../../classes/src/scanner/context_drift/ContextDriftScanner.md)
- [new](../../../functions/src/scanner/context_drift/ContextDriftScanner/new.md)
- [with_unknown](../../../functions/src/scanner/context_drift/ContextDriftScanner/with_unknown.md)
- [with_max_findings](../../../functions/src/scanner/context_drift/ContextDriftScanner/with_max_findings.md)
- [scan_drift](../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
- [discover_claims](../../../functions/src/scanner/context_drift/ContextDriftScanner/discover_claims.md)
- [default](../../../functions/src/scanner/context_drift/ContextDriftScanner/default/default.md)
- [DriftScanResult](../../../classes/src/scanner/context_drift/DriftScanResult.md)
- [DriftCounts](../../../classes/src/scanner/context_drift/DriftCounts.md)
- [count_by_severity](../../../functions/src/scanner/context_drift/count_by_severity.md)
- [name](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/check_installed.md)
- [version](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/parse_output.md)
- [install_hint](../../../functions/src/scanner/context_drift/ContextDriftScanner/scanner/install_hint.md)
- [temp_file](../../../functions/src/scanner/context_drift/temp_file.md)
- [test_detect_file_type](../../../functions/src/scanner/context_drift/test_detect_file_type.md)
- [test_discover_context_files_agents_md](../../../functions/src/scanner/context_drift/test_discover_context_files_agents_md.md)
- [test_parse_agents_md_dependency_claim](../../../functions/src/scanner/context_drift/test_parse_agents_md_dependency_claim.md)
- [test_parse_cursor_rules_frontmatter](../../../functions/src/scanner/context_drift/test_parse_cursor_rules_frontmatter.md)
- [test_dependency_cache_has_dependency](../../../functions/src/scanner/context_drift/test_dependency_cache_has_dependency.md)
- [test_package_json_dependency_check](../../../functions/src/scanner/context_drift/test_package_json_dependency_check.md)
- [test_extract_dep_name_technology](../../../functions/src/scanner/context_drift/test_extract_dep_name_technology.md)
- [test_extract_json_section](../../../functions/src/scanner/context_drift/test_extract_json_section.md)
- [test_verify_dependency_claim_matched](../../../functions/src/scanner/context_drift/test_verify_dependency_claim_matched.md)
- [test_verify_dependency_claim_drifted](../../../functions/src/scanner/context_drift/test_verify_dependency_claim_drifted.md)
- [test_verify_path_claim_exists](../../../functions/src/scanner/context_drift/test_verify_path_claim_exists.md)
- [test_verify_path_claim_missing](../../../functions/src/scanner/context_drift/test_verify_path_claim_missing.md)
- [test_security_claim_jwt_verified](../../../functions/src/scanner/context_drift/test_security_claim_jwt_verified.md)
- [test_full_drift_scan_no_context_files](../../../functions/src/scanner/context_drift/test_full_drift_scan_no_context_files.md)
- [test_full_drift_scan_with_claims](../../../functions/src/scanner/context_drift/test_full_drift_scan_with_claims.md)
- [test_drift_findings_to_canonical_conversion](../../../functions/src/scanner/context_drift/test_drift_findings_to_canonical_conversion.md)
- [test_classify_claim_dependency](../../../functions/src/scanner/context_drift/test_classify_claim_dependency.md)
- [test_classify_claim_security](../../../functions/src/scanner/context_drift/test_classify_claim_security.md)
- [test_classify_claim_convention](../../../functions/src/scanner/context_drift/test_classify_claim_convention.md)
- [test_parse_claude_md_structure](../../../functions/src/scanner/context_drift/test_parse_claude_md_structure.md)
- [test_technology_search_rust](../../../functions/src/scanner/context_drift/test_technology_search_rust.md)
- [test_go_mod_dependency_check](../../../functions/src/scanner/context_drift/test_go_mod_dependency_check.md)
- [test_verification_result_types](../../../functions/src/scanner/context_drift/test_verification_result_types.md)
- [test_empty_context_file_no_claims](../../../functions/src/scanner/context_drift/test_empty_context_file_no_claims.md)
- [test_scanner_name_and_type](../../../functions/src/scanner/context_drift/test_scanner_name_and_type.md)
- [test_drift_severity_mapping](../../../functions/src/scanner/context_drift/test_drift_severity_mapping.md)

# Imports

- `pub use types::{ContextClaim, DriftFinding, VerificationResult}`
- `pub use verify::drift_findings_to_canonical`
- `pub(crate) use discover::discover_context_files`
- `pub(crate) use parse::parse_all_context_files`
- `pub(crate) use verify::verify_claims`
- `pub(crate) use discover::detect_file_type`
- `pub(crate) use parse::{classify_claim, parse_agents_md, parse_claude_md, parse_cursor_rules}`
- `pub(crate) use types::{ClaimCategory, ContextFileRef, ContextFileType}`
- `pub(crate) use verify::{
    drift_severity, extract_dep_name, extract_json_section, search_for_technology_usage,
    verify_dependency_claim, verify_path_claim, verify_single_claim, DependencyCache,
}`
- `crate::find::{CanonicalFinding, Severity}`
- `crate::scanner::{Scanner, ScannerError, ScannerType}`
- `std::path::{Path, PathBuf}`
- `super::*`
- `crate::find::{Confidence, Severity}`
- `std::io::Write`

# Member of

- [apeguard](../../../packages/apeguard.md)
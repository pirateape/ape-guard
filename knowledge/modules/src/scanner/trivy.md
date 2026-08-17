---
type: Rust Module
title: trivy
resource: src/scanner/trivy.rs#L1-L551
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-scanner-scannererror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Trivy](../../../classes/src/scanner/trivy/Trivy.md)
- [TrivyMode](../../../classes/src/scanner/trivy/TrivyMode.md)
- [new](../../../functions/src/scanner/trivy/Trivy/new.md)
- [with_mode](../../../functions/src/scanner/trivy/Trivy/with_mode.md)
- [with_mode_and_binary](../../../functions/src/scanner/trivy/Trivy/with_mode_and_binary.md)
- [mode_flag](../../../functions/src/scanner/trivy/Trivy/mode_flag.md)
- [mode_name](../../../functions/src/scanner/trivy/Trivy/mode_name.md)
- [name](../../../functions/src/scanner/trivy/Trivy/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/trivy/Trivy/scanner/scanner_type.md)
- [install_hint](../../../functions/src/scanner/trivy/Trivy/scanner/install_hint.md)
- [check_installed](../../../functions/src/scanner/trivy/Trivy/scanner/check_installed.md)
- [version](../../../functions/src/scanner/trivy/Trivy/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/trivy/Trivy/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/trivy/Trivy/scanner/parse_output.md)
- [parse_vuln](../../../functions/src/scanner/trivy/Trivy/parse_vuln.md)
- [TrivyReport](../../../classes/src/scanner/trivy/TrivyReport.md)
- [TrivyResult](../../../classes/src/scanner/trivy/TrivyResult.md)
- [TrivyVuln](../../../classes/src/scanner/trivy/TrivyVuln.md)
- [parse_secret](../../../functions/src/scanner/trivy/Trivy/parse_secret.md)
- [TrivyReport](../../../classes/src/scanner/trivy/TrivyReport-2.md)
- [TrivySecretResult](../../../classes/src/scanner/trivy/TrivySecretResult.md)
- [TrivySecret](../../../classes/src/scanner/trivy/TrivySecret.md)
- [parse_misconfig](../../../functions/src/scanner/trivy/Trivy/parse_misconfig.md)
- [TrivyReport](../../../classes/src/scanner/trivy/TrivyReport-3.md)
- [TrivyMisconfigResult](../../../classes/src/scanner/trivy/TrivyMisconfigResult.md)
- [TrivyMisconfig](../../../classes/src/scanner/trivy/TrivyMisconfig.md)
- [MisconfigCauseMetadata](../../../classes/src/scanner/trivy/MisconfigCauseMetadata.md)
- [make_trivy](../../../functions/src/scanner/trivy/make_trivy.md)
- [test_parse_vuln_real_fixture](../../../functions/src/scanner/trivy/test_parse_vuln_real_fixture.md)
- [test_parse_vuln_empty_results](../../../functions/src/scanner/trivy/test_parse_vuln_empty_results.md)
- [test_parse_vuln_no_vulnerabilities_key](../../../functions/src/scanner/trivy/test_parse_vuln_no_vulnerabilities_key.md)
- [test_parse_vuln_invalid_json](../../../functions/src/scanner/trivy/test_parse_vuln_invalid_json.md)
- [test_parse_secret_fixture](../../../functions/src/scanner/trivy/test_parse_secret_fixture.md)

# Imports

- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::path::Path`
- `super::*`

# Member of

- [apeguard](../../../packages/apeguard.md)
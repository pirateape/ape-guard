---
type: Rust Module
title: orchestrate
resource: src/orchestrate.rs#L1-L731
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/sha2-digest-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-future-join-all
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-cache
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-chain
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-cli
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-dedup
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-filter
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-self-attackchain-findingsbyseverity-zerotrustscorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-grade
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-normalize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-policy
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-report
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-aws-s3-awss3scanner-checkov-checkov-container-containerscanner-context-drift-dast-dastscanner-gitleaks-gitleaks-mcp-security-mcpscanner-semgrep-semgrep-syft-syft-terraform-terraformscanner-tls-tlsscanner-trivy-trivy-trufflehog-trufflehog-scanner-scannerresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-score
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-stride
    resolved_by: tree-sitter
    confidence: exact
  - target: external/find-severity
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ScanOutput](../../classes/src/orchestrate/ScanOutput.md)
- [ScanArgs](../../classes/src/orchestrate/ScanArgs.md)
- [filter_by_severity](../../functions/src/orchestrate/filter_by_severity.md)
- [load_completed_scanners](../../functions/src/orchestrate/load_completed_scanners.md)
- [check_fail_on](../../functions/src/orchestrate/check_fail_on.md)
- [generate_summary_report](../../functions/src/orchestrate/generate_summary_report.md)
- [run_scan](../../functions/src/orchestrate/run_scan.md)

# Imports

- `sha2::Digest as _`
- `std::io::Write as _`
- `std::path::{Path, PathBuf}`
- `std::time::Instant`
- `futures::future::join_all`
- `crate::cache`
- `crate::chain`
- `crate::cli`
- `crate::config`
- `crate::dedup`
- `crate::filter`
- `crate::find::{self, AttackChain, FindingsBySeverity, ZeroTrustScorecard}`
- `crate::grade`
- `crate::llm`
- `crate::normalize`
- `crate::policy`
- `crate::reachability`
- `crate::report`
- `crate::scanner::{
    aws_s3::AwsS3Scanner, checkov::Checkov, container::ContainerScanner, context_drift,
    dast::DastScanner, gitleaks::Gitleaks, mcp_security::McpScanner, semgrep::Semgrep, syft::Syft,
    terraform::TerraformScanner, tls::TlsScanner, trivy::Trivy, trufflehog::Trufflehog, Scanner,
    ScannerResult,
}`
- `crate::score`
- `crate::stride`
- `find::Severity`

# Member of

- [apeguard](../../packages/apeguard.md)
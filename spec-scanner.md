---
id: apeguard-scanner
type: module-design
title: scanner/ — Scanner trait + 13 scanner implementations
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# scanner/ — Scanner trait + 13 scanner implementations

## Responsibility

`src/scanner/mod.rs` defines the `Scanner` async trait (`name`, `scanner_type`,
`check_installed`, `version`, `scan_raw`, `parse_output`, `scan`, `install_hint`) plus
shared helpers: `binary_exists`, `generate_finding_id` (AG-{SCANNER}-{DATE}-{NONCE8}-{SEQ}),
`is_empty_output`, `run_command_with_timeout`, and `reject_flag_like_arg` (argument-injection
guard for scanner targets).

The directory holds 13 scanner implementations, each wrapping an external CLI or native
Rust logic:

- `gitleaks.rs` — Gitleaks secrets (Layer 1)
- `trufflehog.rs` — TruffleHog secrets (Layer 1)
- `semgrep.rs` — Semgrep SAST (Layer 2)
- `trivy.rs` — Trivy in 3 modes: Vuln (SCA, Layer 3), Secret (Layer 3), Misconfig
  (Layer 3), plus Container mode (Layer 4)
- `container.rs` — ContainerScanner for container images (Layer 4)
- `dast.rs` — DastScanner via Nuclei (Layer 5)
- `checkov.rs` — Checkov IaC misconfiguration (Layer 6)
- `syft.rs` — Syft SBOM inventory (Layer 7)
- `arch.rs` — ArchitectureAnalyzer (architecture artifacts)
- `context_drift/` — ContextDriftScanner (Layer 8, native Rust; types/discover/parse/verify)
- `mcp_security.rs` — McpScanner (Layer 9, MCP server/tool poisoning)
- `terraform.rs` — TerraformScanner (Layer 10)
- `aws_s3.rs` — AwsS3Scanner (Layer 11)
- `tls.rs` — TlsScanner (Layer 12)

`ScannerResult` is `Complete` / `NotInstalled` (with install hint) / `Error`.
`ScannerError` covers NotFound, InvalidArgument, ExecutionFailed, ParseFailed, Timeout, Io.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ScannerType}`, `async-trait`, `tokio`,
  `which`, `chrono`, `uuid`, `sha2`, `glob`, `thiserror`, `anyhow`, `reqwest`.
- **Forbidden reaches:** must not import `orchestrate`, `report`, `cli`, `main`. Scanners
  are leaf modules owned by the pipeline.

## Invariant

- Finding IDs use the shared `generate_finding_id(prefix, index)` format so all scanners
  emit `AG-{SCANNER}-{DATE}-{NONCE8}-{SEQ}`.
- `reject_flag_like_arg` blocks scanner targets beginning with `-` to prevent argument
  injection.

## Open questions

- `ScannerType::Zap` exists in the enum but no `zap.rs` impl is wired into
  `orchestrate.rs` — confirm stub vs dead variant.

---
id: apeguard-config
type: module-design
title: config.rs — configuration load/merge
status: draft
parent: apeguard-architecture
depends-on: [apeguard-cli, apeguard-llm, apeguard-reachability, apeguard-stride, apeguard-policy]
references: []
implements: []
covers: []
tags: []
---

# config.rs — configuration load/merge

## Responsibility

`src/config.rs` loads and merges configuration from **defaults → file → environment → CLI
flags**. The `Config` struct holds: `layers`, `severity`, `binaries` (scanner paths),
`cache`, `report`, `llm`, `context_drift`, `filters` (false-positive suppression),
`reachability`, `stride`, `policy`, and `output_dir`. `load()` merges defaults with
`.apeguard.yaml` (auto-discovered or `--config`), `APEGUARD_*` env vars, and CLI overrides.
`generate_init()` writes a default `.apeguard.yaml`. Sub-structs: `ScannerBinaries`,
`CacheConfig` (with `store_findings` flag), `ReportConfig`, `FilterConfig`,
`ContextDriftConfig`.

## Boundary

- **Allowed deps:** `cli`, `llm::{LlmConfig}`, `reachability::{ReachabilityConfig}`,
  `stride::{StrideConfig}`, `policy::{PolicyConfig}`, `serde`, `std::path::PathBuf`,
  `tempfile` (tests), `anyhow`, `serde_yaml`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `main`.
  `filter.rs` reads `config::FilterConfig` (read-only direction).

## Invariants

- Merge precedence: defaults < file < env < CLI.
- Empty `layers` in an overlay does **not** override existing layers (defensive merge).
- `store_findings` defaults `true`; set `false` to store only scan metadata (report/compare
  depend on stored findings).

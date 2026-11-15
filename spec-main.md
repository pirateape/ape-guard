---
id: apeguard-main
type: module-design
title: main.rs — entry point & CLI dispatch
status: draft
parent: apeguard-architecture
depends-on: [apeguard-cli, apeguard-config, apeguard-orchestrate, apeguard-mcp, apeguard-cache, apeguard-report, apeguard-chain, apeguard-find, apeguard-normalize, apeguard-arch]
references: []
implements: []
covers: []
tags: []
---

# main.rs — entry point & CLI dispatch

## Responsibility

`src/main.rs` is the binary entry point (`#[tokio::main] async fn main`). It parses CLI
args, initializes tracing (honouring `--log-level`/`--no-color`, writing to stderr),
installs a SIGINT/SIGTERM handler that cleans up child scanner processes (`pkill -P`),
loads config, and dispatches to each command: `scan` (builds `orchestrate::ScanArgs`,
runs `run_scan`, records scan in cache, exits 1 on fail threshold), `report` (regenerate
from cache), `compare` (diff two snapshots), `init` (generate `.apeguard.yaml`),
`config` (validate/paths/show), `version` (scanner availability status), `completions`,
`serve` (MCP server), `cache` (stats/prune).

## Boundary

- **Allowed deps:** `cli`, `config`, `orchestrate`, `mcp`, `cache`, `report`, `chain`,
  `find`, `normalize`, `arch`, `chrono`, `sha2`, `tokio`, `tracing`, `tracing-subscriber`,
  `anyhow`, `std::process`.
- **Forbidden reaches:** leaf analysis modules must not import `main`; `main` is the
  ultimate consumer (top of the dependency graph).

## Invariants

- Tracing writes to stderr so stdout stays clean for machine-readable output.
- Signal handler kills child scanner processes before `exit(130)` on Ctrl+C / SIGTERM.
- `--fail-on high`/`critical` and `--ci` gate `std::process::exit(1)` for CI pipelines.

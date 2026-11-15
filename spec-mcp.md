---
id: apeguard-mcp
type: module-design
title: mcp.rs — Model Context Protocol server
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find, apeguard-cache, apeguard-report, apeguard-chain, apeguard-normalize, apeguard-arch]
references: []
implements: []
covers: []
tags: []
---

# mcp.rs — Model Context Protocol server

## Responsibility

`src/mcp.rs` implements the Model Context Protocol (MCP) server (`apeguard serve`) for AI
agent integration. Exposes 6 tools: `scan` (full assessment, layers 1–12, optional severity
filter), `findings` (last-scan findings with severity filter + limit), `scorecard` (ZT
scorecard), `chains` (attack-chain analysis), `arch_analysis` (component risks + Mermaid
diagram), plus standard `initialize`/`tools/list`. Exposes resources:
`apeguard://reports/latest`, `apeguard://reports/{scan_id}`, `apeguard://scorecard/latest`,
`apeguard://scorecard/{scan_id}`, `apeguard://latest`, `apeguard://arch_analysis`.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ZeroTrustScorecard, AttackChain, ...}`,
  `cache::ScanCache`, `report`, `chain`, `normalize`, `arch`, `tokio`, `serde`,
  `serde_json`, `std::path::{Path, PathBuf}`, `anyhow`, `tracing`.
- **Forbidden reaches:** must not import `cli`, `main`, `orchestrate` (it is a separate
  command entry point owned by `main.rs`).

## Notes

- MCP server is a standalone command; `main.rs` dispatches to `mcp::serve()`.
- Resources are keyed by scan_id from the cache; `report`/`scorecard` resources read from
  `ScanCache`.

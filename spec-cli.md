---
id: apeguard-cli
type: module-design
title: cli.rs — CLI argument parsing
status: draft
parent: apeguard-architecture
depends-on: []
references: []
implements: []
covers: []
tags: []
---

# cli.rs — CLI argument parsing

## Responsibility

`src/cli.rs` defines the clap `Args` structure and all `Command`/`ValueEnum` types:
`Scan` (path, layers, web, container, severity, no_cache, format, reports, fail_on,
resume, grade, context_drift, stride, policy, policy_dir, output_dir), `Report`,
`Compare`, `Init`, `Config`, `Version`, `Completions`, `Serve`, `Cache`. It also defines
`parse()`, `generate_completions()`, and `Command::is_scan()`.

## Boundary

- **Allowed deps:** `clap` (derive, env, complete), `std::io`.
- **Forbidden reaches:** must not import any other apeguard module. Pure argument
  definitions; `main.rs` consumes `cli::Args`.

## Notes

- Default layers: `1,2,3,4`. Default severity: `all`. Default format: `md`. Default
  reports: `tech,exec,roadmap`. Default output_dir: `.apeguard/reports`. Default fail_on:
  `never` (CI mode upgrades to `high`).

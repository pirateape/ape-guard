# ApeGuard — Agent Guide

## Project Overview

This project is written in Rust. We use clap for CLI argument parsing, tokio for async runtime, and serde for serialization. The app is an orchestrator and synthesizer that wraps existing OSS security scanning tools (Gitleaks, Semgrep, Trivy, Nuclei, Checkov, Syft) with unified analysis and reporting.

## Dependencies

We use clap for CLI parsing.
We use tokio for async runtime.
We use serde for serialization.
We use tera for report template rendering.
We use rusqlite for scan caching.
We use regex for pattern matching.
We use tracing for structured logging.

## Architecture

The main entry point is in `src/main.rs`.
The CLI module is at `src/cli.rs`.
The config module is at `src/config.rs`.
The scanner module is at `src/scanner/`.
The find module is at `src/find/mod.rs`.
The find module defines the CanonicalFinding struct that all scanners output.
The report module is at `src/report/mod.rs`.
The normalize module is at `src/normalize.rs`.
The dedup module is at `src/dedup.rs`.
The chain module is at `src/chain.rs`.
The score module is at `src/score.rs`.
The grade module is at `src/grade.rs`.
The llm module is at `src/llm.rs`.
The arch module is at `src/arch.rs`.
The cache module is at `src/cache.rs`.
The mcp module is at `src/mcp.rs`.

## Scanner Layers

The project has 8 scanner layers. Layers 1 through 7 wrap external binaries. Layer 8 is an internal scanner with no external binary.

Layer 1 uses Gitleaks for secret scanning.
Layer 2 uses Semgrep for SAST.
Layer 3 uses Trivy filesystem for SCA.
Layer 4 uses Trivy image for container scanning.
Layer 5 uses Nuclei for DAST.
Layer 6 uses Checkov for IaC scanning.
Layer 7 uses Syft for SBOM.
Layer 8 is the context drift scanner at `src/scanner/context_drift.rs`.

## Conventions

Always use explicit error handling with Result types.
Never use unwrap() in production code paths — use expect() in tests only.
Prefer early returns over nested if chains.
Avoid cloning large structs unnecessarily.

## Build Commands

Run `cargo build` to compile.
Run `cargo test` to run the test suite.
Run `cargo clippy -- -D warnings` to check lints.
Run `cargo fmt --check` to verify formatting.

## Loop Engineering

This project is operated with [loop engineering](https://github.com/cobusgreyling/loop-engineering) patterns. See `LOOP.md` for the full loop spec and `loop-constraints.md` for binding rules.

### Files

- `STATE.md` — per-project run-state spine. Loop reads this first every run; updates `Last run` and High Priority items.
- `LOOP.md` — describes the loops that operate on ApeGuard (cadence, skills, gates, upgrade path).
- `loop-constraints.md` — binding rules; the loop reads this at the start of every run before any action.
- `loop-budget.md` — daily token caps, kill switch, per-loop limits.
- `loop-run-log.md` — append-only history of every scheduled run.

### Active Loops

- **Daily Triage (L1)** — 1d cadence via macOS launchd. Report-only. No source edits in week 1–2.
- **Dependency Sweeper (L2, future)** — 6h cadence, patch CVE-only, requires `loop-verifier` in worktree.
- **CI Sweeper (L2, future)** — event-driven on `ci.yml` workflow_run failure. Max 3 attempts → escalate.

### Skills in Use

- `loop-triage` — produces prioritized findings (High Priority / Watch / Noise / State Updates). No narrative.
- `loop-verifier` — REJECT-by-default. Must run `cargo test + cargo clippy -D + cargo fmt --check` in worktree.
- `minimal-fix` — smallest possible diff that addresses one failure. No unrelated refactors.
- `loop-guard` — circuit breaker via `loop-ledger.json`. Stops loops from infinite-retrying.
- `loop-constraints` — reads `loop-constraints.md` and bakes rules into context before any triage runs.
- `loop-budget` — checks spend at start/end of each run; pauses loop on exceed.

### Sub-Agents

Named agents live in `opencode.json` at repo root. Default to maker/checker split; never let an implementer mark its own work "done".

### Invariants (Do Not Violate)

- The 8 scanner layers are orthogonal — never merge layers without design discussion.
- CanonicalFinding struct (`src/find/mod.rs`) public fields are the contract — do not change without migration story.
- Pipeline order is sacred: `scan → normalize → dedup → REACHABILITY → FP FILTER → LLM → GRADE → severity → POLICY → chains → SCORE → ZT → STRIDE → report`. Any insertion requires discussion.

### Kill Switch

Set `loop-pause-all: true` in `STATE.md`. The loop-triage skill reads this flag at start and exits without action.

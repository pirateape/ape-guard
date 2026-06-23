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

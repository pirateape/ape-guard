---
id: apeguard-orchestrate
type: module-design
title: orchestrate.rs — scan pipeline orchestrator
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find, apeguard-scanner, apeguard-config, apeguard-cache, apeguard-chain, apeguard-dedup, apeguard-filter, apeguard-grade, apeguard-llm, apeguard-normalize, apeguard-policy, apeguard-reachability, apeguard-report, apeguard-score, apeguard-stride]
references: []
implements: []
covers: []
tags: []
---

# orchestrate.rs — scan pipeline orchestrator

## Responsibility

`src/orchestrate.rs` owns the top-level scan pipeline (`run_scan`). It generates ONE
canonical `scan_id` (`scan_{YYYYmmdd_HHMMSSmmm}`) used by BOTH the reports and the cache
record, so `report --snapshot <id>` and `compare <id>` work with the ID shown in reports.
It collects scanners by layer, runs them in parallel, streams each result to
`found_findings.jsonl` (status line + one line per finding, enabling `--resume`), then
applies the analysis stages in order: normalize → cross-reference → deduplicate →
reachability → FP filter → LLM remediation → adversarial grading → post-grade rejection →
ZT scorecard → severity filter → policy-as-code → fail-on check → attack chains → risk
scoring → STRIDE → report generation. It also holds `ScanArgs` (pipeline arguments) and
`ScanOutput` (all data main.rs needs for reporting/exit codes), plus helpers
`filter_by_severity`, `load_completed_scanners`, `load_previous_findings`,
`check_fail_on`, `generate_summary_report`.

## Boundary

- **Allowed deps:** `cli`, `config`, `cache`, `chain`, `dedup`, `filter`, `find`,
  `grade`, `llm`, `normalize`, `policy`, `reachability`, `report`, `scanner` (all 13
  impls), `score`, `stride`, `tokio`, `futures`, `sha2`, `chrono`, `serde_json`,
  `std::fs`, `tracing`.
- **Forbidden reaches:** must not be imported by `main.rs` for anything other than
  `run_scan`; leaf analysis modules must not import orchestrate (no cycles).

## Pipeline invariants

- Scanners run in parallel via `futures::future::join_all`.
- `found_findings.jsonl` is TRUNCATED at scan start when NOT resuming (stale entries
  from an earlier run would otherwise cause a later `--resume` to skip unrelated
  scanners). Every scanner result (complete / not-installed / error) is appended as a
  status line, and each finding is appended as a `"status": "finding"` line so
  `--resume` can reload results for skipped scanners via `load_previous_findings`.
- ZT scorecard is computed **before** the severity filter (reflects full findings).
- Grade-rejected findings are dropped AFTER grading (post-grade pass) — before the
  ZT scorecard, severity filter, and fail-on check.
- Attack chains and risk scores are computed after policy evaluation and severity filter.
- `fail_threshold_reached` drives `std::process::exit(1)` in `main.rs` for CI.

## Open questions

- `ScanOutput` and `ScanArgs` are `#[expect(dead_code)]` — confirm they are still the
  intended contract with `main.rs`.

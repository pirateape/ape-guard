---
id: apeguard-architecture
type: architecture-design
title: ApeGuard architecture
status: draft
parent: apeguard-goal
depends-on: []
references: []
implements: []
covers: []
tags: []
---

# Architecture

## Topology

ApeGuard is a single Rust binary (`apeguard`) built on a **scan pipeline** owned by
`orchestrate::run_scan`. The pipeline is a linear sequence of stages, but the
scanner-execution stage runs all selected scanners **in parallel** (`futures::join_all`).
The crate is layered so the core data model (`find/`) knows nothing about the rest of the
codebase; everything else depends on it.

## Module boundaries and dependency edges (DAG)

Edges point from importer → imported. `find/` is the root; nothing imports it back.

```
main.rs ──┐
          ├─ cli.rs      (standalone; clap argument parsing)
          ├─ config.rs   ─┐                          imports cli, llm, reachability,
          │               │                          stride, policy (config merge)
orchestrate.rs ─┼─ scanner/  (13 scanner impls + Scanner trait; each imports find only)
          ├─ find/       ─┤                          core data model; imports nothing
          ├─ normalize.rs ─┤                         imports find
          ├─ dedup.rs      ─┤                         imports find
          ├─ score.rs      ─┤                         imports find
          ├─ chain.rs      ─┤                         imports find
          ├─ grade.rs   ──┼─ llm.rs  ─┐               imports find + llm (Ollama call)
          ├─ report/     ─┤           │               imports find
          ├─ cache.rs    ─┘           │               imports find
          ├─ llm.rs      ─────────────┘               imports find
          ├─ arch.rs     ─────────────────────────── imports find
          ├─ stride.rs   ─────────────────────────── imports find
          ├─ policy.rs   ─────────────────────────── imports find
          ├─ reachability.rs ─────────────────────── imports find
          └─ filter.rs   ─┐                         imports find + config::FilterConfig
                          │
config.rs ────────────────┘ (owned by config; filter reads it)
```

Scan pipeline stage order (in `orchestrate::run_scan`):

1. Collect scanners by layer (layers 1–12; default `1,2,3,4`)
2. Run scanners in parallel; stream each result to `found_findings.jsonl` (enables `--resume`)
3. `normalize::normalize_findings` — ZT pillar mapping, Gitleaks severity overrides, MITRE tags
4. `dedup::cross_reference` — link findings at the same (file, line)
5. `dedup::deduplicate` — drop exact (file, line, rule_id) dupes
6. `reachability::analyze_reachability` + `apply_reachability` — opt-in; mark reachable/unreachable
7. `filter::apply_fp_filters` — 6-stage false-positive suppression
8. `llm::enhance_remediations` — opt-in Ollama remediation
9. `grade::grade_findings` — opt-in adversarial grading (Ollama)
10. `normalize::compute_zt_scorecard` — computed **before** severity filter (reflects full findings)
11. Severity filter (`filter_by_severity`)
12. `policy::evaluate_policies` — opt-in Rego/OPA
13. Fail-on threshold check (`check_fail_on`)
14. `chain::build_attack_chains`
15. `score::score_all_findings` — 6-dimension risk score
16. `stride::analyze_stride_coverage` — opt-in
17. `report::generate_all_reports` + JSON/SARIF/HTML writers

## Invariants the code enforces

- Every finding carries an ID of the form `AG-{SCANNER}-{DATE}-{NONCE8}-{SEQ}`
  (e.g. `AG-GL-20260531-a1b2c3d4-001`).
- All scanner output is normalized into `CanonicalFinding` before any analysis.
- Dedup key is `(file, line, rule_id)`; findings with `line == None` are always kept
  (cannot confirm a duplicate location).
- Cross-references group by `(file, line)` **regardless of rule_id**, so different rules at
  the same location still link.
- The Zero Trust scorecard is computed before the severity filter, so it reflects the full
  finding set, not the filtered subset.
- FP suppression and reachability/STRIDE/policy/grading are opt-in (disabled by default).
- The tool is 100% local: Ollama (grading/remediation) runs on localhost; no telemetry.

## Decisions (why, confirmed by user)

- **Default layers = 1,2,3,4** (secrets, SAST, SCA, container): those four need only
  commonly-available binaries and give the highest signal; DAST/IaC/TLS/etc. need optional
  scanner binaries plus targets, so they're gated behind `--layers`.
- **UZTF v2.0 mapping**: extends the CISA Zero Trust Maturity Model with quantitative,
  severity-weighted pillar scoring (0–100 per pillar, 0–1200 overall).
- **Elastic License 2.0**: permits use, modification, and redistribution — including
  commercial — but prohibits offering it as a paid SaaS product.
- **`find/` holds both the finding schema and the ZT scorecard types**: the canonical data
  model is central and intentionally dependency-free.
- **reachability / stride / policy / grade are opt-in**: they need external dependencies
  (Ollama, Rego policies) or are Phase 2+ features.

## Cross-references

- `docs/adr/0001-record-architecture-decisions.md` — ADR practice (adopted as spec node)
- `docs/grade-engine.md` — adversarial grader design (adopted as spec node)
- `docs/scoring-pipeline.md` — 6-dimension scoring design (adopted as spec node)

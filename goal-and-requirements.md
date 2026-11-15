---
id: apeguard-goal
type: goal-and-requirements
title: ApeGuard — one-command security posture assessment
status: draft
parent: ~
depends-on: []
references: []
implements: []
covers: []
tags: []
---

# Goal

**ApeGuard** is a one-command security posture assessment CLI (Rust) that runs a layered
scanning pipeline against a target (directory, container image, or web endpoint), normalizes
every scanner's output into a single canonical finding schema, then deduplicates, scores,
maps to the Unified Zero Trust Framework, builds attack chains, and emits three audience-
targeted reports (technical / executive / roadmap) in Markdown, JSON, SARIF, and HTML.

It is 100% local: no SaaS, no telemetry, and no data leaves the machine. Optional AI
remediation and adversarial grading run against a locally-hosted Ollama model.

## Why (the problem it solves)

Security teams run a dozen different scanners (secrets, SAST, SCA, container, DAST, IaC,
SBOM, …), each with its own output format, severity scale, and false-positive rate. ApeGuard
consolidates them into one command, one finding model, one risk score, and one Zero Trust
scorecard — so a single `apeguard scan` produces an actionable, comparable posture picture
instead of a dozen siloed reports.

## What it already does (capabilities, inferred from code)

- **12 scanning layers** orchestrated in parallel: secrets (Gitleaks, TruffleHog), SAST
  (Semgrep), SCA (Trivy vuln/secret/misconfig), container (Trivy image), DAST (Nuclei),
  IaC (Checkov), SBOM (Syft), context drift (native Rust), MCP security, Terraform IaC,
  AWS S3, TLS certificates. Layers 1–4 run by default; the rest are opt-in (`--layers`).
- **Canonical finding model** (`find::CanonicalFinding`): scanner, rule_id, severity,
  confidence, location, CWE, CVSS, remediation, evidence, tags, ZT pillars, cross-refs,
  grade, reachability, risk score.
- **Normalization**: Gitleaks rule→severity overrides, ZT pillar keyword mapping (12 UZTF
  pillars), MITRE ATT&CK tactic mapping, default pillar fallback ("applications").
- **Deduplication** by (file, line, rule_id); cross-referencing by (file, line) across
  scanners regardless of rule_id.
- **Pipeline** (`orchestrate::run_scan`): collect scanners by layer → run in parallel →
  stream to `found_findings.jsonl` (supports `--resume`) → normalize → cross-ref → dedup →
  reachability → FP filter → LLM remediation → adversarial grading → ZT scorecard → severity
  filter → policy-as-code → attack chains → risk scoring → STRIDE → reports.
- **Zero Trust scorecard** (`normalize::compute_zt_scorecard`): 12 UZTF v2.0 pillars, each
  scored 0–100 from severity-weighted deductions (Critical 20, High 15, Medium 10, Low 5,
  Info 1); maturity tier per pillar (Baseline/Advanced/Adaptive); overall scorecard
  0–1200 classified Initial / MostlyBaseline / MostlyAdvanced / MostlyAdaptive; gap analysis
  per pillar.
- **Attack chains** (`chain::build_attack_chains`): 4 structured rules (credential
  compromise, supply chain, infrastructure escalation, data exfiltration) plus multi-scanner
  confirmation and ZT cross-pillar strategies; risk scores capped at 10.
- **Risk scoring** (`score::score_all_findings`): 6 dimensions — severity, confidence,
  context (file-path heuristics), chain position, ZT pillar count, AI grade — weighted sum
  (defaults: sev 0.25, conf 0.20, ctx 0.15, chain 0.15, zt 0.10, grade 0.15).
- **Adversarial AI grading** (`grade.rs`): Ollama-backed "prove this is a false positive"
  grader per finding; verdicts Confirmed / Rejected / NeedsReview with reject reasons
  (TestCode, Unreachable, FalsePositive, AlreadyMitigated, SeverityInflated); guardrail check
  strips instruction-injection and dangerous-command patterns.
- **Reports**: technical (engineers), executive (leadership), roadmap (engineering managers);
  output formats md / json / sarif / html; auto-generated Mermaid architecture risk diagram.
- **Caching**: SQLite-backed scan cache (`cache.rs`) with TTL pruning, `--resume` skip of
  completed scanners, `--no-cache` force re-scan.
- **Config** (`config.rs`): merge defaults → `.apeguard.yaml` → env (`APEGUARD_*`) → CLI
  flags; scanner binary paths, cache TTL, report formats/types, filters, reachability,
  stride, policy.
- **False-positive suppression** (`filter.rs`): 6 stages — path exclusions (test/vendor/
  examples), test-code low-severity suppression, cross-scanner confirmation for low/info,
  AI-grade rejection, confidence threshold, severity floor.
- **CLI commands** (`cli.rs`): scan, report, compare, init, config, version, completions,
  serve (MCP server), cache (stats/prune).
- **MCP server** (`mcp.rs`): tools scan/findings/scorecard/chains/arch_analysis; resources
  `apeguard://reports/latest`, `apeguard://scorecard/latest`, `apeguard://arch_analysis`.
- **Reachability** (`reachability.rs`): import-graph BFS from discovered entry points to
  mark findings reachable/unreachable (dead-code findings lower risk).
- **STRIDE coverage** (`stride.rs`: findings mapped to Spoofing/Tampering/Repudiation/
  Information Disclosure/Denial of Service/Elevation of Privilege; coverage ratio per
  category vs threshold.
- **Policy-as-code** (`policy.rs`): Rego/OPA (regorus) policies evaluated against findings;
  actions block/escalate/downgrade/flag/tag.

## Explicit non-goals (confirmed by user)

- **Not a SaaS or managed service** — Elastic License 2.0 prohibits offering it as a paid
  SaaS product.
- **Not real-time continuous monitoring** — scan-on-demand CLI; no agents, no webhooks, no
  scheduled ingestion.
- **Not a SIEM or log aggregator** — it produces point-in-time scan reports, not event
  correlation or long-term log storage.
- **Not a replacement for dedicated DAST/IaC scanners** — it orchestrates Gitleaks,
  Semgrep, Trivy, Nuclei, Checkov, Syft, etc.; it does not re-implement their detection.

## Open questions / drift markers (marked unconfirmed)

- The exact version at which layer 8 (context drift) and layer 11 (AWS S3) became stable is
  not recorded in code; treat as "shipped" pending user confirmation.
- `ScannerType::Zap` exists in the enum but no `zap.rs` scanner impl is wired into
  `orchestrate.rs` — may be a stub or dead variant (P3/P4).
- `report/html` and `pdf` are listed in the README; the CLI `CompareFormat::Html` returns
  "not yet implemented" — treat HTML compare as unimplemented.

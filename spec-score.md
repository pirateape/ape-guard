---
id: apeguard-score
type: module-design
title: score.rs — 6-dimension risk scoring
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find, apeguard-chain]
references: []
implements: []
covers: []
tags: []
---

# score.rs — 6-dimension risk scoring

## Responsibility

`src/score.rs` computes a unified risk score per finding across 6 dimensions:
severity, confidence, context (file-path heuristics), chain position, ZT pillar count, and
AI grade. `ScoreWeights` (defaults: sev 0.25, conf 0.20, ctx 0.15, chain 0.15, zt 0.10,
grade 0.15; total 1.0). `score_all_findings` populates `finding.risk_score` with
`UnifiedRiskScore { overall, dimensions, score_confidence }`. Deterministic math only — no
LLM calls, no external deps.

Dimension scores (0.0–1.0): severity (Critical 1.0 … Info 0.1); confidence from cross-ref
count (0 → 0.4, 1 → 0.6, 2 → 0.8, 3+ → 0.95, boosted by AI Confirmed, crushed by Rejected
to 0.05); context from file path (test/docs 0.1 … crypto/TLS 0.95); chain (not in chain
0.3 / 1 chain 0.7 / 2+ chains 0.9); ZT pillars (0 → 0.1 … 3+ → 0.85); grade (Confirmed≥0.8
→ 0.9 … Rejected → 0.1, None → 0.5 neutral).

## Boundary

- **Allowed deps:** `find::{AttackChain, CanonicalFinding, GradeVerdict, RiskDimensions,
  Severity, UnifiedRiskScore}`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Open questions

- Default weights are calibrated but unvalidated against real scan data (noted in
  docs/scoring-pipeline.md open questions).

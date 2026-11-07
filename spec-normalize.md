---
id: apeguard-normalize
type: module-design
title: normalize.rs — ZT pillar mapping & scorecard
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# normalize.rs — ZT pillar mapping & scorecard

## Responsibility

`src/normalize.rs` post-processes raw findings into a unified, enriched format:
1. Gitleaks rule→severity overrides (`GITLEAKS_SEVERITY_MAP`, e.g. aws-access-token → Critical).
2. Zero Trust pillar keyword mapping (`ZT_MAPPINGS`, 12 UZTF pillars → keyword associations), with default pillar fallback `"applications"`.
3. MITRE ATT&CK tactic mapping (`mitre_mapping`, e.g. secret → TA0006 Credential Access).
4. `compute_zt_scorecard` — per-pillar score (100 − severity-weighted deduction, floored 0), maturity tier (Baseline <51 / Advanced ≥51 / Adaptive 100), overall scorecard (0–1200), posture classification (Initial / MostlyBaseline / MostlyAdvanced / MostlyAdaptive), gap analysis per pillar.
5. `compute_gap_analysis` — gap level (None/Small/Medium/Large) vs target maturity `Advanced`.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ...}`, `std::collections::HashMap`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Invariants

- Scorecard computed from **all** findings (orchestrate calls it before the severity filter).
- Default pillar `"applications"` when no keyword matches.
- Severity weights (UZTF §4.1): Critical 20, High 15, Medium 10, Low 5, Info 1.

## Open questions

- `ZT_MAPPINGS` keyword→pillar associations are heuristic; no per-pillar calibration data in repo.

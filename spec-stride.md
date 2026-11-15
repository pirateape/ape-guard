---
id: apeguard-stride
type: module-design
title: stride.rs — STRIDE threat model coverage
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# stride.rs — STRIDE threat model coverage

## Responsibility

`src/stride.rs` maps each security finding to one or more STRIDE categories
(Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of
Privilege) via keyword matching on rule_id, title, CWE, tags, and scanner type, then
analyzes coverage across the threat model: per-category finding count, coverage ratio, and
covered/not-covered vs a threshold. `StrideCategory` is `Copy + Hash` with label/id/
description/ all() helpers. `StrideCoverage` holds category + count + ratio + covered.

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `serde`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Notes

- Opt-in via `--stride` flag or `config.stride.enabled`; coverage threshold clamped 0–1.
- Keyword-based mapping (heuristic); no CWE→STRIDE table beyond keyword matching.

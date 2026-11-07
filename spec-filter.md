---
id: apeguard-filter
type: module-design
title: filter.rs — false-positive suppression pipeline
status: draft
parent: apeguard-architecture
depends-on: [apeguard-config, apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# filter.rs — false-positive suppression pipeline

## Responsibility

`src/filter.rs` applies a 6-stage false-positive suppression pipeline after dedup and
before LLM grading: (1) path-based exclusions (test/vendor/examples + custom patterns),
(2) test-code suppression (info/low in test files), (3) cross-scanner confirmation (require
2+ scanners for low/info), (4) AI-grade rejection (drop grader-rejected findings),
(5) confidence threshold (drop below min_confidence), (6) severity floor (drop below
min_severity). Returns filtered findings + `FilterStats` (counts per stage + total removed).

## Boundary

- **Allowed deps:** `config::FilterConfig`, `find::{CanonicalFinding, Confidence,
  GradeVerdict, Severity}`, `std::path::Path`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Invariants

- All filters opt-in; by default `exclude_test_files=true`, `exclude_vendor=true`,
  `exclude_examples=false`, `require_cross_scanner_for_low=false`, `min_confidence=0`,
  `min_severity=None`.
- `FilterConfig` is owned by `config.rs`; `filter.rs` reads it (read-only direction).

## Tests

Inline unit tests cover each filter stage and `FilterStats::total_removed` (in
`src/filter.rs`).

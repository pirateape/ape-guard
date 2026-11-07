---
id: apeguard-cache
type: module-design
title: cache.rs — SQLite scan cache
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# cache.rs — SQLite scan cache

## Responsibility

`src/cache.rs` is a SQLite-backed cache (`apeguard.db` in the cache dir) for incremental
scanning. `ScanCache::open` creates the `scan_history` table
(scan_id, target, started_at, completed_at, total_findings, scanners_used,
findings_snapshot). `ScanCache::disabled()` returns an in-memory no-op. Operations:
`record_scan(RecordScanInput)` (INSERT OR REPLACE; `store_findings` flag controls whether
the full findings snapshot or `[]` is persisted), `get_scan_findings`,
`get_latest_scan_findings`, `get_latest_scan_record` (metadata only), `stats`,
`prune(keep N)`, `enforce_ttl(ttl_hours)`, `recent_scans` (dead_code P3/P4).

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `rusqlite` (bundled), `serde`, `chrono`,
  `std::path::{Path, PathBuf}`, `anyhow`, `tracing`.
- **Forbidden reaches:** must not import any other apeguard module.

## Invariants

- `enforce_ttl` and `prune` run `VACUUM` when rows are removed.
- `store_findings = false` keeps metadata (count, scanners, timestamps) but stores an empty
  findings snapshot — `report --snapshot` and `compare` require `store_findings = true`.

## Tests

Inline unit tests cover timestamp persistence, store_findings flag, TTL pruning
(all in `src/cache.rs`).

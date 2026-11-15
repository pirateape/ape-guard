---
id: apeguard-reachability
type: module-design
title: reachability.rs — import-graph reachability analysis
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# reachability.rs — import-graph reachability analysis

## Responsibility

`src/reachability.rs` determines which source files are transitively reachable from known
entry points, to flag dead-code findings (unreachable = lower risk). Pipeline:
1. `discover_entry_points` — auto-discover well-known entry points (main.rs, main.py,
   index.js, main.go, …) plus user-specified globs.
2. `collect_source_files` — gather source files by extension, excluding vendor/build dirs.
3. `build_import_graph` — regex-based import extraction (Rust `use crate::`/`mod`, Python
   `import`/`from`, JS `import`/`require`, Go module-path imports, C `#include`) into an
   `Rc<PathBuf>` adjacency list + reverse graph.
4. `bfs_reachable_files` — transitive closure from entry points.
5. `analyze_reachability` / `apply_reachability` — set `finding.reachable`:
   - `Some(true)` — file transitively reachable from an entry point,
   - `Some(false)` — file WAS part of the import graph but is not reachable (dead code),
   - `None` — file was never part of the analysis (.env, Dockerfile, lockfiles) or has no
     meaningful path. Never-analyzed files must NOT read as unreachable.
   Relative finding paths are resolved against both CWD and the scan target (Gitleaks
   emits paths relative to `--source`).

`ReachabilityConfig { enabled, entry_points, include_extensions, exclude_dirs }`.

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `serde`, `std::collections::{HashMap,
  HashSet, VecDeque}`, `std::path::{Path, PathBuf}`, `std::rc::Rc`, `tracing`,
  `tempfile` (tests).
- **Forbidden reaches:** must not import any other apeguard module.

## Notes

- Opt-in (`enabled` defaults false).
- Go import resolution is skipped (V1) — Go uses full module paths, not relative file paths.
- `#[expect(dead_code)]` on `ReachabilityResult` fields (P3/P4: not all read yet).

## Tests

Inline unit tests cover entry-point discovery (Rust/Python/none), source collection with
exclusions, Rust mod/crate/submodule imports, Python from/relative imports, JS import/require
and index resolution, C include, BFS simple/cycle/unreachable/multiple entry points,
disabled analysis, reachability markings, config defaults (all in `src/reachability.rs`).

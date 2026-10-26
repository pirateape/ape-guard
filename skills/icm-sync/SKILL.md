---
name: icm-sync
description: >-
  Run sync-context.sh to refresh an ICM project's knowledge hubs. Use after
  substantive code changes, before commits, when context feels stale, after
  framework updates, or when okf-rs/codegraph/mempalace indexes lag the code.
  Covers flags, graceful degradation, and the idempotent entry-point copy.
---

# Sync Context

`sync-context.sh` is the maintenance loop for a project's knowledge hubs.

## When to run
- After substantive code changes (generated indexes lag the code).
- Before commits — the pre-commit hook runs it; run manually when bypassed.
- When context feels stale or an index is obviously behind.

## How
```bash
./sync-context.sh                    # everything that applies
./sync-context.sh --okf              # okf-rs bundle only (Rust projects)
./sync-context.sh --codegraph        # codegraph index only
./sync-context.sh --mempalace        # mempalace index only
./sync-context.sh --context          # context.md freshness check only
./sync-context.sh /path/to/project   # from outside the project
```

## What it refreshes
okf-rs `knowledge/` bundle (Rust only) · codegraph `.codegraph/` index ·
mempalace `.mempalace/` index · context.md freshness check · router.md
registration reminder. Missing CLI → informational message, still exit 0
(safe in CI and on bare machines).

## Entry-point copy
First run copies a standalone `sync-context.sh` into the project root
(idempotent — an existing entry point is never overwritten). The copy is a
snapshot: re-copy from the ICM checkout after framework updates.

Details: docs/CONVENTIONS.md §d (memory hubs) · docs/ROADMAP.md (drift guard)

---
name: icm-context
description: >-
  Two-tier context navigation for ICM projects. Use when working in an ICM
  project, reading project context, updating AGENTS.md or context.md, adding
  project knowledge, or when context feels stale or bloated. Enforces the
  small-tier/deep-tier split with "Details:" pointers.
---

# Two-Tier Context Navigation

Every ICM project has exactly two always-loaded context files — `AGENTS.md`
and `context.md` — kept small: ≤30 lines target; beyond 40 the contract suite
flags the file as stale-prone (test 17). Depth lives in `docs/`.

## The two tiers
- **Small tier (always loaded)**: `AGENTS.md` + `context.md`. High-signal,
  hand-editable, `TODO:`-marked where unknown.
- **Deep tier (on demand)**: `docs/` — ADRs (`docs/adr/`), research
  (`docs/research/`), `docs/ARCHITECTURE.md`, generated docs.

## Rules
1. When behavior changes, update the small tier in the same commit —
   staleness is the #1 context failure mode.
2. Never inline depth into the small tier. Add a "Details:" pointer:
   `Details: docs/research/<topic>.md`
3. Facts an agent must know before writing code → small tier. Everything
   else → deep tier, linked.

## Keeping it healthy
- After substantive changes run `./sync-context.sh` (see icm-sync).
- Line count creeping up? Move a section to `docs/`, leave a pointer.
- Verify structure with the contract suite (see icm-contract).

Details: docs/CONVENTIONS.md §a in the ICM checkout (two-tier rule, ADR-0005)

---
name: icm-contract
description: >-
  Run and interpret the ICM contract suite (19 checks). Use when verifying
  project structure, onboarding to an ICM project, after scaffold or
  convention changes, or when a contract test fails and you need to know
  what it means and how to fix it.
---

# Contract Suite

```bash
scripts/test-icm-framework.sh [project-dir]   # or ./test-icm-framework.sh if copied locally
```

Exit 0 = all hard-fail checks pass. Warnings never block.

## The 19 checks
- **Hard fail (7)**: 1 required files (AGENTS.md casing-tolerant, context.md,
  root router.md) · 2 AGENTS.md sections · 3 context.md sections ·
  4 mempalace.yaml · 7 sync-context.sh present + executable · 8 git hooks ·
  13 project registered in router.md.
- **Conditional (1)**: 16 workflow-module integrity — runs only when the
  feature-discovery orchestrator is installed.
- **Advisory (11)**: 5 .codegraph · 6 knowledge/ bundle · 9 CI workflow ·
  10 secrets template · 11 observability · 12 supply chain ·
  14 skills referenced in AGENTS.md · 15 APE-Brain referenced ·
  17 two-tier staleness · 18 v1.2 conventions · 19 base-skills integrity.

## Fixing failures
- Router registration (13): add the project to `$ICM_SRC_ROOT/router.md`.
- Hooks (8): `bash <icm-checkout>/scripts/install-git-hooks.sh <project>`.
- Sections (2–3): restore the missing `## Header` the suite greps for.
- Advisory ⚠️ lines are breadcrumbs — resolve when convenient, they never
  block.

Details: docs/CONVENTIONS.md §c (full table) · README "contract tests" table

---
name: icm-problem-solving
description: >-
  Route any non-trivial problem to the right methodology. Use when facing a
  complex, multi-step, or vaguely defined problem, investigating root causes,
  planning research, or when stuck. This is a routing skill: it points at the
  global methodology tier and encodes the project-local evidence loop — it is
  not a methodology itself.
---

# Problem-Solving Router

Routes to the **global methodology tier** — user-level skills and MCP that
evolve outside this repo. This skill is the project-local router, never a
copy: copying global methodology per-project would drift and duplicate
systems.

## Route
1. **Decompose** → `sequential-thinking` MCP
2. **Root-cause** → `first-principles-solver` skill
3. **Research** → the research agent (8-phase methodology)
4. **Pressure-test designs** → `multi-agent-brainstorming` skill
5. **Clarify vague asks** → `ice-methodology` skill

## The project-local evidence loop
question → local evidence (okf-rs / mempalace / codegraph in this repo) →
external (web / context7 / gh_grep) → triangulate 3+ sources → record in
`docs/research/` (findings) or `docs/adr/` (decisions) → durable findings
graduate to APE-Brain `02-Notes/` (see icm-research).

## Fallback loop (machines without the globals)
1. State the question and success criteria in one sentence.
2. Local evidence first: indexes, `docs/`, ADRs, git log.
3. External evidence second: upstream docs, changelogs, issues.
4. Triangulate 3+ independent sources — or state the confidence gap.
5. Record: research note or ADR; graduate durable findings.

Expected globals: `sequential-thinking` MCP; `first-principles-solver`,
`multi-agent-brainstorming`, `ice-methodology` (APE-Brain `~/.agents/skills/`).

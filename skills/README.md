# Project Skills

Agent skills extend an AI agent's capabilities with project-specific
instructions and resources. This directory is the project's curated skill home.

## Format (Agent Skills spec — https://agentskills.io)

Each skill is a directory containing a `SKILL.md` with YAML frontmatter:

```yaml
---
name: my-skill
description: >-
  What the skill does and when to use it ("Use when ..."). Keyword-rich,
  ≤1024 chars. `name` is kebab-case, ≤64 chars, and must match the directory.
---
```

The body below the frontmatter holds the instructions — keep it under
500 lines / ~5k tokens; link deeper material as separate resource files.

## Discovery

Runtimes discover skills from fixed locations and merge them with global ones:

- opencode: `.opencode/skills/` (this project) + `~/.config/opencode/skills/` (global)
- Copy or symlink a skill from `skills/` into `.opencode/skills/` when a runtime
  should load it automatically, and reference it from `AGENTS.md`.
- Validate with `skills-ref validate <skill-dir>` when the CLI is available.

## Project agent config (`opencode.json`)

The scaffolded `opencode.json` holds per-project MCP servers under the `mcp`
key, merged with your global config; `{env:VAR}` substitution keeps secrets
out of the committed file. JSON has no comments — extend it like:

```json
{ "mcp": { "context7": { "type": "remote", "url": "https://mcp.context7.com/mcp" } } }
```

## Shipped base skills (ADR-0006)

Six `icm-*` skills ship with the scaffold. Each is ≤40 lines and points into
`docs/` for depth (the two-tier rule applies to skills too) — they teach
agents this framework's moving parts:

| Skill | Use when |
|-------|----------|
| `icm-context` | Working in an ICM project; reading or updating the two-tier context (`AGENTS.md`/`context.md` vs `docs/`) |
| `icm-sync` | After substantive code changes, before commits, when context feels stale — run `sync-context.sh` |
| `icm-contract` | Verifying project structure, onboarding, after scaffold changes — run + interpret the contract suite |
| `icm-adr` | A decision alters conventions, scripts, or scaffold output — record it as an ADR |
| `icm-research` | Investigating, benchmarking, spiking — project-local research + the graduation rule |
| `icm-problem-solving` | Any non-trivial problem — routes to the global methodology tier + the project-local evidence loop |

The routing-skill principle: heavy methodology (sequential-thinking,
first-principles-solver, research agent, multi-agent-brainstorming,
ice-methodology) stays global at user level and evolves there —
`icm-problem-solving` is the project-local router to it, never a copy.

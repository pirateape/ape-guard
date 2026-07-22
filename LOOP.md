# LOOP.md — ApeGuard Loop Engineering

This file documents how ApeGuard is operated with loop engineering patterns.

## Active Loops

### Daily Triage (L1 — automated + report)

- Cadence: 1d weekdays (08:00 local via macOS launchd)
- Skill: `loop-triage` (auto-discovered from `~/.agents/skills/loop-triage/`)
- State: `STATE.md` at repo root (updated every run; human reviews weekly)
- Phase: **Report-only**. Human reviews and decides actions. No auto-fix in week 1–2.
- Handoff: Design decisions, refactors (P5-A through P5-F), new scanner layers, release prep.

### Dependency Sweeper (L2 — patch-only, future)

- Cadence: 6h–1d (planned for week 3+)
- Skill: `minimal-fix` + `loop-verifier`
- Scope: patch-version CVE fixes only for first 30 days; human gate on majors and denylisted crates
- Verifier: `cargo test + cargo clippy -D warnings + cargo fmt --check` in isolated worktree
- Denylist: `Cargo.toml` major bumps, `deny.toml` changes, `Cargo.lock` manual edits

### CI Sweeper (L2 — future, opportunistic)

- Trigger: `workflow_run` failure on ci.yml (event-driven, not polling cadence)
- Scope: failing CI on `main` branch only
- Max attempts per failure: 3 → escalate to human (enforced by `loop-guard`)

## Multi-Loop Coordination

Priority order (when multiple loops want to act on the same repo):

1. CI Sweeper (red main blocks everything)
2. PR Babysitter (if active PRs exist)
3. Dependency Sweeper (off-peak)
4. Post-Merge Cleanup (off-peak)
5. Daily Triage (report, never blocks)

State files must stay separated — one section per loop in `STATE.md`, or one `*-state.md` file per loop pattern when the loop graduates to L2+.

## Worktrees

- Any unattended code-change experiment runs in an **isolated git worktree** per attempt.
- One worktree per fix; discard after verifier REJECT or human escalation.
- Worktree naming: `../wt-apeguard-<fix-id>-<timestamp>` (sibling of repo root).
- Cleanup: `git worktree remove --force <path>` after PR merge or REJECT.

## Connectors (MCP)

- Read-only for L1 daily triage: filesystem read of `target/`, `Cargo.toml`, `.github/workflows/`, `docs/`.
- Optional GitHub MCP for issue/PR discovery — scope to read + comment until L2 trust earned.
- No database, no production writes from any loop.

## Budget & Observability

- Token caps: `loop-budget.md`
- Run history: `loop-run-log.md` (appended each scheduled run)
- Estimate: `npx @cobusgreyling/loop-cost --pattern daily-triage --level L1`
- Kill switch: `loop-pause-all: true` in `STATE.md` (loop skill reads this and exits)
- Hard cap for unattended runs: 1M tokens/day across all loops on this repo

## Safety & Gates

- No auto-merge on main — ever. All loop-proposed changes land as draft PRs.
- Denylist paths (defined in `loop-constraints.md`): `secrets/`, `auth/`, `migrations/`, `Cargo.toml` major bumps, `deny.toml` policy changes without human review.
- Live loop state lives in `STATE.md` at repo root (gitignored from public commits; committed only for personal projects where state history is valuable).
- Verifier default stance: REJECT until tests pass in worktree.

## How to run locally

```bash
# Manually trigger a triage run (before scheduling is set up)
cd /Users/pirateape/Documents/src/ApeGuard
opencode run \
  "Run the loop-triage skill. Read STATE.md first. Append high-priority items under High Priority and Watch List. Update Last run timestamp. Do not edit source code. End with a 5-line summary." \
  --title "Daily triage — ApeGuard"

# After scheduling is set up, the launchd plist handles this automatically.
launchctl list | grep apeguard   # check scheduling
tail -n 50 loop-run-log.md       # read recent runs

# Audit readiness
npx @cobusgreyling/loop-audit . --suggest
```

## Evolution

Journey recorded in `loop-run-log.md`. Target: solid L2 with excellent observability, then L3 only after sustained high-quality triage output and trusted verifier behavior.

---

_This file is both documentation and the seed for the loops that maintain ApeGuard._

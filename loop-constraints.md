# Loop Constraints — ApeGuard

> The `loop-constraints` skill reads this file at the start of every loop run.
> Constraints here are **binding** — the agent MUST follow them.
> Edit directly or invoke: `opencode run "Append this rule to loop-constraints.md verbatim: '<rule>'"`.

## Push & Merge

- Don't push before telling me
- Never auto-merge to `main` without explicit human approval
- Always create a draft PR first; let me review before marking ready
- Never force-push to `main` or `release/*` branches

## Paths

- Never edit `.env`, `.env.*`, `.secrets.baseline`, `.apeguard/` config without explicit human approval
- Never edit `auth/`, `payments/`, `secrets/`, `credentials/`, `migrations/` (none exist yet but reserve)
- Never edit `deny.toml` policy changes without human review (supply-chain risk)
- Never edit `Cargo.toml` for major dependency version bumps without human approval
- Never edit `release.yml` workflow or `update-homebrew-tap.yml` without human approval
- Never edit `.pre-commit-config.yaml` without human review (CI safety gates)

## Code

- Always run `cargo test` before proposing a fix
- Always run `cargo clippy -- -D warnings` before proposing a fix
- Always run `cargo fmt --check` before proposing a fix
- Never disable tests to make CI green
- Never refactor unrelated code — one fix per run
- Max 3 fix attempts per item; escalate after
- Enforce the attempt limit mechanically: log each try to `loop-ledger.json` and run `loop-guard` before retrying
- Use `expect()` only in test code; production paths must use `Result` types
- Never `unwrap()` in production code paths

## Communication

- Always tell me what you're about to do before doing it
- Never close an issue or PR without my approval
- Loop-produced PR comments must be clearly signed: "🤖 ApeGuard Loop — <pattern-name>"
- Escalation messages must include: failing job, attempt count, worktree path, diff path

## Budget

- Daily token cap: 1M tokens across all loops on this repo
- If token spend hits 80% of daily cap (800k), switch to report-only mode
- If `loop-pause-all: true` in STATE.md, exit immediately without action
- Max sub-agent spawns per run: 3 (L1 = 0)
- Empty watchlist → exit in <5k tokens (early exit required)

## ApeGuard-Specific

- Treat the 8 scanner layers as orthogonal — never collapse or merge layers without design discussion
- The canonical `CanonicalFinding` struct in `src/find/mod.rs` is the contract — do not change its public fields without a migration story
- The pipeline order is sacred: `scan → normalize → dedup → REACHABILITY → FP FILTER → LLM → GRADE → severity → POLICY → chains → SCORE → ZT → STRIDE → report`. Any insertion must be discussed.
- ApeGuard's own scan output (`apeguard scan --format sarif`) is the authoritative CI-status read for triage — prefer it over parsing GitHub Actions logs when available
- Release versioning follows `vX.Y.Z` tags with 4-platform binary build + Homebrew tap update (see `release.yml`)

---

<!-- Add your own rules below. Use plain English. The loop reads this verbatim. -->

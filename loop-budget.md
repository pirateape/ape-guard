# Loop Budget — ApeGuard

> Cost caps and kill switch for loops operating on this repository.

## Daily Limits

| Loop               | Max runs/day | Max tokens/day | Max sub-agent spawns/run |
| ------------------ | ------------ | -------------- | ------------------------ |
| Daily Triage       | 1            | 100k           | 0 (L1 report-only)       |
| Dependency Sweeper | 4            | 250k           | 2 (future L2)            |
| CI Sweeper         | 96           | 1M             | 3 (future L2)            |
| Post-Merge Cleanup | 1            | 100k           | 2 (future L2)            |

### Aggregate cap

- **Total across all loops: 1M tokens/day** (hard ceiling)
- On exceed: pause all non-triage schedulers, append event to `loop-run-log.md`, open human issue in `STATE.md`

## On Budget Exceed

1. Pause schedulers: `launchctl unload ~/Library/LaunchAgents/com.apeguard.loop-*.plist`
2. Append event to `loop-run-log.md` with offending loop name + token estimate
3. Append to `STATE.md` → "High Priority (waiting on human)" section: "Loop budget exceeded on <date>" with the cause
4. Resume only after explicit human review

## Kill Switch

- Flag: `loop-pause-all: true` in `STATE.md` (read by `loop-triage` skill at start of every run)
- Resume only after cleared (`loop-pause-all: false`) in `STATE.md` by human
- Also disable launchd plist: `launchctl unload ~/Library/LaunchAgents/com.apeguard.loop-daily-triage.plist`

## Estimate Spend

```bash
npx @cobusgreyling/loop-cost --pattern daily-triage --level L1 --cadence 1d
npx @cobusgreyling/loop-cost --pattern dependency-sweeper --level L2 --cadence 6h
npx @cobusgreyling/loop-cost --pattern ci-sweeper --level L2 --cadence 15m
```

## Notes

- ApeGuard's local Ollama-backed scanning already runs at ~$0 token cost for `apeguard scan` itself — the budget above covers only the **orchestrating agent** (opencode runs), not the scanner binaries.
- The 1M/day cap is conservative for week 1 L1-only operation (actual: ~50k/day). Loosen incrementally as trust in triage quality grows.
- CI Sweeper at 15m cadence can burn 1M+ tokens/day in the worst case — only enable after Dependency Sweeper has proven trustworthy.

## weekly_cost_log

| Week starting       | estimated_tokens | actual_tokens | under_budget | notes          |
| ------------------- | ---------------- | ------------- | ------------ | -------------- |
| (first run pending) | —                | —             | —            | L1 report-only |

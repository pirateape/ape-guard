---
type: Rust Function
title: handle_cache
resource: src/main.rs#L640-L686
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/stats
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/prune
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_cache( subcommand: &cli::CacheSubcommand, cfg: &config::Config, quiet: bool, ) -> anyhow::Result<()>`

# Calls

- [open](../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../functions/src/cache/ScanCache/enforce_ttl.md)
- [stats](../../functions/src/cache/ScanCache/stats.md)
- [prune](../../functions/src/cache/ScanCache/prune.md)

# Called by

- [main](../../functions/src/main.md)
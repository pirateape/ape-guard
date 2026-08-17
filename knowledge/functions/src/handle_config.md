---
type: Rust Function
title: handle_config
resource: src/main.rs#L615-L637
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn handle_config( subcommand: &Option<cli::ConfigSubcommand>, cfg: &config::Config, ) -> anyhow::Result<()>`

# Called by

- [main](../../functions/src/main.md)
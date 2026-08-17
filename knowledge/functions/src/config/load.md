---
type: Rust Function
title: load
resource: src/config.rs#L265-L329
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/merge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/Command/is_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load(args: &cli::Args) -> anyhow::Result<Config>`

# Calls

- [merge](../../../functions/src/config/merge.md)
- [parse](../../../functions/src/cli/parse.md)
- [is_scan](../../../functions/src/cli/Command/is_scan.md)
- [default](../../../functions/src/config/Config/default/default.md)

# Called by

- [main](../../../functions/src/main.md)
- [load_effective_config](../../../functions/src/mcp/load_effective_config.md)
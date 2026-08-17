---
type: Rust Function
title: parse
resource: src/cli.rs#L247-L249
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse() -> Args`

# Called by

- [load](../../../functions/src/config/load.md)
- [main](../../../functions/src/main.md)
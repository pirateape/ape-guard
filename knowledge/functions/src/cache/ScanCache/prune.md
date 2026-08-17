---
type: Rust Method
title: prune
resource: src/cache.rs#L191-L229
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/handle_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn prune(&self, keep: u32) -> anyhow::Result<u64>`

# Called by

- [handle_cache](../../../../functions/src/handle_cache.md)
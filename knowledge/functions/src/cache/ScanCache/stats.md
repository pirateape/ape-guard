---
type: Rust Method
title: stats
resource: src/cache.rs#L157-L188
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/handle_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn stats(&self) -> anyhow::Result<CacheStats>`

# Called by

- [handle_cache](../../../../functions/src/handle_cache.md)
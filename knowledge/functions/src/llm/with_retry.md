---
type: Rust Function
title: with_retry
resource: src/llm.rs#L100-L124
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/llm/call_ollama
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn with_retry<R, F, Fut>(max_retries: u32, op: F) -> anyhow::Result<R> where F: Fn() -> Fut, Fut: std::future::Future<Output = anyhow::Result<R>>,`

# Called by

- [call_ollama](../../../functions/src/llm/call_ollama.md)
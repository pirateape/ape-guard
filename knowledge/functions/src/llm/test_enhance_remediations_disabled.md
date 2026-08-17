---
type: Rust Function
title: test_enhance_remediations_disabled
resource: src/llm.rs#L181-L218
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/llm/enhance_remediations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/LlmConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`async fn test_enhance_remediations_disabled()`

# Calls

- [enhance_remediations](../../../functions/src/llm/enhance_remediations.md)
- [default](../../../functions/src/llm/LlmConfig/default/default.md)
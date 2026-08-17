---
type: Rust Function
title: call_ollama
resource: src/llm.rs#L126-L164
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/llm/with_retry
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/grade/grade_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/enhance_remediations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn call_ollama( endpoint: &str, model: &str, prompt: &str, ) -> anyhow::Result<String>`

# Calls

- [with_retry](../../../functions/src/llm/with_retry.md)

# Called by

- [grade_findings](../../../functions/src/grade/grade_findings.md)
- [enhance_remediations](../../../functions/src/llm/enhance_remediations.md)
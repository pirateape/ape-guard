---
type: Rust Function
title: enhance_remediations
resource: src/llm.rs#L28-L80
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/llm/build_remediation_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/call_ollama
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/test_enhance_remediations_disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn enhance_remediations( findings: &mut [CanonicalFinding], config: &LlmConfig, ) -> anyhow::Result<u32>`

# Calls

- [build_remediation_prompt](../../../functions/src/llm/build_remediation_prompt.md)
- [call_ollama](../../../functions/src/llm/call_ollama.md)

# Called by

- [test_enhance_remediations_disabled](../../../functions/src/llm/test_enhance_remediations_disabled.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
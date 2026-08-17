---
type: Rust Function
title: grade_findings
resource: src/grade.rs#L17-L66
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/grade/build_grade_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/call_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/parse_grade_response
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn grade_findings( findings: &mut [CanonicalFinding], endpoint: &str, model: &str, ) -> anyhow::Result<u32>`

# Calls

- [build_grade_prompt](../../../functions/src/grade/build_grade_prompt.md)
- [call_ollama](../../../functions/src/llm/call_ollama.md)
- [parse_grade_response](../../../functions/src/grade/parse_grade_response.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
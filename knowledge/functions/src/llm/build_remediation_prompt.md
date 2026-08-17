---
type: Rust Function
title: build_remediation_prompt
resource: src/llm.rs#L82-L95
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/llm/enhance_remediations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/test_build_remediation_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_remediation_prompt(finding: &CanonicalFinding) -> String`

# Called by

- [enhance_remediations](../../../functions/src/llm/enhance_remediations.md)
- [test_build_remediation_prompt](../../../functions/src/llm/test_build_remediation_prompt.md)
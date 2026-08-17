---
type: Rust Function
title: build_grade_prompt
resource: src/grade.rs#L73-L122
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/grade/grade_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_build_grade_prompt_includes_finding
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_grade_prompt(finding: &CanonicalFinding) -> String`

# Called by

- [grade_findings](../../../functions/src/grade/grade_findings.md)
- [test_build_grade_prompt_includes_finding](../../../functions/src/grade/test_build_grade_prompt_includes_finding.md)
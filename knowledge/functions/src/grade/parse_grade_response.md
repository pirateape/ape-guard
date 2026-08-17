---
type: Rust Function
title: parse_grade_response
resource: src/grade.rs#L144-L207
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/grade/parse_reject_reason
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/grade/grade_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_confirmed_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_rejected_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_needs_review_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_markdown_fenced_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_fallback_keyword_confirmed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_fallback_keyword_rejected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/test_parse_unknown_verdict
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_grade_response(response: &str) -> GradeVerdict`

# Calls

- [parse_reject_reason](../../../functions/src/grade/parse_reject_reason.md)

# Called by

- [grade_findings](../../../functions/src/grade/grade_findings.md)
- [test_parse_confirmed_json](../../../functions/src/grade/test_parse_confirmed_json.md)
- [test_parse_rejected_json](../../../functions/src/grade/test_parse_rejected_json.md)
- [test_parse_needs_review_json](../../../functions/src/grade/test_parse_needs_review_json.md)
- [test_parse_markdown_fenced_json](../../../functions/src/grade/test_parse_markdown_fenced_json.md)
- [test_parse_fallback_keyword_confirmed](../../../functions/src/grade/test_parse_fallback_keyword_confirmed.md)
- [test_parse_fallback_keyword_rejected](../../../functions/src/grade/test_parse_fallback_keyword_rejected.md)
- [test_parse_unknown_verdict](../../../functions/src/grade/test_parse_unknown_verdict.md)
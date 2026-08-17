---
type: Rust Module
title: grade
resource: src/grade.rs#L1-L508
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-gradeverdict-rejectreason
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [grade_findings](../../functions/src/grade/grade_findings.md)
- [build_grade_prompt](../../functions/src/grade/build_grade_prompt.md)
- [GradeResponse](../../classes/src/grade/GradeResponse.md)
- [parse_grade_response](../../functions/src/grade/parse_grade_response.md)
- [parse_reject_reason](../../functions/src/grade/parse_reject_reason.md)
- [count_verdicts](../../functions/src/grade/count_verdicts.md)
- [GradeCounts](../../classes/src/grade/GradeCounts.md)
- [make_finding](../../functions/src/grade/make_finding.md)
- [test_build_grade_prompt_includes_finding](../../functions/src/grade/test_build_grade_prompt_includes_finding.md)
- [test_parse_confirmed_json](../../functions/src/grade/test_parse_confirmed_json.md)
- [test_parse_rejected_json](../../functions/src/grade/test_parse_rejected_json.md)
- [test_parse_needs_review_json](../../functions/src/grade/test_parse_needs_review_json.md)
- [test_parse_markdown_fenced_json](../../functions/src/grade/test_parse_markdown_fenced_json.md)
- [test_parse_fallback_keyword_confirmed](../../functions/src/grade/test_parse_fallback_keyword_confirmed.md)
- [test_parse_fallback_keyword_rejected](../../functions/src/grade/test_parse_fallback_keyword_rejected.md)
- [test_parse_unknown_verdict](../../functions/src/grade/test_parse_unknown_verdict.md)
- [test_count_verdicts_all_types](../../functions/src/grade/test_count_verdicts_all_types.md)
- [test_parse_reject_reason_all_variants](../../functions/src/grade/test_parse_reject_reason_all_variants.md)
- [test_grade_verdict_serialize_roundtrip](../../functions/src/grade/test_grade_verdict_serialize_roundtrip.md)
- [test_grade_field_on_canonical_finding](../../functions/src/grade/test_grade_field_on_canonical_finding.md)

# Imports

- `crate::find::{CanonicalFinding, GradeVerdict, RejectReason}`
- `serde::Deserialize`
- `super::*`
- `crate::find::*`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
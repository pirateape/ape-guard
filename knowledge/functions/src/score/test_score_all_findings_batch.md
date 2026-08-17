---
type: Rust Function
title: test_score_all_findings_batch
resource: src/score.rs#L786-L837
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/score_all_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/ScoreWeights/default/default
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_score_all_findings_batch()`

# Calls

- [score_all_findings](../../../functions/src/score/score_all_findings.md)
- [default](../../../functions/src/score/ScoreWeights/default/default.md)
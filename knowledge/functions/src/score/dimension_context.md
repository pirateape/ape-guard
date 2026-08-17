---
type: Rust Function
title: dimension_context
resource: src/score.rs#L151-L226
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_test_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_crypto_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_auth_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_api_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_util_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_config_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_context_default
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dimension_context(finding: &CanonicalFinding) -> f32`

# Called by

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)
- [test_context_test_dir](../../../functions/src/score/test_context_test_dir.md)
- [test_context_crypto_dir](../../../functions/src/score/test_context_crypto_dir.md)
- [test_context_auth_dir](../../../functions/src/score/test_context_auth_dir.md)
- [test_context_api_dir](../../../functions/src/score/test_context_api_dir.md)
- [test_context_util_dir](../../../functions/src/score/test_context_util_dir.md)
- [test_context_config_dir](../../../functions/src/score/test_context_config_dir.md)
- [test_context_default](../../../functions/src/score/test_context_default.md)
---
type: Rust Function
title: make_finding
resource: src/score.rs#L426-L476
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/test_confidence_no_cross_refs
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_confidence_with_cross_refs
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_confidence_rejected_by_ai
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_confidence_confirmed_by_ai
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_test_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_crypto_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_auth_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_api_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_util_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_config_dir
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_context_default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_chain_not_in_any
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_chain_in_one
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_chain_in_two
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_zt_zero_pillars
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_zt_one_pillar
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_zt_two_pillars
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_zt_three_pillars
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_critical_finding_scores_high
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_rejected_low_finding_scores_low
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_chain_finding_boosted
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn make_finding( id: &str, severity: Severity, file: &str, cross_refs: usize, zt_count: usize, grade: Option<GradeVerdict>, ) -> CanonicalFinding`

# Called by

- [test_confidence_no_cross_refs](../../../functions/src/score/test_confidence_no_cross_refs.md)
- [test_confidence_with_cross_refs](../../../functions/src/score/test_confidence_with_cross_refs.md)
- [test_confidence_rejected_by_ai](../../../functions/src/score/test_confidence_rejected_by_ai.md)
- [test_confidence_confirmed_by_ai](../../../functions/src/score/test_confidence_confirmed_by_ai.md)
- [test_context_test_dir](../../../functions/src/score/test_context_test_dir.md)
- [test_context_crypto_dir](../../../functions/src/score/test_context_crypto_dir.md)
- [test_context_auth_dir](../../../functions/src/score/test_context_auth_dir.md)
- [test_context_api_dir](../../../functions/src/score/test_context_api_dir.md)
- [test_context_util_dir](../../../functions/src/score/test_context_util_dir.md)
- [test_context_config_dir](../../../functions/src/score/test_context_config_dir.md)
- [test_context_default](../../../functions/src/score/test_context_default.md)
- [test_chain_not_in_any](../../../functions/src/score/test_chain_not_in_any.md)
- [test_chain_in_one](../../../functions/src/score/test_chain_in_one.md)
- [test_chain_in_two](../../../functions/src/score/test_chain_in_two.md)
- [test_zt_zero_pillars](../../../functions/src/score/test_zt_zero_pillars.md)
- [test_zt_one_pillar](../../../functions/src/score/test_zt_one_pillar.md)
- [test_zt_two_pillars](../../../functions/src/score/test_zt_two_pillars.md)
- [test_zt_three_pillars](../../../functions/src/score/test_zt_three_pillars.md)
- [test_critical_finding_scores_high](../../../functions/src/score/test_critical_finding_scores_high.md)
- [test_rejected_low_finding_scores_low](../../../functions/src/score/test_rejected_low_finding_scores_low.md)
- [test_chain_finding_boosted](../../../functions/src/score/test_chain_finding_boosted.md)
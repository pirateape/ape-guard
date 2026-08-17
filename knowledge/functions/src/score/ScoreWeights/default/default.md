---
type: Rust Method
title: default
resource: src/score.rs#L29-L38
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
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
  - target: functions/src/score/test_score_all_findings_batch
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_scan_health_with_findings
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_scan_health_bounds
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_scanner_risk_per_scanner
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_score_weights_default_total
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/score/test_score_weights_custom
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_critical_finding_scores_high](../../../../../functions/src/score/test_critical_finding_scores_high.md)
- [test_rejected_low_finding_scores_low](../../../../../functions/src/score/test_rejected_low_finding_scores_low.md)
- [test_chain_finding_boosted](../../../../../functions/src/score/test_chain_finding_boosted.md)
- [test_score_all_findings_batch](../../../../../functions/src/score/test_score_all_findings_batch.md)
- [test_scan_health_with_findings](../../../../../functions/src/score/test_scan_health_with_findings.md)
- [test_scan_health_bounds](../../../../../functions/src/score/test_scan_health_bounds.md)
- [test_scanner_risk_per_scanner](../../../../../functions/src/score/test_scanner_risk_per_scanner.md)
- [test_score_weights_default_total](../../../../../functions/src/score/test_score_weights_default_total.md)
- [test_score_weights_custom](../../../../../functions/src/score/test_score_weights_custom.md)
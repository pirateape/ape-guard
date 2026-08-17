---
type: Rust Function
title: score_all_findings
resource: src/score.rs#L62-L73
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/score/compute_finding_risk
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_score_all_findings_batch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_scan_health_with_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_scan_health_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/test_scanner_risk_per_scanner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn score_all_findings( findings: &mut [CanonicalFinding], chains: &[AttackChain], weights: &ScoreWeights, )`

# Calls

- [compute_finding_risk](../../../functions/src/score/compute_finding_risk.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
- [test_score_all_findings_batch](../../../functions/src/score/test_score_all_findings_batch.md)
- [test_scan_health_with_findings](../../../functions/src/score/test_scan_health_with_findings.md)
- [test_scan_health_bounds](../../../functions/src/score/test_scan_health_bounds.md)
- [test_scanner_risk_per_scanner](../../../functions/src/score/test_scanner_risk_per_scanner.md)
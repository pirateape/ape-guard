---
type: Rust Function
title: compute_scan_health
resource: src/score.rs#L342-L416
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/score/test_scan_health_no_findings
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

`pub fn compute_scan_health( findings: &[CanonicalFinding], scanners_used: &[String], zt_maturity: u32, ) -> ScanHealthScore`

# Called by

- [test_scan_health_no_findings](../../../functions/src/score/test_scan_health_no_findings.md)
- [test_scan_health_with_findings](../../../functions/src/score/test_scan_health_with_findings.md)
- [test_scan_health_bounds](../../../functions/src/score/test_scan_health_bounds.md)
- [test_scanner_risk_per_scanner](../../../functions/src/score/test_scanner_risk_per_scanner.md)
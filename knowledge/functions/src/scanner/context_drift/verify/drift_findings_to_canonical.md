---
type: Rust Function
title: drift_findings_to_canonical
resource: src/scanner/context_drift/verify.rs#L656-L720
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_drift_findings_to_canonical_conversion
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn drift_findings_to_canonical(findings: &[DriftFinding]) -> Vec<CanonicalFinding>`

# Called by

- [run_scan](../../../../../functions/src/orchestrate/run_scan.md)
- [test_drift_findings_to_canonical_conversion](../../../../../functions/src/scanner/context_drift/test_drift_findings_to_canonical_conversion.md)
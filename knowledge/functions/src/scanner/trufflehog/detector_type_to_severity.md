---
type: Rust Function
title: detector_type_to_severity
resource: src/scanner/trufflehog.rs#L381-L440
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/trufflehog/parse_trufflehog_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trufflehog/test_detector_type_severity
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detector_type_to_severity(detector_type: i32, verified: bool) -> (Severity, String)`

# Called by

- [parse_trufflehog_line](../../../../functions/src/scanner/trufflehog/parse_trufflehog_line.md)
- [test_detector_type_severity](../../../../functions/src/scanner/trufflehog/test_detector_type_severity.md)
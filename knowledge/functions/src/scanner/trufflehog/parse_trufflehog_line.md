---
type: Rust Function
title: parse_trufflehog_line
resource: src/scanner/trufflehog.rs#L183-L372
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trufflehog/detector_type_to_severity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trufflehog/Trufflehog/parse_json_array
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_trufflehog_line(line: &str) -> Result<Option<CanonicalFinding>, ScannerError>`

# Calls

- [detector_type_to_severity](../../../../functions/src/scanner/trufflehog/detector_type_to_severity.md)

# Called by

- [parse_output](../../../../functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output.md)
- [parse_json_array](../../../../functions/src/scanner/trufflehog/Trufflehog/parse_json_array.md)
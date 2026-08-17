---
type: Rust Method
title: parse_json_array
resource: src/scanner/trufflehog.rs#L157-L179
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trufflehog/parse_trufflehog_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_json_array(arr: &[serde_json::Value]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Calls

- [parse_trufflehog_line](../../../../../functions/src/scanner/trufflehog/parse_trufflehog_line.md)

# Called by

- [parse_output](../../../../../functions/src/scanner/trufflehog/Trufflehog/scanner/parse_output.md)
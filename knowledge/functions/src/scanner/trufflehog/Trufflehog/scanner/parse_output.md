---
type: Rust Method
title: parse_output
resource: src/scanner/trufflehog.rs#L93-L152
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trufflehog/Trufflehog/parse_json_array
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trufflehog/parse_trufflehog_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/trufflehog/test_parse_output_single_finding
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_unverified
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_multiple_lines
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_empty
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_whitespace_only
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_git_source
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_parse_output_skip_no_secret
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Calls

- [parse_json_array](../../../../../../functions/src/scanner/trufflehog/Trufflehog/parse_json_array.md)
- [parse_trufflehog_line](../../../../../../functions/src/scanner/trufflehog/parse_trufflehog_line.md)

# Called by

- [test_parse_output_single_finding](../../../../../../functions/src/scanner/trufflehog/test_parse_output_single_finding.md)
- [test_parse_output_unverified](../../../../../../functions/src/scanner/trufflehog/test_parse_output_unverified.md)
- [test_parse_output_multiple_lines](../../../../../../functions/src/scanner/trufflehog/test_parse_output_multiple_lines.md)
- [test_parse_output_empty](../../../../../../functions/src/scanner/trufflehog/test_parse_output_empty.md)
- [test_parse_output_whitespace_only](../../../../../../functions/src/scanner/trufflehog/test_parse_output_whitespace_only.md)
- [test_parse_output_git_source](../../../../../../functions/src/scanner/trufflehog/test_parse_output_git_source.md)
- [test_parse_output_skip_no_secret](../../../../../../functions/src/scanner/trufflehog/test_parse_output_skip_no_secret.md)
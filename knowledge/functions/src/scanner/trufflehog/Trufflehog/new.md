---
type: Rust Method
title: new
resource: src/scanner/trufflehog.rs#L17-L21
generated:
  by: okf-rs/0.4.0
relationships:
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
  - target: functions/src/scanner/trufflehog/test_scanner_name
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_scanner_type
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/test_install_hint
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [test_parse_output_single_finding](../../../../../functions/src/scanner/trufflehog/test_parse_output_single_finding.md)
- [test_parse_output_unverified](../../../../../functions/src/scanner/trufflehog/test_parse_output_unverified.md)
- [test_parse_output_multiple_lines](../../../../../functions/src/scanner/trufflehog/test_parse_output_multiple_lines.md)
- [test_parse_output_empty](../../../../../functions/src/scanner/trufflehog/test_parse_output_empty.md)
- [test_parse_output_whitespace_only](../../../../../functions/src/scanner/trufflehog/test_parse_output_whitespace_only.md)
- [test_parse_output_git_source](../../../../../functions/src/scanner/trufflehog/test_parse_output_git_source.md)
- [test_parse_output_skip_no_secret](../../../../../functions/src/scanner/trufflehog/test_parse_output_skip_no_secret.md)
- [test_scanner_name](../../../../../functions/src/scanner/trufflehog/test_scanner_name.md)
- [test_scanner_type](../../../../../functions/src/scanner/trufflehog/test_scanner_type.md)
- [test_install_hint](../../../../../functions/src/scanner/trufflehog/test_install_hint.md)
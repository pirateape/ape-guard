---
type: Rust Method
title: parse_output
resource: src/scanner/tls.rs#L111-L225
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/tls/test_parse_output_manual_inspection
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_empty_input
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_expired_path
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_cert_found_path
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_not_found_path
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_ok_response
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/test_parse_output_no_relevant_lines
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_manual_inspection](../../../../../../functions/src/scanner/tls/test_parse_output_manual_inspection.md)
- [test_parse_output_empty_input](../../../../../../functions/src/scanner/tls/test_parse_output_empty_input.md)
- [test_parse_output_expired_path](../../../../../../functions/src/scanner/tls/test_parse_output_expired_path.md)
- [test_parse_output_cert_found_path](../../../../../../functions/src/scanner/tls/test_parse_output_cert_found_path.md)
- [test_parse_output_not_found_path](../../../../../../functions/src/scanner/tls/test_parse_output_not_found_path.md)
- [test_parse_output_ok_response](../../../../../../functions/src/scanner/tls/test_parse_output_ok_response.md)
- [test_parse_output_no_relevant_lines](../../../../../../functions/src/scanner/tls/test_parse_output_no_relevant_lines.md)
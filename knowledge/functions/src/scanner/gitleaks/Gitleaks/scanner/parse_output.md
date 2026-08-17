---
type: Rust Method
title: parse_output
resource: src/scanner/gitleaks.rs#L111-L189
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/gitleaks/test_parse_output_real_fixture
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/gitleaks/test_parse_output_empty_array
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/gitleaks/test_parse_output_empty_bytes
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/gitleaks/test_parse_output_single_object_fallback
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_real_fixture](../../../../../../functions/src/scanner/gitleaks/test_parse_output_real_fixture.md)
- [test_parse_output_empty_array](../../../../../../functions/src/scanner/gitleaks/test_parse_output_empty_array.md)
- [test_parse_output_empty_bytes](../../../../../../functions/src/scanner/gitleaks/test_parse_output_empty_bytes.md)
- [test_parse_output_single_object_fallback](../../../../../../functions/src/scanner/gitleaks/test_parse_output_single_object_fallback.md)
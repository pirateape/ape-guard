---
type: Rust Method
title: parse_output
resource: src/scanner/syft.rs#L66-L151
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/syft/test_parse_output_real_fixture
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/syft/test_parse_output_empty
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Called by

- [test_parse_output_real_fixture](../../../../../../functions/src/scanner/syft/test_parse_output_real_fixture.md)
- [test_parse_output_empty](../../../../../../functions/src/scanner/syft/test_parse_output_empty.md)
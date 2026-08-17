---
type: Rust Method
title: new
resource: src/scanner/syft.rs#L15-L19
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

`pub fn new() -> Self`

# Called by

- [test_parse_output_real_fixture](../../../../../functions/src/scanner/syft/test_parse_output_real_fixture.md)
- [test_parse_output_empty](../../../../../functions/src/scanner/syft/test_parse_output_empty.md)
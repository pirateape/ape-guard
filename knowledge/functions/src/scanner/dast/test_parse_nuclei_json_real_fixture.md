---
type: Rust Function
title: test_parse_nuclei_json_real_fixture
resource: src/scanner/dast.rs#L248-L283
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/dast/DastScanner/parse_nuclei_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/DastScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn test_parse_nuclei_json_real_fixture()`

# Calls

- [parse_nuclei_json](../../../../functions/src/scanner/dast/DastScanner/parse_nuclei_json.md)
- [new](../../../../functions/src/scanner/dast/DastScanner/new.md)
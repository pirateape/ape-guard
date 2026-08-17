---
type: Rust Function
title: test_parse_nuclei_json_severity_from_info_block
resource: src/scanner/dast.rs#L305-L314
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

`fn test_parse_nuclei_json_severity_from_info_block()`

# Calls

- [parse_nuclei_json](../../../../functions/src/scanner/dast/DastScanner/parse_nuclei_json.md)
- [new](../../../../functions/src/scanner/dast/DastScanner/new.md)
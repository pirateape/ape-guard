---
type: Rust Function
title: is_claim_line
resource: src/scanner/context_drift/parse.rs#L11-L77
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/parse/extract_claim_from_line
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_claim_line(line: &str) -> bool`

# Called by

- [extract_claim_from_line](../../../../../functions/src/scanner/context_drift/parse/extract_claim_from_line.md)
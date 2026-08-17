---
type: Rust Function
title: scanner_is
resource: src/chain.rs#L44-L46
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/is_misconfig
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scanner_is(scanners: &'static [ScannerType]) -> impl Fn(&CanonicalFinding) -> bool`

# Called by

- [is_misconfig](../../../functions/src/chain/is_misconfig.md)
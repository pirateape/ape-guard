---
type: Rust Function
title: dep_pattern
resource: src/scanner/context_drift/discover.rs#L8-L15
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/parse/classify_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn dep_pattern() -> &'static Regex`

# Called by

- [classify_claim](../../../../../functions/src/scanner/context_drift/parse/classify_claim.md)
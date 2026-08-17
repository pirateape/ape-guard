---
type: Rust Function
title: path_pattern
resource: src/scanner/context_drift/discover.rs#L25-L31
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_path_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn path_pattern() -> &'static Regex`

# Called by

- [verify_path_claim](../../../../../functions/src/scanner/context_drift/verify/verify_path_claim.md)
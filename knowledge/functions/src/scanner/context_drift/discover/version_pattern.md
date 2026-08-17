---
type: Rust Function
title: version_pattern
resource: src/scanner/context_drift/discover.rs#L17-L23
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/verify/extract_dep_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn version_pattern() -> &'static Regex`

# Called by

- [extract_dep_name](../../../../../functions/src/scanner/context_drift/verify/extract_dep_name.md)
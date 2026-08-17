---
type: Rust Function
title: extract_quoted_string
resource: src/reachability.rs#L532-L541
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/extract_js_imports
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_quoted_string(s: &str) -> Option<String>`

# Called by

- [extract_js_imports](../../../functions/src/reachability/extract_js_imports.md)
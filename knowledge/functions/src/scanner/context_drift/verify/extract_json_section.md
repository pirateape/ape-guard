---
type: Rust Function
title: extract_json_section
resource: src/scanner/context_drift/verify.rs#L150-L179
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/test_extract_json_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/has_dependency
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn extract_json_section(json: &str, key: &str) -> Option<String>`

# Called by

- [test_extract_json_section](../../../../../functions/src/scanner/context_drift/test_extract_json_section.md)
- [has_dependency](../../../../../functions/src/scanner/context_drift/verify/DependencyCache/has_dependency.md)
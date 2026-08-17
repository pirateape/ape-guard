---
type: Rust Method
title: has_dependency
resource: src/scanner/context_drift/verify.rs#L87-L145
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/verify/extract_json_section
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/verify/verify_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn has_dependency( &self, dep_name: &str, version_hint: Option<&str>, ) -> Option<String>`

# Calls

- [extract_json_section](../../../../../../functions/src/scanner/context_drift/verify/extract_json_section.md)

# Called by

- [verify_dependency_claim](../../../../../../functions/src/scanner/context_drift/verify/verify_dependency_claim.md)
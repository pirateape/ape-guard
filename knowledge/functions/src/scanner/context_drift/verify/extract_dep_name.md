---
type: Rust Function
title: extract_dep_name
resource: src/scanner/context_drift/verify.rs#L182-L196
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/technology_keyword_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/discover/version_pattern
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_extract_dep_name_technology
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn extract_dep_name(text: &str) -> Option<(String, Option<String>)>`

# Calls

- [technology_keyword_pattern](../../../../../functions/src/scanner/context_drift/discover/technology_keyword_pattern.md)
- [version_pattern](../../../../../functions/src/scanner/context_drift/discover/version_pattern.md)

# Called by

- [test_extract_dep_name_technology](../../../../../functions/src/scanner/context_drift/test_extract_dep_name_technology.md)
- [verify_dependency_claim](../../../../../functions/src/scanner/context_drift/verify/verify_dependency_claim.md)
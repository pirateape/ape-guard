---
type: Rust Function
title: search_for_technology_usage
resource: src/scanner/context_drift/verify.rs#L389-L489
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/read_file
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_technology_search_rust
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_architecture_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn search_for_technology_usage(tech: &str, root: &Path) -> Option<String>`

# Calls

- [read_file](../../../../../functions/src/scanner/context_drift/discover/read_file.md)

# Called by

- [test_technology_search_rust](../../../../../functions/src/scanner/context_drift/test_technology_search_rust.md)
- [verify_architecture_claim](../../../../../functions/src/scanner/context_drift/verify/verify_architecture_claim.md)
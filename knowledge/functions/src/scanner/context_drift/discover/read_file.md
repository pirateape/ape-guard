---
type: Rust Function
title: read_file
resource: src/scanner/context_drift/discover.rs#L79-L81
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/parse/parse_context_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/DependencyCache/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/search_for_technology_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_security_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn read_file(path: &Path) -> Option<String>`

# Called by

- [parse_context_file](../../../../../functions/src/scanner/context_drift/parse/parse_context_file.md)
- [new](../../../../../functions/src/scanner/context_drift/verify/DependencyCache/new.md)
- [search_for_technology_usage](../../../../../functions/src/scanner/context_drift/verify/search_for_technology_usage.md)
- [verify_security_claim](../../../../../functions/src/scanner/context_drift/verify/verify_security_claim.md)
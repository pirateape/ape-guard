---
type: Rust Function
title: technology_keyword_pattern
resource: src/scanner/context_drift/discover.rs#L33-L38
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/parse/classify_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/extract_dep_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/verify_architecture_claim
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn technology_keyword_pattern() -> &'static Regex`

# Called by

- [classify_claim](../../../../../functions/src/scanner/context_drift/parse/classify_claim.md)
- [extract_dep_name](../../../../../functions/src/scanner/context_drift/verify/extract_dep_name.md)
- [verify_architecture_claim](../../../../../functions/src/scanner/context_drift/verify/verify_architecture_claim.md)
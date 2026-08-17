---
type: Rust Function
title: classify_claim
resource: src/scanner/context_drift/parse.rs#L121-L207
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/dep_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/discover/technology_keyword_pattern
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/parse/extract_claim_from_line
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn classify_claim(text: &str, lower: &str, section: Option<&str>) -> ClaimCategory`

# Calls

- [dep_pattern](../../../../../functions/src/scanner/context_drift/discover/dep_pattern.md)
- [technology_keyword_pattern](../../../../../functions/src/scanner/context_drift/discover/technology_keyword_pattern.md)

# Called by

- [extract_claim_from_line](../../../../../functions/src/scanner/context_drift/parse/extract_claim_from_line.md)
---
type: Rust Function
title: parse_cursor_rules
resource: src/scanner/context_drift/parse.rs#L263-L329
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/parse/extract_claim_from_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_parse_cursor_rules_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_context_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn parse_cursor_rules( content: &str, source_path: &Path, file_type: ContextFileType, ) -> Vec<ContextClaim>`

# Calls

- [extract_claim_from_line](../../../../../functions/src/scanner/context_drift/parse/extract_claim_from_line.md)

# Called by

- [test_parse_cursor_rules_frontmatter](../../../../../functions/src/scanner/context_drift/test_parse_cursor_rules_frontmatter.md)
- [parse_context_file](../../../../../functions/src/scanner/context_drift/parse/parse_context_file.md)
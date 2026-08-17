---
type: Rust Function
title: extract_claim_from_line
resource: src/scanner/context_drift/parse.rs#L80-L118
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/parse/is_claim_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/classify_claim
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/parse/parse_agents_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_cursor_rules
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_claim_from_line( line: &str, line_number: u32, source_path: &Path, file_type: ContextFileType, current_section: Option<&str>, ) -> Option<ContextClaim>`

# Calls

- [is_claim_line](../../../../../functions/src/scanner/context_drift/parse/is_claim_line.md)
- [classify_claim](../../../../../functions/src/scanner/context_drift/parse/classify_claim.md)

# Called by

- [parse_agents_md](../../../../../functions/src/scanner/context_drift/parse/parse_agents_md.md)
- [parse_cursor_rules](../../../../../functions/src/scanner/context_drift/parse/parse_cursor_rules.md)
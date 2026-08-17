---
type: Rust Function
title: parse_agents_md
resource: src/scanner/context_drift/parse.rs#L210-L248
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/parse/extract_claim_from_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_parse_agents_md_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_empty_context_file_no_claims
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_claude_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_context_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn parse_agents_md( content: &str, source_path: &Path, file_type: ContextFileType, ) -> Vec<ContextClaim>`

# Calls

- [extract_claim_from_line](../../../../../functions/src/scanner/context_drift/parse/extract_claim_from_line.md)

# Called by

- [test_parse_agents_md_dependency_claim](../../../../../functions/src/scanner/context_drift/test_parse_agents_md_dependency_claim.md)
- [test_empty_context_file_no_claims](../../../../../functions/src/scanner/context_drift/test_empty_context_file_no_claims.md)
- [parse_claude_md](../../../../../functions/src/scanner/context_drift/parse/parse_claude_md.md)
- [parse_context_file](../../../../../functions/src/scanner/context_drift/parse/parse_context_file.md)
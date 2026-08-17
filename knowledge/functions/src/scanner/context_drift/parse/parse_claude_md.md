---
type: Rust Function
title: parse_claude_md
resource: src/scanner/context_drift/parse.rs#L251-L260
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/parse/parse_agents_md
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/test_parse_claude_md_structure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_context_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn parse_claude_md( content: &str, source_path: &Path, file_type: ContextFileType, ) -> Vec<ContextClaim>`

# Calls

- [parse_agents_md](../../../../../functions/src/scanner/context_drift/parse/parse_agents_md.md)

# Called by

- [test_parse_claude_md_structure](../../../../../functions/src/scanner/context_drift/test_parse_claude_md_structure.md)
- [parse_context_file](../../../../../functions/src/scanner/context_drift/parse/parse_context_file.md)
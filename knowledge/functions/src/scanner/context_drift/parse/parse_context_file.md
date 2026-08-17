---
type: Rust Function
title: parse_context_file
resource: src/scanner/context_drift/parse.rs#L332-L343
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/discover/read_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_agents_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_claude_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_cursor_rules
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/parse/parse_all_context_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_context_file(path: &Path, file_type: ContextFileType) -> Vec<ContextClaim>`

# Calls

- [read_file](../../../../../functions/src/scanner/context_drift/discover/read_file.md)
- [parse_agents_md](../../../../../functions/src/scanner/context_drift/parse/parse_agents_md.md)
- [parse_claude_md](../../../../../functions/src/scanner/context_drift/parse/parse_claude_md.md)
- [parse_cursor_rules](../../../../../functions/src/scanner/context_drift/parse/parse_cursor_rules.md)

# Called by

- [parse_all_context_files](../../../../../functions/src/scanner/context_drift/parse/parse_all_context_files.md)
---
type: Rust Function
title: temp_file
resource: src/scanner/context_drift/mod.rs#L230-L236
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/context_drift/test_discover_context_files_agents_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_parse_agents_md_dependency_claim
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_parse_cursor_rules_frontmatter
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn temp_file(content: &str) -> (PathBuf, tempfile::TempDir)`

# Called by

- [test_discover_context_files_agents_md](../../../../functions/src/scanner/context_drift/test_discover_context_files_agents_md.md)
- [test_parse_agents_md_dependency_claim](../../../../functions/src/scanner/context_drift/test_parse_agents_md_dependency_claim.md)
- [test_parse_cursor_rules_frontmatter](../../../../functions/src/scanner/context_drift/test_parse_cursor_rules_frontmatter.md)
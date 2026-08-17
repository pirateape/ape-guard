---
type: Rust Function
title: test_parse_agents_md_dependency_claim
resource: src/scanner/context_drift/mod.rs#L269-L290
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/temp_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_agents_md
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_parse_agents_md_dependency_claim()`

# Calls

- [temp_file](../../../../functions/src/scanner/context_drift/temp_file.md)
- [parse_agents_md](../../../../functions/src/scanner/context_drift/parse/parse_agents_md.md)
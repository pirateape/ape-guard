---
type: Rust Function
title: extract_dependencies
resource: src/arch.rs#L381-L402
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_dependencies
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_dependencies(content: &str) -> Vec<(String, String)>`

# Called by

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)
- [test_extract_dependencies](../../../functions/src/arch/test_extract_dependencies.md)
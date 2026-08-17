---
type: Rust Function
title: extract_markdown_components
resource: src/arch.rs#L328-L362
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/is_generic_heading
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/extract_components
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/test_extract_markdown_components
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_markdown_components(content: &str) -> Vec<String>`

# Calls

- [is_generic_heading](../../../functions/src/arch/is_generic_heading.md)

# Called by

- [extract_components](../../../functions/src/arch/extract_components.md)
- [test_extract_markdown_components](../../../functions/src/arch/test_extract_markdown_components.md)
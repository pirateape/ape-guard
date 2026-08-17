---
type: Rust Function
title: parse_adr
resource: src/arch.rs#L405-L440
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/parse_adr_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/extract_adr_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/summarize_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/arch/parse_artifact
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_adr(content: &str) -> Vec<DecisionRecord>`

# Calls

- [parse_adr_title](../../../functions/src/arch/parse_adr_title.md)
- [extract_status](../../../functions/src/arch/extract_status.md)
- [extract_adr_context](../../../functions/src/arch/extract_adr_context.md)
- [summarize_content](../../../functions/src/arch/summarize_content.md)

# Called by

- [parse_artifact](../../../functions/src/arch/parse_artifact.md)
---
type: Rust Function
title: generate_component_recommendations
resource: src/arch.rs#L591-L620
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/arch/assess_component_risks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_component_recommendations(total: u32, critical: u32, high: u32) -> Vec<String>`

# Called by

- [assess_component_risks](../../../functions/src/arch/assess_component_risks.md)
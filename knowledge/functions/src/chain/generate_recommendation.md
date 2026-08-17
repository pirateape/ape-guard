---
type: Rust Function
title: generate_recommendation
resource: src/chain.rs#L417-L444
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/evaluate_rules_on_group
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_recommendation(rule: &ChainRule, _findings: &[&CanonicalFinding]) -> String`

# Called by

- [evaluate_rules_on_group](../../../functions/src/chain/evaluate_rules_on_group.md)
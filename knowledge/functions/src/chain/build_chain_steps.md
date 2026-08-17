---
type: Rust Function
title: build_chain_steps
resource: src/chain.rs#L389-L415
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

`fn build_chain_steps(findings: &[&CanonicalFinding]) -> Vec<String>`

# Called by

- [evaluate_rules_on_group](../../../functions/src/chain/evaluate_rules_on_group.md)
---
type: Rust Function
title: compute_chain_risk
resource: src/chain.rs#L374-L387
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/chain/severity_to_score
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/chain/evaluate_rules_on_group
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn compute_chain_risk(findings: &[&CanonicalFinding], rule: &ChainRule) -> f32`

# Calls

- [severity_to_score](../../../functions/src/chain/severity_to_score.md)

# Called by

- [evaluate_rules_on_group](../../../functions/src/chain/evaluate_rules_on_group.md)
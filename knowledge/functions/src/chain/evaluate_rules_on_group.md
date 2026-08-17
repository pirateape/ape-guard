---
type: Rust Function
title: evaluate_rules_on_group
resource: src/chain.rs#L301-L348
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/StrideCategory/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/compute_chain_risk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/build_chain_steps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/generate_recommendation
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn evaluate_rules_on_group(group: &[&CanonicalFinding], chains: &mut Vec<AttackChain>)`

# Calls

- [all](../../../functions/src/stride/StrideCategory/all.md)
- [compute_chain_risk](../../../functions/src/chain/compute_chain_risk.md)
- [build_chain_steps](../../../functions/src/chain/build_chain_steps.md)
- [generate_recommendation](../../../functions/src/chain/generate_recommendation.md)

# Called by

- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
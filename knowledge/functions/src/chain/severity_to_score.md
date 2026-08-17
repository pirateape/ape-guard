---
type: Rust Function
title: severity_to_score
resource: src/chain.rs#L354-L362
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/compute_chain_risk
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn severity_to_score(severity: &Severity) -> u32`

# Called by

- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
- [compute_chain_risk](../../../functions/src/chain/compute_chain_risk.md)
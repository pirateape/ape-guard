---
type: Rust Function
title: group_by_directory
resource: src/chain.rs#L164-L176
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn group_by_directory(findings: &[CanonicalFinding]) -> HashMap<String, Vec<&CanonicalFinding>>`

# Called by

- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
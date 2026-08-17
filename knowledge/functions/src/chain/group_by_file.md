---
type: Rust Function
title: group_by_file
resource: src/chain.rs#L154-L161
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

`fn group_by_file(findings: &[CanonicalFinding]) -> HashMap<String, Vec<&CanonicalFinding>>`

# Called by

- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
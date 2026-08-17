---
type: Rust Function
title: make_basic
resource: src/chain.rs#L515-L536
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/chain/make_finding
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/chain/test_zt_pillar_chain
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_basic( id: &str, rule_id: &str, title: &str, severity: Severity, file: &str, line: u32, scanner: ScannerType, ) -> CanonicalFinding`

# Calls

- [make_finding](../../../functions/src/chain/make_finding.md)

# Called by

- [test_zt_pillar_chain](../../../functions/src/chain/test_zt_pillar_chain.md)
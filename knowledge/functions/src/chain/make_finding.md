---
type: Rust Function
title: make_finding
resource: src/chain.rs#L471-L512
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/chain/make_basic
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn make_finding( id: &str, rule_id: &str, title: &str, severity: Severity, file: &str, line: u32, scanner: ScannerType, cross_refs: Vec<CrossReference>, cwe: Option<String>, tags: Vec<String>, ) -> CanonicalFinding`

# Called by

- [make_basic](../../../functions/src/chain/make_basic.md)
---
type: Rust Function
title: make_finding
resource: src/normalize.rs#L450-L480
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/normalize/test_mitre_mapping_secret
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn make_finding(id: &str, rule_id: &str, title: &str) -> CanonicalFinding`

# Called by

- [test_mitre_mapping_secret](../../../functions/src/normalize/test_mitre_mapping_secret.md)
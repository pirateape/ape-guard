---
type: Rust Function
title: test_enforce_ttl_prunes_old_scan_history
resource: src/cache.rs#L409-L445
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_enforce_ttl_prunes_old_scan_history()`

# Calls

- [open](../../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../../functions/src/cache/ScanCache/enforce_ttl.md)
---
type: Rust Module
title: cache
resource: src/cache.rs#L1-L446
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ScanCache](../../classes/src/cache/ScanCache.md)
- [RecordScanInput](../../classes/src/cache/RecordScanInput.md)
- [open](../../functions/src/cache/ScanCache/open.md)
- [disabled](../../functions/src/cache/ScanCache/disabled.md)
- [get_scan_findings](../../functions/src/cache/ScanCache/get_scan_findings.md)
- [get_latest_scan_findings](../../functions/src/cache/ScanCache/get_latest_scan_findings.md)
- [get_latest_scan_record](../../functions/src/cache/ScanCache/get_latest_scan_record.md)
- [stats](../../functions/src/cache/ScanCache/stats.md)
- [prune](../../functions/src/cache/ScanCache/prune.md)
- [enforce_ttl](../../functions/src/cache/ScanCache/enforce_ttl.md)
- [record_scan](../../functions/src/cache/ScanCache/record_scan.md)
- [recent_scans](../../functions/src/cache/ScanCache/recent_scans.md)
- [ScanRecord](../../classes/src/cache/ScanRecord.md)
- [CacheStats](../../classes/src/cache/CacheStats.md)
- [sample_finding](../../functions/src/cache/sample_finding.md)
- [test_record_scan_persists_timestamps](../../functions/src/cache/test_record_scan_persists_timestamps.md)
- [test_enforce_ttl_prunes_old_scan_history](../../functions/src/cache/test_enforce_ttl_prunes_old_scan_history.md)

# Imports

- `crate::find::CanonicalFinding`
- `rusqlite::{params, Connection}`
- `serde::Serialize`
- `std::path::{Path, PathBuf}`
- `super::*`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`

# Member of

- [apeguard](../../packages/apeguard.md)
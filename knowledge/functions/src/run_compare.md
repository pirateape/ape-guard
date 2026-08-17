---
type: Rust Function
title: run_compare
resource: src/main.rs#L408-L612
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
  - target: functions/src/cache/ScanCache/get_scan_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/count_by_sev
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run_compare( a: &str, b: &str, format: &cli::CompareFormat, cfg: &config::Config, quiet: bool, ) -> anyhow::Result<()>`

# Calls

- [open](../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../functions/src/cache/ScanCache/enforce_ttl.md)
- [get_scan_findings](../../functions/src/cache/ScanCache/get_scan_findings.md)
- [count_by_sev](../../functions/src/count_by_sev.md)

# Called by

- [main](../../functions/src/main.md)
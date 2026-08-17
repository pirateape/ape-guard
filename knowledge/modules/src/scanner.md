---
type: Rust Module
title: scanner
resource: src/scanner/mod.rs#L1-L175
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-scannertype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-arch-architectureanalyzer
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-scannertype
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [binary_exists](../../functions/src/scanner/binary_exists.md)
- [run_command_with_timeout](../../functions/src/scanner/run_command_with_timeout.md)
- [Scanner](../../interfaces/src/scanner/Scanner.md)
- [scan](../../functions/src/scanner/Scanner/scan.md)
- [ScannerResult](../../classes/src/scanner/ScannerResult.md)
- [ScannerError](../../classes/src/scanner/ScannerError.md)
- [test_arch_scanner_name](../../functions/src/scanner/test_arch_scanner_name.md)
- [test_arch_scanner_type](../../functions/src/scanner/test_arch_scanner_type.md)
- [test_arch_scanner_installed](../../functions/src/scanner/test_arch_scanner_installed.md)
- [test_arch_scanner_version](../../functions/src/scanner/test_arch_scanner_version.md)

# Imports

- `crate::find::{CanonicalFinding, ScannerType}`
- `async_trait::async_trait`
- `std::path::Path`
- `std::time::Duration`
- `super::*`
- `crate::scanner::arch::ArchitectureAnalyzer`
- `crate::find::ScannerType`

# Member of

- [apeguard](../../packages/apeguard.md)
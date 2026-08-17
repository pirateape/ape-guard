---
type: Rust Method
title: new
resource: src/scanner/arch.rs#L23-L28
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/arch/ArchitectureAnalyzer/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/arch/ArchitectureScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/test_arch_scanner_name
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/test_arch_scanner_type
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/test_arch_scanner_installed
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/test_arch_scanner_version
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`pub fn new(root: &Path) -> Self`

# Called by

- [default](../../../../../functions/src/scanner/arch/ArchitectureAnalyzer/default/default.md)
- [new](../../../../../functions/src/scanner/arch/ArchitectureScanner/new.md)
- [test_arch_scanner_name](../../../../../functions/src/scanner/test_arch_scanner_name.md)
- [test_arch_scanner_type](../../../../../functions/src/scanner/test_arch_scanner_type.md)
- [test_arch_scanner_installed](../../../../../functions/src/scanner/test_arch_scanner_installed.md)
- [test_arch_scanner_version](../../../../../functions/src/scanner/test_arch_scanner_version.md)
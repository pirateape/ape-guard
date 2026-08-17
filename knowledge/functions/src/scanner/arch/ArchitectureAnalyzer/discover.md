---
type: Rust Method
title: discover
resource: src/scanner/arch.rs#L31-L33
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/arch/discover_artifacts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/arch/ArchitectureScanner/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn discover(&self) -> Vec<ArchitectureArtifact>`

# Calls

- [discover_artifacts](../../../../../functions/src/arch/discover_artifacts.md)

# Called by

- [scan](../../../../../functions/src/scanner/arch/ArchitectureScanner/scan.md)
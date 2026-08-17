---
type: Rust Method
title: scan
resource: src/scanner/arch.rs#L100-L119
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/arch/ArchitectureAnalyzer/discover
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/arch/assess_component_risks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/arch/ArchitectureAnalyzer/generate_diagram
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn scan(&self) -> Result<ArchitectureResult, ScannerError>`

# Calls

- [discover](../../../../../functions/src/scanner/arch/ArchitectureAnalyzer/discover.md)
- [assess_component_risks](../../../../../functions/src/arch/assess_component_risks.md)
- [generate_diagram](../../../../../functions/src/scanner/arch/ArchitectureAnalyzer/generate_diagram.md)
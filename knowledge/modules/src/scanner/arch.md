---
type: Rust Module
title: arch
resource: src/scanner/arch.rs#L1-L417
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-arch-assess-component-risks-architectureartifact-artifacttype-componentrisk-decisionstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-scanner-scannererror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ArchitectureAnalyzer](../../../classes/src/scanner/arch/ArchitectureAnalyzer.md)
- [new](../../../functions/src/scanner/arch/ArchitectureAnalyzer/new.md)
- [discover](../../../functions/src/scanner/arch/ArchitectureAnalyzer/discover.md)
- [generate_diagram](../../../functions/src/scanner/arch/ArchitectureAnalyzer/generate_diagram.md)
- [default](../../../functions/src/scanner/arch/ArchitectureAnalyzer/default/default.md)
- [name](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/name.md)
- [scanner_type](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/scanner_type.md)
- [check_installed](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/check_installed.md)
- [version](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/version.md)
- [scan_raw](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/scan_raw.md)
- [parse_output](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/parse_output.md)
- [install_hint](../../../functions/src/scanner/arch/ArchitectureAnalyzer/scanner/install_hint.md)
- [ArchitectureScanner](../../../classes/src/scanner/arch/ArchitectureScanner.md)
- [new](../../../functions/src/scanner/arch/ArchitectureScanner/new.md)
- [scan](../../../functions/src/scanner/arch/ArchitectureScanner/scan.md)
- [ArchitectureResult](../../../classes/src/scanner/arch/ArchitectureResult.md)
- [generate_architecture_findings](../../../functions/src/scanner/arch/generate_architecture_findings.md)
- [generate_doc_findings](../../../functions/src/scanner/arch/generate_doc_findings.md)
- [generate_adr_findings](../../../functions/src/scanner/arch/generate_adr_findings.md)
- [generate_diagram_findings](../../../functions/src/scanner/arch/generate_diagram_findings.md)

# Imports

- `crate::arch::{
    assess_component_risks, ArchitectureArtifact, ArtifactType, ComponentRisk, DecisionStatus,
}`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `crate::scanner::{Scanner, ScannerError}`
- `std::path::{Path, PathBuf}`

# Member of

- [apeguard](../../../packages/apeguard.md)
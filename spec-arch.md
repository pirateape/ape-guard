---
id: apeguard-arch
type: module-design
title: arch.rs — architecture discovery & component risk
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# arch.rs — architecture discovery & component risk

## Responsibility

`src/arch.rs` discovers architecture artifacts (`ARCHITECTURE.md`, ADRs in `adr/`,
Mermaid `.mmd`/`.mermaid` diagrams, C4 models) via glob patterns, parses them into
`ArchitectureArtifact { path, artifact_type, content_summary, components, dependencies,
decisions }`, classifies artifact type, extracts Mermaid nodes/edges and markdown headings,
parses ADR title/status/context, then `assess_component_risks` matches findings to
components by path and assigns `RiskLevel` (Low/Medium/High/Critical), and
`generate_mermaid_diagram` emits a color-coded Mermaid graph.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ...}`, `glob`, `regex`, `std::path::{Path,
  PathBuf}`, `std::collections::HashMap`/`HashSet`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Notes

- `DecisionRecord` fields are `#[expect(dead_code)]` (P3/P4: populated by future analysis
  phases).
- Used by `main.rs` report regeneration and `orchestrate` report generation to rebuild the
  architecture diagram.

## Tests

Inline unit tests cover artifact classification, Mermaid node/subgraph extraction, dependency
extraction, ADR title parsing, status extraction, markdown component extraction, id
sanitization, and component risk generation (all in `src/arch.rs`).

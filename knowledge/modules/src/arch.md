---
type: Rust Module
title: arch
resource: src/arch.rs#L1-L840
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ArchitectureArtifact](../../classes/src/arch/ArchitectureArtifact.md)
- [ArtifactType](../../classes/src/arch/ArtifactType.md)
- [DecisionRecord](../../classes/src/arch/DecisionRecord.md)
- [DecisionStatus](../../classes/src/arch/DecisionStatus.md)
- [ComponentRisk](../../classes/src/arch/ComponentRisk.md)
- [RiskLevel](../../classes/src/arch/RiskLevel.md)
- [discover_artifacts](../../functions/src/arch/discover_artifacts.md)
- [parse_artifact](../../functions/src/arch/parse_artifact.md)
- [classify_artifact](../../functions/src/arch/classify_artifact.md)
- [extract_components](../../functions/src/arch/extract_components.md)
- [extract_mermaid_components](../../functions/src/arch/extract_mermaid_components.md)
- [extract_mermaid_node](../../functions/src/arch/extract_mermaid_node.md)
- [extract_bracket_label](../../functions/src/arch/extract_bracket_label.md)
- [extract_mermaid_subgraph](../../functions/src/arch/extract_mermaid_subgraph.md)
- [extract_markdown_components](../../functions/src/arch/extract_markdown_components.md)
- [is_generic_heading](../../functions/src/arch/is_generic_heading.md)
- [extract_dependencies](../../functions/src/arch/extract_dependencies.md)
- [parse_adr](../../functions/src/arch/parse_adr.md)
- [parse_adr_title](../../functions/src/arch/parse_adr_title.md)
- [extract_status](../../functions/src/arch/extract_status.md)
- [extract_adr_context](../../functions/src/arch/extract_adr_context.md)
- [summarize_content](../../functions/src/arch/summarize_content.md)
- [assess_component_risks](../../functions/src/arch/assess_component_risks.md)
- [generate_component_recommendations](../../functions/src/arch/generate_component_recommendations.md)
- [generate_mermaid_diagram](../../functions/src/arch/generate_mermaid_diagram.md)
- [sanitize_mermaid_id](../../functions/src/arch/sanitize_mermaid_id.md)
- [test_classify_mermaid_diagram](../../functions/src/arch/test_classify_mermaid_diagram.md)
- [test_classify_adr](../../functions/src/arch/test_classify_adr.md)
- [test_classify_architecture_doc](../../functions/src/arch/test_classify_architecture_doc.md)
- [test_extract_mermaid_node_square](../../functions/src/arch/test_extract_mermaid_node_square.md)
- [test_extract_mermaid_node_round](../../functions/src/arch/test_extract_mermaid_node_round.md)
- [test_extract_mermaid_node_quoted](../../functions/src/arch/test_extract_mermaid_node_quoted.md)
- [test_extract_mermaid_subgraph](../../functions/src/arch/test_extract_mermaid_subgraph.md)
- [test_extract_mermaid_components](../../functions/src/arch/test_extract_mermaid_components.md)
- [test_extract_dependencies](../../functions/src/arch/test_extract_dependencies.md)
- [test_parse_adr_title](../../functions/src/arch/test_parse_adr_title.md)
- [test_extract_status_accepted](../../functions/src/arch/test_extract_status_accepted.md)
- [test_extract_markdown_components](../../functions/src/arch/test_extract_markdown_components.md)
- [test_sanitize_mermaid_id](../../functions/src/arch/test_sanitize_mermaid_id.md)
- [test_component_risk_generation](../../functions/src/arch/test_component_risk_generation.md)

# Imports

- `crate::find::*`
- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `super::*`

# Member of

- [apeguard](../../packages/apeguard.md)
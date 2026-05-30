// ApeGuard Architecture Analysis
// Discovers and parses architecture artifacts (ARCHITECTURE.md, ADRs, Mermaid diagrams)
// Cross-references security findings with architectural components.

use crate::find::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents a discovered architecture artifact.
#[derive(Debug, Clone)]
pub struct ArchitectureArtifact {
    pub path: PathBuf,
    pub artifact_type: ArtifactType,
    pub content_summary: String,
    /// Components mentioned in the artifact
    pub components: Vec<String>,
    /// Dependencies between components (from, to)
    pub dependencies: Vec<(String, String)>,
    /// Decision IDs (for ADRs)
    pub decisions: Vec<DecisionRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactType {
    ArchitectureDoc,
    Adr,
    MermaidDiagram,
    C4Model,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DecisionRecord {
    pub id: String,
    pub title: String,
    pub status: DecisionStatus,
    pub context_summary: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecisionStatus {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
    Unknown,
}

/// Risk assessment for an architectural component based on security findings.
#[derive(Debug, Clone)]
pub struct ComponentRisk {
    pub component_name: String,
    pub finding_count: u32,
    pub critical_count: u32,
    pub high_count: u32,
    pub risk_level: RiskLevel,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Discover architecture artifacts in a codebase.
pub fn discover_artifacts(root: &Path) -> Vec<ArchitectureArtifact> {
    let mut artifacts = Vec::new();

    // Common architecture file patterns
    let patterns = [
        "**/ARCHITECTURE.md",
        "**/docs/architecture/**/*.md",
        "**/docs/arch/**/*.md",
        "**/adr/**/*.md",
        "**/decisions/**/*.md",
        "**/*.mmd",
        "**/*.mermaid",
        "**/docs/architecture/**/*.mmd",
        "**/diagrams/**/*.mmd",
        "**/diagrams/**/*.mermaid",
    ];

    let mut seen = std::collections::HashSet::new();

    for pattern in &patterns {
        let full_pattern = root.join(pattern);
        let pattern_str = full_pattern.to_string_lossy().to_string();

        if let Ok(paths) = glob::glob(&pattern_str) {
            for entry in paths.flatten() {
                let canonical = entry.canonicalize().unwrap_or_else(|_| entry.clone());
                if seen.contains(&canonical) {
                    continue;
                }
                seen.insert(canonical.clone());

                if let Some(artifact) = parse_artifact(&entry) {
                    artifacts.push(artifact);
                }
            }
        }
    }

    artifacts
}

/// Parse an architecture artifact file.
fn parse_artifact(path: &Path) -> Option<ArchitectureArtifact> {
    let content = std::fs::read_to_string(path).ok()?;
    let filename = path.file_name()?.to_string_lossy().to_lowercase();
    let dirname = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let artifact_type = classify_artifact(path, &filename, &dirname);
    let components = extract_components(&content, &artifact_type);
    let dependencies = extract_dependencies(&content);
    let decisions = if artifact_type == ArtifactType::Adr {
        parse_adr(&content)
    } else {
        Vec::new()
    };

    Some(ArchitectureArtifact {
        path: path.to_path_buf(),
        artifact_type,
        content_summary: summarize_content(&content),
        components,
        dependencies,
        decisions,
    })
}

/// Classify the type of architecture artifact.
fn classify_artifact(path: &Path, filename: &str, dirname: &str) -> ArtifactType {
    let filename = filename.to_lowercase();
    let dirname = dirname.to_lowercase();

    if filename.ends_with(".mmd") || filename.ends_with(".mermaid") {
        return ArtifactType::MermaidDiagram;
    }

    if dirname.contains("adr") || dirname.contains("decision") {
        return ArtifactType::Adr;
    }

    if filename.contains("architecture") || filename.contains("arch.") {
        return ArtifactType::ArchitectureDoc;
    }

    // Check content for C4 model markers
    if let Ok(content) = std::fs::read_to_string(path) {
        let lower = content.to_lowercase();
        if lower.contains("container diagram") || lower.contains("c4model") {
            return ArtifactType::C4Model;
        }
        if lower.contains("## decision") || lower.contains("# adr") {
            return ArtifactType::Adr;
        }
    }

    ArtifactType::Unknown
}

/// Extract component names from content based on artifact type.
fn extract_components(content: &str, artifact_type: &ArtifactType) -> Vec<String> {
    match artifact_type {
        ArtifactType::MermaidDiagram | ArtifactType::C4Model => {
            extract_mermaid_components(content)
        }
        ArtifactType::ArchitectureDoc => extract_markdown_components(content),
        _ => Vec::new(),
    }
}

/// Extract component names from Mermaid diagram syntax.
fn extract_mermaid_components(content: &str) -> Vec<String> {
    let mut components = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Match Mermaid node definitions: NodeId[Label] or NodeId(Label) or NodeId{Label}
        // e.g., A[API Gateway] or DB[(Database)] or C{Service}
        if let Some(node_def) = extract_mermaid_node(trimmed) {
            if seen.insert(node_def.clone()) {
                components.push(node_def);
            }
            continue;
        }

        // Match Mermaid subgraph definitions: subgraph name [Title]
        if let Some(subgraph) = extract_mermaid_subgraph(trimmed) {
            if seen.insert(subgraph.clone()) {
                components.push(subgraph);
            }
        }
    }

    components
}

/// Extract a single Mermaid node definition from a line.
fn extract_mermaid_node(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Ignore arrows, classDef, subgraph lines
    if trimmed.starts_with("classDef") || trimmed.starts_with("subgraph") {
        return None;
    }

    // Match node ID followed by bracket content: NODE_ID[content] or NODE_ID(content) etc.
    // First find the node ID (alphanumeric starting with letter)
    let node_start = trimmed.find(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_')?;
    let after_id = &trimmed[node_start..];

    // Extract node ID
    let id_end = after_id.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    let _node_id = &after_id[..id_end];
    let rest = after_id[id_end..].trim();

    if rest.is_empty() {
        return None;
    }

    // Extract label from brackets
    extract_bracket_label(rest)
}

/// Extract a label from bracket-enclosed content, handling Mermaid's special syntaxes.
fn extract_bracket_label(text: &str) -> Option<String> {
    // Handle [(Database)] → cylindrical DB shape
    if text.starts_with("[(") {
        if let Some(end) = text.find(")]") {
            return Some(text[2..end].trim().to_string());
        }
    }

    // Handle ["Quoted Label"] or ["Label"]
    if text.starts_with("[\"") {
        if let Some(end) = text.find("\"]") {
            return Some(text[2..end].trim().to_string());
        }
    }

    // Handle [Label] — rectangular node
    if text.starts_with('[') {
        if let Some(end) = text.find(']') {
            let label = &text[1..end].trim();
            // Remove surrounding quotes if present
            let clean = label.trim_matches('"');
            if !clean.is_empty() {
                return Some(clean.to_string());
            }
        }
    }

    // Handle (Label) — rounded node
    if text.starts_with('(') {
        if let Some(end) = text.find(')') {
            let label = &text[1..end].trim();
            let clean = label.trim_matches('"');
            if !clean.is_empty() {
                return Some(clean.to_string());
            }
        }
    }

    // Handle {Label} — rhombus node
    if text.starts_with('{') {
        if let Some(end) = text.find('}') {
            let label = &text[1..end].trim();
            let clean = label.trim_matches('"');
            if !clean.is_empty() {
                return Some(clean.to_string());
            }
        }
    }

    None
}

/// Extract a Mermaid subgraph definition.
fn extract_mermaid_subgraph(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("subgraph") {
        // subgraph name [Title]
        let after_subgraph = trimmed.strip_prefix("subgraph")?.trim();
        if let Some(open_bracket) = after_subgraph.find('[') {
            let title = &after_subgraph[open_bracket + 1..];
            let title = title.strip_suffix(']').unwrap_or(title);
            return Some(title.trim().to_string());
        }
        return Some(after_subgraph.to_string());
    }
    None
}

/// Extract component names from markdown architecture docs.
fn extract_markdown_components(content: &str) -> Vec<String> {
    let mut components = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Match markdown headings that look like component names
        // e.g., ### API Gateway, ## Database Layer, ### Auth Service
        if trimmed.starts_with("### ") || trimmed.starts_with("## ") {
            let heading = trimmed.trim_start_matches('#').trim();
            // Filter out generic headings
            if !is_generic_heading(heading) {
                if seen.insert(heading.to_string()) {
                    components.push(heading.to_string());
                }
            }
        }
    }

    // Also look for bullet lists containing component/service names
    for line in content.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with("- ") || trimmed.starts_with("* "))
            && (trimmed.contains("Service") || trimmed.contains("API") || trimmed.contains("Database"))
        {
            let name = trimmed
                .trim_start_matches(|c| c == '-' || c == '*' || c == ' ')
                .to_string();
            if seen.insert(name.clone()) {
                components.push(name);
            }
        }
    }

    components
}

/// Check if a heading is generic (not a component name).
fn is_generic_heading(heading: &str) -> bool {
    let lower = heading.to_lowercase();
    lower == "overview"
        || lower == "architecture"
        || lower == "introduction"
        || lower == "conclusion"
        || lower == "references"
        || lower == "summary"
        || lower == "getting started"
        || lower == "setup"
        || lower == "installation"
        || lower.starts_with("table of")
        || lower.ends_with("guide")
}

/// Extract dependencies from Mermaid diagram content.
fn extract_dependencies(content: &str) -> Vec<(String, String)> {
    let mut deps = Vec::new();

    // Match Mermaid edges: A[shape] --> B[shape] (with optional bracket content after IDs)
    // Also handles: A -->|label| B, A -.-> B, A ==> B, A -- B
    let re = regex::Regex::new(
        r#"([A-Za-z_]\w*)(?:\s*\[[^\]]*\]|\s*\([^)]*\)|\s*\{[^}]*\})?\s*(?:-->|==>|-\.->|==)\s*(?:\|[^|]+\|\s*)?([A-Za-z_]\w*)"#,
    )
    .ok();

    if let Some(re) = re {
        for line in content.lines() {
            for caps in re.captures_iter(line) {
                let from = caps.get(1).unwrap().as_str().to_string();
                let to = caps.get(2).unwrap().as_str().to_string();
                deps.push((from, to));
            }
        }
    }

    deps
}

/// Parse an Architecture Decision Record (ADR) from markdown content.
fn parse_adr(content: &str) -> Vec<DecisionRecord> {
    let mut decisions = Vec::new();

    // Match ADR title: # ADR-NNN: Title or ## Decision: Title
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(decision) = parse_adr_title(trimmed) {
            let status = extract_status(content);
            let summary = extract_adr_context(content);
            decisions.push(DecisionRecord {
                id: decision.0,
                title: decision.1,
                status,
                context_summary: summary,
            });
        }
    }

    if decisions.is_empty() {
        // Fallback: treat the whole file as one ADR
        let filename_summary = content
            .lines()
            .find(|l| l.starts_with('#'))
            .map(|l| l.trim_start_matches('#').trim().to_string())
            .unwrap_or_else(|| "Untitled Decision".to_string());

        decisions.push(DecisionRecord {
            id: "ADR-UNKNOWN".to_string(),
            title: filename_summary,
            status: extract_status(content),
            context_summary: summarize_content(content),
        });
    }

    decisions
}

/// Parse a single ADR title line.
fn parse_adr_title(line: &str) -> Option<(String, String)> {
    let re =
        regex::Regex::new(r#"^#+\s*(?:ADR\s*[-:]\s*)?(\d+)\s*[-:]\s*(.+)$"#).ok()?;
    let caps = re.captures(line)?;
    let id = format!("ADR-{}", caps.get(1).unwrap().as_str());
    let title = caps.get(2).unwrap().as_str().trim().to_string();
    Some((id, title))
}

/// Extract decision status from ADR content.
fn extract_status(content: &str) -> DecisionStatus {
    let lower = content.to_lowercase();

    // Common ADR status markers
    if lower.contains("status: accepted") || lower.contains("**accepted**") {
        DecisionStatus::Accepted
    } else if lower.contains("status: proposed") || lower.contains("**proposed**") {
        DecisionStatus::Proposed
    } else if lower.contains("status: deprecated") || lower.contains("**deprecated**") {
        DecisionStatus::Deprecated
    } else if lower.contains("status: superseded") || lower.contains("**superseded**") {
        DecisionStatus::Superseded
    } else {
        DecisionStatus::Unknown
    }
}

/// Extract ADR context summary.
fn extract_adr_context(content: &str) -> String {
    // Look for "## Context" section
    for (i, line) in content.lines().enumerate() {
        let lower = line.trim().to_lowercase();
        if lower.starts_with("## context") || lower.starts_with("### context") {
            // Collect the next few non-empty lines
            let summary: Vec<&str> = content
                .lines()
                .skip(i + 1)
                .take(5)
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                .collect();
            if !summary.is_empty() {
                return summary.join(" ").chars().take(200).collect();
            }
        }
    }
    // Fallback: first non-heading paragraph
    content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(200)
        .collect()
}

/// Summarize the content of an artifact.
fn summarize_content(content: &str) -> String {
    content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(160)
        .collect()
}

/// Cross-reference security findings with architectural components.
pub fn assess_component_risks(
    findings: &[CanonicalFinding],
    artifacts: &[ArchitectureArtifact],
) -> Vec<ComponentRisk> {
    let mut risk_map: HashMap<String, (u32, u32, u32)> = HashMap::new(); // total, critical, high

    // Collect all known components from artifacts
    let mut all_components: HashMap<String, Vec<String>> = HashMap::new(); // component → aliases

    for artifact in artifacts {
        for comp in &artifact.components {
            let key = comp.to_lowercase();
            all_components
                .entry(key.clone())
                .or_default()
                .push(comp.clone());
        }
    }

    // Match findings against components by path
    for finding in findings {
        let path_str = finding.location.file.to_string_lossy().to_lowercase();

        for (comp_key, aliases) in &all_components {
            // Check if finding path relates to component
            let matches = aliases.iter().any(|alias| {
                let alias_lower = alias.to_lowercase();
                path_str.contains(&alias_lower)
                    || alias_lower.split_whitespace().any(|word| {
                        word.len() > 3 && path_str.contains(word)
                    })
            });

            if matches {
                let entry = risk_map.entry(comp_key.clone()).or_insert((0, 0, 0));
                entry.0 += 1;
                match finding.severity {
                    Severity::Critical => entry.1 += 1,
                    Severity::High => entry.2 += 1,
                    _ => {}
                }
            }
        }
    }

    // Build ComponentRisk structs
    risk_map
        .into_iter()
        .map(|(comp_name, (total, critical, high))| {
            let risk_level = if critical > 0 {
                RiskLevel::Critical
            } else if high > 2 || total > 5 {
                RiskLevel::High
            } else if total > 2 {
                RiskLevel::Medium
            } else {
                RiskLevel::Low
            };

            let recommendations = generate_component_recommendations(total, critical, high);

            ComponentRisk {
                component_name: all_components
                    .get(&comp_name)
                    .and_then(|aliases| aliases.first())
                    .cloned()
                    .unwrap_or(comp_name.clone()),
                finding_count: total,
                critical_count: critical,
                high_count: high,
                risk_level,
                recommendations,
            }
        })
        .collect()
}

/// Generate remediation recommendations for a component.
fn generate_component_recommendations(total: u32, critical: u32, high: u32) -> Vec<String> {
    let mut recs = Vec::new();

    if critical > 0 {
        recs.push(format!(
            "URGENT: {} critical finding(s) require immediate attention",
            critical
        ));
    }
    if high > 0 {
        recs.push(format!(
            "Address {} high-severity finding(s) in this component",
            high
        ));
    }
    if total > 5 {
        recs.push(format!(
            "Component has {} total findings — consider architectural review",
            total
        ));
    }
    if critical == 0 && high == 0 && total > 0 {
        recs.push("Low-priority findings — address in next sprint".to_string());
    }
    if recs.is_empty() {
        recs.push("No significant risks detected".to_string());
    }

    recs
}

/// Generate a Mermaid architecture diagram from component dependencies.
pub fn generate_mermaid_diagram(
    artifacts: &[ArchitectureArtifact],
    risks: &[ComponentRisk],
) -> String {
    let mut diagram = String::from("```mermaid\ngraph TD\n");

    // Add subgraphs for risk levels
    let mut critical_comps = Vec::new();
    let mut high_comps = Vec::new();

    for risk in risks {
        match risk.risk_level {
            RiskLevel::Critical => critical_comps.push(&risk.component_name),
            RiskLevel::High => high_comps.push(&risk.component_name),
            _ => {}
        }
    }

    // Add nodes with color coding
    for risk in risks {
        let style = match risk.risk_level {
            RiskLevel::Critical => ":::critical",
            RiskLevel::High => ":::high",
            RiskLevel::Medium => ":::medium",
            RiskLevel::Low => "",
        };
        diagram.push_str(&format!(
            "    {}[\"<b>{}</b><br/>{} findings: {} critical, {} high\"]{}\n",
            sanitize_mermaid_id(&risk.component_name),
            risk.component_name,
            risk.finding_count,
            risk.critical_count,
            risk.high_count,
            style,
        ));
    }

    // Add dependencies between components
    let mut seen_edges = std::collections::HashSet::new();
    for artifact in artifacts {
        for (from, to) in &artifact.dependencies {
            let edge = (sanitize_mermaid_id(from), sanitize_mermaid_id(to));
            if seen_edges.insert(edge.clone()) {
                diagram.push_str(&format!("    {} --> {}\n", edge.0, edge.1));
            }
        }
    }

    // Add style definitions
    diagram.push_str("\n    classDef critical fill:#ff4444,stroke:#cc0000,color:white\n");
    diagram.push_str("    classDef high fill:#ff8800,stroke:#cc6600,color:white\n");
    diagram.push_str("    classDef medium fill:#ffcc00,stroke:#cc9900\n");

    diagram.push_str("```\n");
    diagram
}

/// Sanitize a string for use as a Mermaid node ID.
fn sanitize_mermaid_id(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_mermaid_diagram() {
        let result = classify_artifact(
            Path::new("/src/diagrams/system.mmd"),
            "system.mmd",
            "diagrams",
        );
        assert_eq!(result, ArtifactType::MermaidDiagram);
    }

    #[test]
    fn test_classify_adr() {
        let result = classify_artifact(
            Path::new("/adr/001-choose-database.md"),
            "001-choose-database.md",
            "adr",
        );
        assert_eq!(result, ArtifactType::Adr);
    }

    #[test]
    fn test_classify_architecture_doc() {
        let result = classify_artifact(
            Path::new("/docs/ARCHITECTURE.md"),
            "ARCHITECTURE.md",
            "docs",
        );
        assert_eq!(result, ArtifactType::ArchitectureDoc);
    }

    #[test]
    fn test_extract_mermaid_node_square() {
        let node = extract_mermaid_node("    A[API Gateway]");
        assert_eq!(node, Some("API Gateway".to_string()));
    }

    #[test]
    fn test_extract_mermaid_node_round() {
        let node = extract_mermaid_node("    DB[(Database)]");
        assert_eq!(node, Some("Database".to_string()));
    }

    #[test]
    fn test_extract_mermaid_node_quoted() {
        let node = extract_mermaid_node("    A[\"API Gateway\"]");
        assert_eq!(node, Some("API Gateway".to_string()));
    }

    #[test]
    fn test_extract_mermaid_subgraph() {
        let node = extract_mermaid_subgraph("    subgraph api [API Layer]");
        assert_eq!(node, Some("API Layer".to_string()));
    }

    #[test]
    fn test_extract_mermaid_components() {
        let content = "graph TD\n    A[Gateway]\n    B[Auth Service]\n    DB[(Database)]";
        let comps = extract_mermaid_components(content);
        assert_eq!(comps.len(), 3);
        assert!(comps.contains(&"Gateway".to_string()));
        assert!(comps.contains(&"Auth Service".to_string()));
        assert!(comps.contains(&"Database".to_string()));
    }

    #[test]
    fn test_extract_dependencies() {
        let content = "graph TD\n    A[Gateway] --> B[Auth]\n    B --> DB[(DB)]";
        let deps = extract_dependencies(content);
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_parse_adr_title() {
        let result = parse_adr_title("# ADR-001: Use PostgreSQL as primary database");
        assert!(result.is_some());
        let (id, title) = result.unwrap();
        assert_eq!(id, "ADR-001");
        assert!(title.contains("PostgreSQL"));
    }

    #[test]
    fn test_extract_status_accepted() {
        let status = extract_status("## Status\n\n**Accepted**");
        assert_eq!(status, DecisionStatus::Accepted);
    }

    #[test]
    fn test_extract_markdown_components() {
        let content = "## Architecture\n\n### API Gateway\n### Auth Service\n### Database Layer\n\n### Overview\n### Setup";
        let comps = extract_markdown_components(content);
        // "Overview" and "Setup" should be filtered out
        assert!(comps.contains(&"API Gateway".to_string()));
        assert!(comps.contains(&"Auth Service".to_string()));
        assert!(!comps.contains(&"Overview".to_string()));
        assert!(!comps.contains(&"Setup".to_string()));
    }

    #[test]
    fn test_sanitize_mermaid_id() {
        assert_eq!(sanitize_mermaid_id("API Gateway"), "APIGateway");
        assert_eq!(sanitize_mermaid_id("auth_service"), "auth_service");
        assert_eq!(sanitize_mermaid_id("hello-world"), "helloworld");
    }

    #[test]
    fn test_component_risk_generation() {
        let artifacts = vec![ArchitectureArtifact {
            path: PathBuf::from("ARCHITECTURE.md"),
            artifact_type: ArtifactType::ArchitectureDoc,
            content_summary: "Test".into(),
            components: vec!["API Gateway".into()],
            dependencies: vec![],
            decisions: vec![],
        }];

        let finding = CanonicalFinding {
            id: "F1".into(),
            scanner: ScannerType::Semgrep,
            scanner_version: None,
            rule_id: "test".into(),
            severity: Severity::Critical,
            confidence: Confidence::Firm,
            title: "Test".into(),
            description: "Test".into(),
            location: FindingLocation {
                file: PathBuf::from("src/api_gateway/handler.py"),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
        };

        let risks = assess_component_risks(&[finding], &artifacts);
        assert!(!risks.is_empty(), "Should identify component risk");
        assert_eq!(risks[0].risk_level, RiskLevel::Critical);
    }
}

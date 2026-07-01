// Architecture Analysis Scanner
// Discovers and parses architecture artifacts (ARCHITECTURE.md, ADRs, Mermaid diagrams)
// Cross-references security findings with architectural components.
#![allow(dead_code)] // P3/P4: entire module is stub implementation for Phase 3 architecture analysis
use crate::arch::{
    assess_component_risks, ArchitectureArtifact, ArtifactType, ComponentRisk, DecisionStatus,
};
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use std::path::{Path, PathBuf};

/// Architecture Analysis Scanner
/// Discovers architecture artifacts and cross-references security findings with architectural components.
pub struct ArchitectureAnalyzer {
    /// Root path to scan for architecture artifacts
    root: PathBuf,
    /// Maximum findings to return per artifact type
    max_findings: usize,
}

impl ArchitectureAnalyzer {
    /// Create a new architecture analyzer
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            max_findings: 100,
        }
    }

    /// Discover all architecture artifacts in the codebase
    fn discover(&self) -> Vec<ArchitectureArtifact> {
        crate::arch::discover_artifacts(&self.root)
    }

    /// Generate Mermaid diagram from architecture artifacts
    fn generate_diagram(
        &self,
        artifacts: &[ArchitectureArtifact],
        risks: &[ComponentRisk],
    ) -> String {
        crate::arch::generate_mermaid_diagram(artifacts, risks)
    }
}

impl Default for ArchitectureAnalyzer {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

#[async_trait::async_trait]
impl Scanner for ArchitectureAnalyzer {
    fn name(&self) -> &'static str {
        "architecture"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Architecture
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        // Architecture scanner is always installed (built-in)
        Ok(true)
    }

    async fn version(&self) -> Result<String, ScannerError> {
        Ok("0.1.0".to_string())
    }

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        // Architecture scanning is file-system based, not binary output
        Ok(b"Architecture scan complete".to_vec())
    }

    fn parse_output(&self, _raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // Architecture scanner doesn't use raw output parsing
        // It discovers artifacts and generates findings
        Ok(vec![])
    }

    fn install_hint(&self) -> &'static str {
        "Architecture scanner is built-in to ApeGuard"
    }
}

/// Architecture Scanner that implements the full scan workflow
pub struct ArchitectureScanner {
    analyzer: ArchitectureAnalyzer,
}

impl ArchitectureScanner {
    /// Create a new architecture scanner
    pub fn new(root: &Path) -> Self {
        Self {
            analyzer: ArchitectureAnalyzer::new(root),
        }
    }

    /// Run a full architecture analysis scan
    pub async fn scan(&self) -> Result<ArchitectureResult, ScannerError> {
        // Discover architecture artifacts
        let artifacts = self.analyzer.discover();

        if artifacts.is_empty() {
            return Ok(ArchitectureResult::NoArtifacts);
        }

        // Assess component risks by cross-referencing with security findings
        // Note: This requires findings from other scanners
        // For now, return artifacts with basic risk assessment
        let component_risks = assess_component_risks(&[], &artifacts);

        Ok(ArchitectureResult::Complete {
            artifact_count: artifacts.len(),
            artifacts: artifacts.clone(),
            component_risks: component_risks.clone(),
            mermaid_diagram: self.analyzer.generate_diagram(&artifacts, &component_risks),
        })
    }
}

/// Architecture Scan Result
#[derive(Debug)]
pub enum ArchitectureResult {
    /// No architecture artifacts found
    NoArtifacts,
    /// Full scan complete with artifacts and findings
    Complete {
        artifact_count: usize,
        artifacts: Vec<ArchitectureArtifact>,
        component_risks: Vec<ComponentRisk>,
        mermaid_diagram: String,
    },
}

/// Generate architecture findings from discovered artifacts
pub fn generate_architecture_findings(artifacts: &[ArchitectureArtifact]) -> Vec<CanonicalFinding> {
    let mut findings = Vec::new();

    for artifact in artifacts {
        // Generate findings for each artifact type
        match artifact.artifact_type {
            ArtifactType::ArchitectureDoc => {
                // Findings for architecture documentation
                findings.extend(generate_doc_findings(artifact));
            }
            ArtifactType::Adr => {
                // Findings for Architecture Decision Records
                findings.extend(generate_adr_findings(artifact));
            }
            ArtifactType::MermaidDiagram | ArtifactType::C4Model => {
                // Findings for Mermaid diagrams
                findings.extend(generate_diagram_findings(artifact));
            }
            ArtifactType::Unknown => {
                // Unknown artifact type - generate warning finding
                findings.push(CanonicalFinding {
                    id: format!("ARCH-{}", findings.len() + 1),
                    scanner: ScannerType::Architecture,
                    scanner_version: Some("0.1.0".to_string()),
                    rule_id: "arch.discovery".to_string(),
                    severity: Severity::Medium,
                    confidence: Confidence::Firm,
                    title: "Architecture artifact discovered".to_string(),
                    description: format!(
                        "Discovered architecture artifact: {:?} at {}",
                        artifact.artifact_type,
                        artifact.path.display()
                    ),
                    location: FindingLocation {
                        file: artifact.path.clone(),
                        line: None,
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
                    tags: vec![String::from("architecture"), String::from("discovery")],
                    zt_pillars: vec![],
                    cross_refs: vec![],
                    grade: None,
                    risk_score: None,
                    reachable: None,
                });
            }
        }
    }

    findings
}

/// Generate findings for architecture documentation
fn generate_doc_findings(artifact: &ArchitectureArtifact) -> Vec<CanonicalFinding> {
    let mut findings = Vec::new();

    // Check for common architecture issues
    let content_lower = artifact.content_summary.to_lowercase();

    // Missing encryption check
    if !content_lower.contains("encryption")
        && !content_lower.contains("tls")
        && !content_lower.contains("ssl")
    {
        findings.push(CanonicalFinding {
            id: format!("ARCH-{}", findings.len() + 1),
            scanner: ScannerType::Architecture,
            scanner_version: Some("0.1.0".to_string()),
            rule_id: "arch.missing-encryption".to_string(),
            severity: Severity::Medium,
            confidence: Confidence::Firm,
            title: "Missing encryption reference in architecture".to_string(),
            description:
                "Architecture documentation does not reference encryption mechanisms (TLS/SSL)"
                    .to_string(),
            location: FindingLocation {
                file: artifact.path.clone(),
                line: None,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some(
                "Add encryption requirements to architecture documentation".to_string(),
            ),
            fix_effort: Some("1".to_string()),
            evidence: Some(artifact.content_summary.clone()),
            tags: vec![String::from("architecture"), String::from("encryption")],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        });
    }

    // Missing authentication check
    if !content_lower.contains("authentication")
        && !content_lower.contains("auth")
        && !content_lower.contains("oauth2")
        && !content_lower.contains("oidc")
    {
        findings.push(CanonicalFinding {
            id: format!("ARCH-{}", findings.len() + 1),
            scanner: ScannerType::Architecture,
            scanner_version: Some("0.1.0".to_string()),
            rule_id: "arch.missing-authentication".to_string(),
            severity: Severity::High,
            confidence: Confidence::Firm,
            title: "Missing authentication reference in architecture".to_string(),
            description: "Architecture documentation does not reference authentication mechanisms"
                .to_string(),
            location: FindingLocation {
                file: artifact.path.clone(),
                line: None,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some(
                "Add authentication requirements to architecture documentation".to_string(),
            ),
            fix_effort: Some("1".to_string()),
            evidence: Some(artifact.content_summary.clone()),
            tags: vec![String::from("architecture"), String::from("authentication")],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        });
    }

    findings
}

/// Generate findings for Architecture Decision Records
fn generate_adr_findings(artifact: &ArchitectureArtifact) -> Vec<CanonicalFinding> {
    let mut findings = Vec::new();

    for decision in &artifact.decisions {
        // Check for deprecated decisions
        if decision.status == DecisionStatus::Deprecated
            || decision.status == DecisionStatus::Superseded
        {
            findings.push(CanonicalFinding {
                id: format!("ADR-{}", findings.len() + 1),
                scanner: ScannerType::Architecture,
                scanner_version: Some("0.1.0".to_string()),
                rule_id: "adr.deprecated".to_string(),
                severity: Severity::Medium,
                confidence: Confidence::Firm,
                title: format!("Deprecated ADR: {}", decision.title),
                description: format!(
                    "Decision ID {} has status: {:?}",
                    decision.id, decision.status
                ),
                location: FindingLocation {
                    file: artifact.path.clone(),
                    line: None,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: None,
                },
                cwe: None,
                cvss: None,
                remediation: Some(format!(
                    "Review and update deprecated decision {}",
                    decision.id
                )),
                fix_effort: Some("2".to_string()),
                evidence: Some(decision.context_summary.clone()),
                tags: vec![
                    String::from("architecture"),
                    String::from("adr"),
                    String::from("deprecation"),
                ],
                zt_pillars: vec![],
                cross_refs: vec![],
                grade: None,
                risk_score: None,
                reachable: None,
            });
        }
    }

    findings
}

/// Generate findings for Mermaid diagrams
fn generate_diagram_findings(artifact: &ArchitectureArtifact) -> Vec<CanonicalFinding> {
    let mut findings = Vec::new();

    // Check for missing components
    if artifact.components.is_empty() {
        findings.push(CanonicalFinding {
            id: format!("ARCH-{}", findings.len() + 1),
            scanner: ScannerType::Architecture,
            scanner_version: Some("0.1.0".to_string()),
            rule_id: "arch.missing-components".to_string(),
            severity: Severity::Low,
            confidence: Confidence::Firm,
            title: "Missing components in diagram".to_string(),
            description: "Architecture diagram does not contain any component definitions"
                .to_string(),
            location: FindingLocation {
                file: artifact.path.clone(),
                line: None,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some("Add component definitions to the diagram".to_string()),
            fix_effort: Some("2".to_string()),
            evidence: Some(artifact.content_summary.clone()),
            tags: vec![String::from("architecture"), String::from("diagram")],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        });
    }

    // Check for database components
    if artifact.components.iter().any(|c| {
        c.to_lowercase().contains("database")
            || c.to_lowercase().contains("db")
            || c.to_lowercase().contains("sql")
    }) {
        findings.push(CanonicalFinding {
            id: format!("ARCH-{}", findings.len() + 1),
            scanner: ScannerType::Architecture,
            scanner_version: Some("0.1.0".to_string()),
            rule_id: "arch.database-component".to_string(),
            severity: Severity::Low,
            confidence: Confidence::Firm,
            title: "Database component detected".to_string(),
            description: "Database component found in architecture diagram".to_string(),
            location: FindingLocation {
                file: artifact.path.clone(),
                line: None,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: Some("Review database security requirements".to_string()),
            fix_effort: Some("1".to_string()),
            evidence: Some(artifact.components.join(", ")),
            tags: vec![String::from("architecture"), String::from("database")],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        });
    }

    findings
}

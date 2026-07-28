// Context Drift Detection Scanner (Layer 8)
// Verifies that claims in agent context files (AGENTS.md, CLAUDE.md, .cursor/rules)
// still match the actual codebase state — detecting "drift" between documented
// assumptions and reality.
//
// This is an internal scanner (like arch.rs) — always available, no external binary.
//
// Pipeline: scan → normalize → dedup → LLM → GRADE → severity → chains → SCORE → ZT → report
// Layer 8 inserts findings into the standard ApeGuard pipeline so drift findings
// get the same dedup, grading, scoring, and reporting as security findings.
//
// Module layout:
//   types.rs    — shared type definitions (ContextFileType, ClaimCategory, ...)
//   discover.rs — regex patterns, file discovery, I/O helpers
//   parse.rs    — claim extraction from AGENTS.md / CLAUDE.md / .cursor/rules
//   verify.rs   — verification strategies per claim category + canonical conversion
//   mod.rs      — ContextDriftScanner (Scanner trait impl) + DriftScanResult + tests

mod discover;
mod parse;
mod types;
mod verify;

// Public API — external callers (orchestrate.rs, mcp.rs) use only these.
// Kept intentionally narrow; deeper types (ContextFileType, ClaimCategory)
// remain accessible via `context_drift::types::*` if needed in the future.
pub use types::{ContextClaim, DriftFinding, VerificationResult};
pub use verify::drift_findings_to_canonical;

// Delegate imports — items used by ContextDriftScanner::scan_drift() in mod.rs
// that live in the sibling submodules. `pub(crate)` (no `pub`) leaks nothing
// outside the crate; only `pub use` above is part of the external API.
pub(crate) use discover::discover_context_files;
pub(crate) use parse::parse_all_context_files;
pub(crate) use verify::verify_claims;

// Test-only re-exports — the inline test module uses `use super::*` to reach
// these private items from the sibling submodules. Listed explicitly here to
// keep test access auditable and avoid `pub(crate)` leakage beyond tests.
#[cfg(test)]
pub(crate) use discover::detect_file_type;
#[cfg(test)]
pub(crate) use parse::{classify_claim, parse_agents_md, parse_claude_md, parse_cursor_rules};
#[cfg(test)]
pub(crate) use types::{ClaimCategory, ContextFileRef, ContextFileType};
#[cfg(test)]
pub(crate) use verify::{
    drift_severity, extract_dep_name, extract_json_section, search_for_technology_usage,
    verify_dependency_claim, verify_path_claim, verify_single_claim, DependencyCache,
};

use crate::find::{CanonicalFinding, Severity};
use crate::scanner::{Scanner, ScannerError, ScannerType};
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════════════════════
// ContextDriftScanner (Scanner trait implementation)
// ═══════════════════════════════════════════════════════════════════════════════

/// Context Drift Scanner — detects drift between agent context files and codebase reality
pub struct ContextDriftScanner {
    /// Root path to scan for context files and verify claims against
    root: PathBuf,
    /// Maximum number of findings to return
    #[allow(dead_code)] // P3/P4: max_findings is set by builder but not yet read by scan_drift
    max_findings: usize,
    /// Whether to include "unverifiable" claims in results (default: false)
    include_unknown: bool,
}

impl ContextDriftScanner {
    /// Create a new context drift scanner
    // P3/P4: scanner construction uses with_binary(); new() not wired
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            max_findings: 100,
            include_unknown: false,
        }
    }

    /// Set whether to include unverifiable claims in results
    #[expect(dead_code)] // P3/P4: builder method not yet used in scan pipeline
    pub fn with_unknown(mut self, include: bool) -> Self {
        self.include_unknown = include;
        self
    }

    /// Set maximum findings limit
    #[expect(dead_code)] // P3/P4: builder method not yet used in scan pipeline
    pub fn with_max_findings(mut self, max: usize) -> Self {
        self.max_findings = max;
        self
    }

    /// Run a full context drift scan
    pub fn scan_drift(&self) -> DriftScanResult {
        // Discover context files
        let context_files = discover_context_files(&self.root);

        if context_files.is_empty() {
            return DriftScanResult::NoContextFiles;
        }

        // Parse all context files
        let claims = self.discover_claims();

        if claims.is_empty() {
            return DriftScanResult::NoClaims;
        }

        // Verify each claim
        let drift_findings = verify_claims(&claims, &self.root);

        // Filter unknown findings if not requested
        let filtered: Vec<DriftFinding> = if self.include_unknown {
            drift_findings
        } else {
            drift_findings
                .into_iter()
                .filter(|df| matches!(df.verification, VerificationResult::Drifted { .. }))
                .collect()
        };

        // Count by severity
        let by_severity = count_by_severity(&filtered);

        DriftScanResult::Complete {
            context_file_count: context_files.len(),
            total_claims: claims.len(),
            drift_findings: filtered,
            drift_counts: by_severity,
        }
    }

    /// Discover and parse all claims from context files
    fn discover_claims(&self) -> Vec<ContextClaim> {
        parse_all_context_files(&self.root)
    }
}

impl Default for ContextDriftScanner {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

/// Results of a context drift scan
#[derive(Debug)]
pub enum DriftScanResult {
    /// No context files found in the project
    NoContextFiles,
    /// Context files found but no claims extracted
    NoClaims,
    /// Full scan complete with drift findings
    Complete {
        context_file_count: usize,
        total_claims: usize,
        drift_findings: Vec<DriftFinding>,
        drift_counts: DriftCounts,
    },
}

/// Drift finding counts by severity
#[derive(Debug, Clone, Default)]
pub struct DriftCounts {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub info: usize,
}

fn count_by_severity(findings: &[DriftFinding]) -> DriftCounts {
    let mut counts = DriftCounts::default();
    for f in findings {
        match f.severity {
            Severity::Critical => counts.critical += 1,
            Severity::High => counts.high += 1,
            Severity::Medium => counts.medium += 1,
            Severity::Low => counts.low += 1,
            Severity::Info => counts.info += 1,
        }
    }
    counts
}

#[async_trait::async_trait]
impl Scanner for ContextDriftScanner {
    fn name(&self) -> &'static str {
        "context-drift"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::ContextDrift
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        // Context drift scanner is always installed (built-in)
        Ok(true)
    }

    async fn version(&self) -> Result<String, ScannerError> {
        Ok("0.1.0".to_string())
    }

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        // Context drift scanning is file-system based, not binary output
        Ok(b"Context drift scan complete".to_vec())
    }

    fn parse_output(&self, _raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // Context drift scanner doesn't use raw output parsing
        // It discovers context files and verifies claims directly
        Ok(vec![])
    }

    fn install_hint(&self) -> &'static str {
        "Context drift scanner is built-in to ApeGuard"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::{Confidence, Severity};
    use std::io::Write;

    /// Create a temporary file with given content and return its path
    fn temp_file(content: &str) -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let path = dir.path().join("AGENTS.md");
        let mut f = std::fs::File::create(&path).expect("failed to create temp file");
        writeln!(f, "{}", content).expect("failed to write temp file");
        (path, dir)
    }

    #[test]
    fn test_detect_file_type() {
        assert_eq!(
            detect_file_type(Path::new("/project/AGENTS.md")),
            Some(ContextFileType::AgentsMd)
        );
        assert_eq!(
            detect_file_type(Path::new("/project/CLAUDE.md")),
            Some(ContextFileType::ClaudeMd)
        );
        assert_eq!(
            detect_file_type(Path::new("/project/.cursor/rules")),
            Some(ContextFileType::CursorRules)
        );
        assert_eq!(detect_file_type(Path::new("/project/README.md")), None);
    }

    #[test]
    fn test_discover_context_files_agents_md() {
        let (_path, dir) = temp_file("# Test\nWe use React in this project.");
        let files = discover_context_files(dir.path());
        assert!(!files.is_empty(), "Should discover AGENTS.md");
        assert!(
            files
                .iter()
                .any(|f| f.file_name().unwrap_or_default() == "AGENTS.md"),
            "Should find AGENTS.md"
        );
    }

    #[test]
    fn test_parse_agents_md_dependency_claim() {
        let (_path, dir) =
            temp_file("# Project\n\n## Dependencies\n- We use React v18.2\n- We use Express");
        let agents_path = dir.path().join("AGENTS.md");
        let claims = parse_agents_md(
            &std::fs::read_to_string(&agents_path).unwrap(),
            &agents_path,
            ContextFileType::AgentsMd,
        );

        assert!(!claims.is_empty(), "Should extract claims");

        let dep_claims: Vec<_> = claims
            .iter()
            .filter(|c| matches!(c.category, ClaimCategory::Dependency))
            .collect();
        assert!(!dep_claims.is_empty(), "Should have dependency claims");

        // Check that React was extracted
        let has_react = claims.iter().any(|c| c.original_text.contains("React"));
        assert!(has_react, "Should extract React claim");
    }

    #[test]
    fn test_parse_cursor_rules_frontmatter() {
        let content = "---\ndescription: \"Always use TypeScript strict mode\"\n---\n\n## Code Style\nPrefer functional components over classes.";
        let (_path, dir) = temp_file(content);
        let rules_path = dir.path().join("AGENTS.md"); // not really a cursor rule, but tests parsing
        let claims = parse_cursor_rules(content, &rules_path, ContextFileType::CursorRules);

        assert!(
            !claims.is_empty(),
            "Should extract claims from cursor rules"
        );

        // Should extract the description from frontmatter
        // The description about TypeScript strict mode should be classified
        let has_claim = claims
            .iter()
            .any(|c| c.original_text.contains("TypeScript strict mode"));
        assert!(has_claim, "Should extract description from frontmatter");
    }

    #[test]
    fn test_dependency_cache_has_dependency() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        // Create a minimal Cargo.toml
        let mut cargo = std::fs::File::create(dir.path().join("Cargo.toml")).unwrap();
        writeln!(
            cargo,
            "[dependencies]\nserde = \"1.0\"\ntokio = {{ version = \"1\", features = [\"full\"] }}"
        )
        .unwrap();

        let cache = DependencyCache::new(dir.path());
        assert!(
            cache.has_dependency("serde", None).is_some(),
            "Should find serde in Cargo.toml"
        );
        assert!(
            cache.has_dependency("tokio", None).is_some(),
            "Should find tokio in Cargo.toml"
        );
        assert!(
            cache.has_dependency("nonexistent", None).is_none(),
            "Should not find nonexistent dep"
        );
    }

    #[test]
    fn test_package_json_dependency_check() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        let pkg_content = r#"{
            "dependencies": {
                "react": "^18.2.0",
                "express": "^4.18.0"
            }
        }"#;

        let mut pkg = std::fs::File::create(dir.path().join("package.json")).unwrap();
        writeln!(pkg, "{}", pkg_content).unwrap();

        let cache = DependencyCache::new(dir.path());
        assert!(
            cache.has_dependency("react", None).is_some(),
            "Should find react"
        );
        assert!(
            cache.has_dependency("express", None).is_some(),
            "Should find express"
        );
        assert!(
            cache.has_dependency("svelte", None).is_none(),
            "Should not find svelte"
        );
    }

    #[test]
    fn test_extract_dep_name_technology() {
        let result = extract_dep_name("We use PostgreSQL for our database");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "PostgreSQL");

        let result = extract_dep_name("Built with React v18.2");
        assert!(result.is_some());
        let (name, version) = result.unwrap();
        assert_eq!(name, "React");
        assert_eq!(version, Some("18.2".to_string()));
    }

    #[test]
    fn test_extract_json_section() {
        let json =
            r#"{"dependencies": {"react": "^18.0.0"}, "devDependencies": {"jest": "^29.0.0"}}"#;
        let deps = extract_json_section(json, "dependencies");
        assert!(deps.is_some(), "Should extract dependencies section");
        assert!(deps.unwrap().contains("react"), "Should contain react");
    }

    #[test]
    fn test_verify_dependency_claim_matched() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let mut cargo = std::fs::File::create(dir.path().join("Cargo.toml")).unwrap();
        writeln!(cargo, "[dependencies]\nserde = \"1.0\"").unwrap();

        let cache = DependencyCache::new(dir.path());
        let claim = ContextClaim {
            original_text: "We use serde for serialization".to_string(),
            category: ClaimCategory::Dependency,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(1),
                section: None,
            },
            extraction_confidence: Confidence::Firm,
        };

        let result = verify_dependency_claim(&claim, dir.path(), &cache);
        assert!(
            matches!(result, VerificationResult::Matched { .. }),
            "Should match serde as dependency"
        );
    }

    #[test]
    fn test_verify_dependency_claim_drifted() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        // No Cargo.toml, no package.json
        let cache = DependencyCache::new(dir.path());
        let claim = ContextClaim {
            original_text: "We use React v18 for the frontend".to_string(),
            category: ClaimCategory::Dependency,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(1),
                section: None,
            },
            extraction_confidence: Confidence::Firm,
        };

        let result = verify_dependency_claim(&claim, dir.path(), &cache);
        assert!(
            matches!(result, VerificationResult::Drifted { .. })
                || matches!(result, VerificationResult::Unknown { .. }),
            "React should not be found in empty project"
        );
    }

    #[test]
    fn test_verify_path_claim_exists() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let src_dir = dir.path().join("src/components");
        std::fs::create_dir_all(&src_dir).unwrap();

        let claim = ContextClaim {
            original_text: "Components are in `src/components`".to_string(),
            category: ClaimCategory::Path,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(1),
                section: None,
            },
            extraction_confidence: Confidence::Firm,
        };

        let result = verify_path_claim(&claim, dir.path());
        assert!(
            matches!(result, VerificationResult::Matched { .. }),
            "Path should exist"
        );
    }

    #[test]
    fn test_verify_path_claim_missing() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        let claim = ContextClaim {
            original_text: "Config is in `config/settings.yaml`".to_string(),
            category: ClaimCategory::Path,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(1),
                section: None,
            },
            extraction_confidence: Confidence::Firm,
        };

        let result = verify_path_claim(&claim, dir.path());
        assert!(
            matches!(result, VerificationResult::Drifted { .. }),
            "Path should not exist"
        );
    }

    #[test]
    fn test_security_claim_jwt_verified() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let mut cargo = std::fs::File::create(dir.path().join("Cargo.toml")).unwrap();
        writeln!(cargo, "[dependencies]\njsonwebtoken = \"9.0\"").unwrap();

        let claim = ContextClaim {
            original_text: "Uses JWT for authentication".to_string(),
            category: ClaimCategory::Security,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(1),
                section: None,
            },
            extraction_confidence: Confidence::Firm,
        };

        let result = verify_single_claim(&claim, dir.path(), &DependencyCache::new(dir.path()));
        assert!(
            matches!(result, VerificationResult::Matched { .. }),
            "JWT claim should be verified"
        );
    }

    #[test]
    fn test_full_drift_scan_no_context_files() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let scanner = ContextDriftScanner::new(dir.path());
        let result = scanner.scan_drift();
        assert!(
            matches!(result, DriftScanResult::NoContextFiles),
            "Empty dir should have no context files"
        );
    }

    #[test]
    fn test_full_drift_scan_with_claims() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        // Create AGENTS.md with claims
        let mut agents = std::fs::File::create(dir.path().join("AGENTS.md")).unwrap();
        writeln!(agents, "# Project\nWe use serde for serialization.").unwrap();

        // Create Cargo.toml with serde
        let mut cargo = std::fs::File::create(dir.path().join("Cargo.toml")).unwrap();
        writeln!(cargo, "[dependencies]\nserde = \"1.0\"").unwrap();

        let scanner = ContextDriftScanner::new(dir.path());
        let result = scanner.scan_drift();

        match result {
            DriftScanResult::Complete {
                total_claims,
                drift_findings,
                ..
            } => {
                assert!(total_claims > 0, "Should have claims");
                // serde claim should match, but other claims may or may not drift
                // Just verify the process works
                println!(
                    "Claims: {}, Drift findings: {}",
                    total_claims,
                    drift_findings.len()
                );
            }
            _ => panic!("Should have completed scan"),
        }
    }

    #[test]
    fn test_drift_findings_to_canonical_conversion() {
        let claim = ContextClaim {
            original_text: "We use nonexistent-dep".to_string(),
            category: ClaimCategory::Dependency,
            source_file: ContextFileRef {
                file_path: PathBuf::from("AGENTS.md"),
                file_type: ContextFileType::AgentsMd,
                line_number: Some(5),
                section: Some("Dependencies".to_string()),
            },
            extraction_confidence: Confidence::Firm,
        };

        let drift = DriftFinding {
            claim,
            verification: VerificationResult::Drifted {
                expected: "nonexistent-dep should be a dependency".to_string(),
                actual: "nonexistent-dep not found in any dependency file".to_string(),
            },
            severity: Severity::Medium,
        };

        let canonical = drift_findings_to_canonical(&[drift]);
        assert_eq!(canonical.len(), 1);
        assert_eq!(canonical[0].scanner, ScannerType::ContextDrift);
        assert!(canonical[0].rule_id.starts_with("context-drift."));
        assert!(canonical[0].id.starts_with("CTX-"));
        assert!(canonical[0].tags.contains(&"context-drift".to_string()));
    }

    #[test]
    fn test_classify_claim_dependency() {
        assert!(matches!(
            classify_claim("We use React v18", "we use react v18", None),
            ClaimCategory::Dependency
        ));
    }

    #[test]
    fn test_classify_claim_security() {
        assert!(matches!(
            classify_claim(
                "Uses JWT for authentication",
                "uses jwt for authentication",
                None
            ),
            ClaimCategory::Security
        ));
    }

    #[test]
    fn test_classify_claim_convention() {
        assert!(matches!(
            classify_claim(
                "Always use TypeScript strict mode",
                "always use typescript strict mode",
                None
            ),
            ClaimCategory::Convention
        ));
    }

    #[test]
    fn test_parse_claude_md_structure() {
        let content = "# CLAUDE.md\n\n## Dependencies\n- We use PostgreSQL\n\n## Code Style\n- Prefer async/await over callbacks";
        let path = Path::new("/test/CLAUDE.md");
        let claims = parse_claude_md(content, path, ContextFileType::ClaudeMd);

        assert!(!claims.is_empty(), "Should extract claims from CLAUDE.md");

        let dep_claims: Vec<_> = claims
            .iter()
            .filter(|c| matches!(c.category, ClaimCategory::Dependency))
            .collect();
        assert!(!dep_claims.is_empty(), "Should have dependency claims");

        let convention_claims: Vec<_> = claims
            .iter()
            .filter(|c| matches!(c.category, ClaimCategory::Convention))
            .collect();
        assert!(
            !convention_claims.is_empty(),
            "Should have convention claims"
        );
    }

    #[test]
    fn test_technology_search_rust() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        // Create Cargo.toml with tokio (implies async Rust)
        let mut cargo = std::fs::File::create(dir.path().join("Cargo.toml")).unwrap();
        writeln!(
            cargo,
            "[dependencies]\ntokio = {{ version = \"1\", features = [\"full\"] }}"
        )
        .unwrap();

        let result = search_for_technology_usage("tokio", dir.path());
        assert!(result.is_some(), "Should find tokio in Cargo.toml");
    }

    #[test]
    fn test_go_mod_dependency_check() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        let mut go_mod = std::fs::File::create(dir.path().join("go.mod")).unwrap();
        writeln!(go_mod, "module example.com/myapp\n\ngo 1.21\n\nrequire (\n\tgithub.com/gin-gonic/gin v1.9.0\n)").unwrap();

        let cache = DependencyCache::new(dir.path());
        assert!(
            cache.has_dependency("gin", None).is_some(),
            "Should find gin in go.mod"
        );
        assert!(
            cache.has_dependency("gin-gonic", None).is_some(),
            "Should find gin-gonic in go.mod"
        );
    }

    #[test]
    fn test_verification_result_types() {
        // Just verify the enum compiles and matches
        let matched = VerificationResult::Matched {
            evidence: "found".into(),
        };
        let drifted = VerificationResult::Drifted {
            expected: "a".into(),
            actual: "b".into(),
        };
        let unknown = VerificationResult::Unknown {
            reason: "cannot verify".into(),
        };

        assert!(matches!(matched, VerificationResult::Matched { .. }));
        assert!(matches!(drifted, VerificationResult::Drifted { .. }));
        assert!(matches!(unknown, VerificationResult::Unknown { .. }));
    }

    #[test]
    fn test_empty_context_file_no_claims() {
        let content = "";
        let path = Path::new("/test/AGENTS.md");
        let claims = parse_agents_md(content, path, ContextFileType::AgentsMd);
        assert!(claims.is_empty(), "Empty file should produce no claims");
    }

    #[test]
    fn test_scanner_name_and_type() {
        let scanner = ContextDriftScanner::new(Path::new("."));
        assert_eq!(scanner.name(), "context-drift");
        assert_eq!(scanner.scanner_type(), ScannerType::ContextDrift);
    }

    #[test]
    fn test_drift_severity_mapping() {
        assert_eq!(
            drift_severity(
                &ClaimCategory::Security,
                &VerificationResult::Drifted {
                    expected: "".into(),
                    actual: "".into()
                }
            ),
            Severity::High
        );
        assert_eq!(
            drift_severity(
                &ClaimCategory::Dependency,
                &VerificationResult::Drifted {
                    expected: "".into(),
                    actual: "".into()
                }
            ),
            Severity::Medium
        );
        assert_eq!(
            drift_severity(
                &ClaimCategory::Convention,
                &VerificationResult::Unknown {
                    reason: "test".into()
                }
            ),
            Severity::Info
        );
    }
}

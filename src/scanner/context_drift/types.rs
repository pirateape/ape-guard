// Context Drift Detection — Type Definitions
// Shared types for context file parsing, claim extraction, and verification.
use crate::find::{Confidence, Severity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported agent context file types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextFileType {
    /// AGENTS.md — standard agent instruction file (OpenCode standard)
    AgentsMd,
    /// CLAUDE.md — Claude Code project instructions
    ClaudeMd,
    /// .cursor/rules — Cursor AI rules (each file is one rule)
    CursorRules,
}

impl ContextFileType {
    pub(crate) fn file_names(&self) -> &[&str] {
        match self {
            ContextFileType::AgentsMd => &["AGENTS.md", "AGENTS", ".agenda.md"],
            ContextFileType::ClaudeMd => &["CLAUDE.md", "CLAUDE"],
            ContextFileType::CursorRules => &[".cursor/rules"],
        }
    }
}

/// Categories of claims that can appear in agent context files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimCategory {
    /// "We use React Router v6" — technology dependency claims
    Dependency,
    /// "Components live in src/components/" — file/folder structure claims
    Path,
    /// "API routes prefixed with /api/v1" — architecture pattern claims
    Architecture,
    /// "Use functional components" — coding convention claims
    Convention,
    /// "Rate limiting on all endpoints" — security practice claims
    Security,
    /// "The app handles file uploads" — general semantic claims
    Semantic,
    /// "Run tests with `npm test`" — build/run command claims
    Command,
}

impl ClaimCategory {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ClaimCategory::Dependency => "dependency",
            ClaimCategory::Path => "path",
            ClaimCategory::Architecture => "architecture",
            ClaimCategory::Convention => "convention",
            ClaimCategory::Security => "security",
            ClaimCategory::Semantic => "semantic",
            ClaimCategory::Command => "command",
        }
    }

    pub(crate) fn default_severity(&self) -> Severity {
        match self {
            ClaimCategory::Dependency => Severity::Medium,
            ClaimCategory::Path => Severity::Low,
            ClaimCategory::Architecture => Severity::Medium,
            ClaimCategory::Convention => Severity::Info,
            ClaimCategory::Security => Severity::High,
            ClaimCategory::Semantic => Severity::Low,
            ClaimCategory::Command => Severity::Low,
        }
    }
}

/// A single claim extracted from a context file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextClaim {
    /// The raw text of the claim as written in the file
    pub original_text: String,
    /// What category of claim this is
    pub category: ClaimCategory,
    /// The source context file
    pub source_file: ContextFileRef,
    /// How confident we are this is a deliberate claim (vs incidental text)
    pub extraction_confidence: Confidence,
}

/// Reference back to where a claim was extracted from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFileRef {
    pub file_path: PathBuf,
    pub file_type: ContextFileType,
    pub line_number: Option<u32>,
    pub section: Option<String>,
}

/// Result of verifying a single claim against the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationResult {
    /// Claim matches reality — no drift
    Matched { evidence: String },
    /// Claim does not match — drift detected
    Drifted { expected: String, actual: String },
    /// Could not verify (e.g., ambiguous claim)
    Unknown { reason: String },
    /// Claim type not yet supported by verifier
    NotVerifiable { reason: String },
}

/// A drift finding linking a claim to its verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftFinding {
    pub claim: ContextClaim,
    pub verification: VerificationResult,
    pub severity: Severity,
}

/// Configuration for a single context file's parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[expect(dead_code)] // P3/P4: context file config not yet wired into config loading
pub struct ContextFileConfig {
    /// Path to the context file relative to project root
    pub path: PathBuf,
    /// File type hint (auto-detected if not specified)
    pub file_type: Option<ContextFileType>,
    /// Whether to include this file in drift detection
    pub enabled: bool,
}

impl Default for ContextFileConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            file_type: None,
            enabled: true,
        }
    }
}

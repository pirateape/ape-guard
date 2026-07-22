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
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

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
    fn file_names(&self) -> &[&str] {
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
    fn as_str(&self) -> &'static str {
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

    fn default_severity(&self) -> Severity {
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

// ═══════════════════════════════════════════════════════════════════════════════
// Lazy-compiled regex patterns
// ═══════════════════════════════════════════════════════════════════════════════

fn dep_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(?:we\s+)?(?:use|run|depend\s+on|built\s+(?:with|on)|powered\s+by|via)\s+(.+?)(?:$|\.|,|\s+for\s+)"
        ).expect("invalid dep regex")
    })
}

fn version_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)v?(\d+\.\d+(?:\.\d+)?(?:-(?:alpha|beta|rc|stable)\.?\d*)?)")
            .expect("invalid version regex")
    })
}

fn path_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(?:in|at|under|located\s+(?:in|at)|stored\s+in)\s+`?([\w/\.\-_]+)`?")
            .expect("invalid path regex")
    })
}

fn technology_keyword_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\b(PostgreSQL|MySQL|SQLite|Redis|MongoDB|DynamoDB|React|Vue|Angular|Svelte|Next\.js|Nuxt|Express|Fastify|Django|Flask|FastAPI|Rails|Spring|Laravel|Symfony|ASP\.NET|Node\.js|Deno|Bun|Rust|Go|Python|TypeScript|JavaScript|Kotlin|Swift|GraphQL|gRPC|REST|WebSocket|tRPC|Prisma|Drizzle|Sequelize|TypeORM|Docker|Kubernetes|AWS|GCP|Azure|Terraform|Ansible|JWT|OAuth|OIDC|SAML|Redis|Kafka|RabbitMQ|NATS|S3|CloudFront|Vercel|Netlify|Railway|Supabase|Firebase)\b").expect("invalid tech regex")
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Context File Discovery
// ═══════════════════════════════════════════════════════════════════════════════

/// Discover all agent context files in the project root
pub fn discover_context_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let file_types = [
        ContextFileType::AgentsMd,
        ContextFileType::ClaudeMd,
        ContextFileType::CursorRules,
    ];

    for ft in &file_types {
        for name in ft.file_names() {
            let candidate = root.join(name);
            if candidate.exists() && candidate.is_file() {
                files.push(candidate);
            }
        }
    }

    files
}

/// Detect the context file type from its path
pub fn detect_file_type(path: &Path) -> Option<ContextFileType> {
    let file_name = path.file_name()?.to_str()?;
    let file_str = path.to_str().unwrap_or("");

    if file_str.contains(".cursor/rules") || file_str.ends_with(".cursor/rules") {
        return Some(ContextFileType::CursorRules);
    }

    match file_name {
        "AGENTS.md" | "AGENTS" => Some(ContextFileType::AgentsMd),
        "CLAUDE.md" | "CLAUDE" => Some(ContextFileType::ClaudeMd),
        _ => None,
    }
}

/// Read a file and return its content as a string, or None if unreadable
fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Single-line claim extraction helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if a line looks like it might contain a claim
fn is_claim_line(line: &str) -> bool {
    let trimmed = line.trim();

    // Skip headers, empty lines, code fences, comments/todos
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("```")
        || trimmed.starts_with("<!--")
        || trimmed.starts_with("//")
        || trimmed.starts_with("TODO")
        || trimmed.starts_with("FIXME")
    {
        return false;
    }

    // Must contain actionable content
    let lower = trimmed.to_lowercase();

    // Strong claim indicators — check both " in text" and "start of text" variants
    let strong_indicators: &[&str] = &[
        " we use ",
        " we run ",
        " we depend ",
        " built with ",
        " powered by ",
        " written in ",
        " uses ",
        " uses:",
        " this project ",
        " the app ",
        " the api ",
        " stack:",
        " technology ",
        " technologies ",
        " framework:",
        " library:",
        " database:",
        " hosted on ",
        " deployed to ",
        " running on ",
        " architecture:",
        " patterns:",
        " authentication:",
        " authorization:",
        " please do ",
        " always ",
        " never ",
        " prefer ",
        " avoid ",
    ];

    // Also check start-of-string variants (text beginning with a claim indicator)
    let start_indicators: &[&str] = &[
        "we use ",
        "we run ",
        "we depend ",
        "uses ",
        "uses:",
        "never ",
        "always ",
        "prefer ",
        "avoid ",
    ];

    strong_indicators.iter().any(|i| lower.contains(i))
        || start_indicators.iter().any(|i| lower.starts_with(i))
}

/// Try to extract a single claim from a line of text
fn extract_claim_from_line(
    line: &str,
    line_number: u32,
    source_path: &Path,
    file_type: ContextFileType,
    current_section: Option<&str>,
) -> Option<ContextClaim> {
    let trimmed = line.trim();
    if !is_claim_line(trimmed) {
        return None;
    }

    let lower = trimmed.to_lowercase();

    // Determine category
    let category = classify_claim(trimmed, &lower, current_section);

    let extraction_confidence = match &category {
        ClaimCategory::Dependency => Confidence::Firm,
        ClaimCategory::Path => Confidence::Firm,
        ClaimCategory::Command => Confidence::Firm,
        ClaimCategory::Security => Confidence::Firm,
        ClaimCategory::Architecture => Confidence::Firm,
        ClaimCategory::Convention => Confidence::Tentative,
        ClaimCategory::Semantic => Confidence::Tentative,
    };

    Some(ContextClaim {
        original_text: trimmed.to_string(),
        category,
        source_file: ContextFileRef {
            file_path: source_path.to_path_buf(),
            file_type,
            line_number: Some(line_number),
            section: current_section.map(|s| s.to_string()),
        },
        extraction_confidence,
    })
}

/// Classify what kind of claim a line represents
fn classify_claim(text: &str, lower: &str, section: Option<&str>) -> ClaimCategory {
    // Check section context first — strongest signal
    if let Some(sec) = section {
        let sec_lower = sec.to_lowercase();
        if sec_lower.contains("dependency") || sec_lower.contains("dependencies") {
            return ClaimCategory::Dependency;
        }
        if sec_lower.contains("security") || sec_lower.contains("auth") {
            return ClaimCategory::Security;
        }
        if (sec_lower.contains("command")
            || sec_lower.contains("build")
            || sec_lower.contains("test"))
            && (lower.contains("run") || lower.contains("command"))
        {
            return ClaimCategory::Command;
        }
        if sec_lower.contains("architecture") || sec_lower.contains("design") {
            return ClaimCategory::Architecture;
        }
        if sec_lower.contains("style") || sec_lower.contains("convention") {
            return ClaimCategory::Convention;
        }
    }

    // Security detection — check BEFORE dep_pattern to avoid false matches
    // "Uses JWT for authentication" should be Security, not Dependency
    if lower.contains("authentication")
        || lower.contains("authorization")
        || lower.contains("permission")
        || lower.contains("encrypt")
        || lower.contains("secret")
        || lower.contains("rate limit")
        || lower.contains("cors")
        || lower.contains("csrf")
        || lower.contains("helmet")
    {
        return ClaimCategory::Security;
    }

    // Convention detection — check before dep_pattern
    if lower.starts_with("prefer")
        || lower.starts_with("avoid")
        || lower.starts_with("always")
        || lower.starts_with("never")
        || lower.contains("should")
        || lower.contains("must ")
    {
        return ClaimCategory::Convention;
    }

    // Check for dependency keywords
    if dep_pattern().is_match(text) || technology_keyword_pattern().is_match(text) {
        if lower.contains("depend") || lower.contains("use ") || lower.contains("uses") {
            return ClaimCategory::Dependency;
        }
        // Path-like patterns
        if lower.contains(" in ") || lower.contains(" at ") || lower.contains(" under ") {
            return ClaimCategory::Path;
        }
        return ClaimCategory::Architecture;
    }

    // Command pattern detection
    if lower.contains("run ")
        || lower.contains("command")
        || lower.contains("execute")
        || (lower.starts_with("npm ") || lower.starts_with("cargo ") || lower.starts_with("make "))
    {
        return ClaimCategory::Command;
    }

    // Path detection via backticks
    if text.contains('`')
        && (lower.contains("file") || lower.contains("director") || lower.contains("folder"))
    {
        return ClaimCategory::Path;
    }

    // Token/keyword-based security (but not authentication — checked above)
    if lower.contains("token") {
        return ClaimCategory::Security;
    }

    // Default — treat as semantic claim
    ClaimCategory::Semantic
}

// ═══════════════════════════════════════════════════════════════════════════════
// AGENTS.md Parser
// ═══════════════════════════════════════════════════════════════════════════════

/// Parse an AGENTS.md file into extracted claims
fn parse_agents_md(
    content: &str,
    source_path: &Path,
    file_type: ContextFileType,
) -> Vec<ContextClaim> {
    let mut claims = Vec::new();
    let mut current_section: Option<String> = None;

    for (i, line) in content.lines().enumerate() {
        let line_number = (i + 1) as u32;
        let trimmed = line.trim();

        // Track sections
        if trimmed.starts_with("## ") {
            current_section = Some(trimmed.trim_start_matches("## ").to_string());
            continue;
        }

        // Skip non-content lines
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("```") {
            continue;
        }

        // Extract claims from bullet points and regular lines
        let text = trimmed.trim_start_matches("- ").trim_start_matches("* ");

        if let Some(claim) = extract_claim_from_line(
            text,
            line_number,
            source_path,
            file_type,
            current_section.as_deref(),
        ) {
            claims.push(claim);
        }
    }

    claims
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLAUDE.md Parser
// ═══════════════════════════════════════════════════════════════════════════════

/// Parse a CLAUDE.md file into extracted claims
fn parse_claude_md(
    content: &str,
    source_path: &Path,
    file_type: ContextFileType,
) -> Vec<ContextClaim> {
    // CLAUDE.md has a similar structure to AGENTS.md but with more
    // structured known sections. Reuse the same parser — the section
    // context helps classify claims more accurately.
    parse_agents_md(content, source_path, file_type)
}

// ═══════════════════════════════════════════════════════════════════════════════
// .cursor/rules Parser
// ═══════════════════════════════════════════════════════════════════════════════

/// Parse a .cursor/rules file into extracted claims
fn parse_cursor_rules(
    content: &str,
    source_path: &Path,
    file_type: ContextFileType,
) -> Vec<ContextClaim> {
    let mut claims = Vec::new();

    // .cursor/rules files have YAML frontmatter between --- delimiters
    // followed by the rule body. We extract claims from both parts.
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    // YAML frontmatter (between first and second ---)
    if parts.len() >= 3 {
        let frontmatter = parts[1];
        for (i, line) in frontmatter.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // YAML key-value pairs
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');

                // Only process description and key metadata fields
                if matches!(key, "description" | "title") && !value.is_empty() {
                    if let Some(claim) = extract_claim_from_line(
                        value,
                        (i + 1) as u32,
                        source_path,
                        file_type,
                        Some("metadata"),
                    ) {
                        claims.push(claim);
                    }
                }
            }
        }

        // Rule body (after second ---)
        let body = parts[2];
        let mut current_section: Option<String> = None;

        for (i, line) in body.lines().enumerate() {
            let line_number = (i + 1 + parts[0].len() + parts[1].len() + 2) as u32; // approximate
            let trimmed = line.trim();

            if trimmed.starts_with("## ") {
                current_section = Some(trimmed.trim_start_matches("## ").to_string());
                continue;
            }

            if let Some(claim) = extract_claim_from_line(
                trimmed,
                line_number,
                source_path,
                file_type,
                current_section.as_deref(),
            ) {
                claims.push(claim);
            }
        }
    }

    claims
}

// ═══════════════════════════════════════════════════════════════════════════════
// Parser Router
// ═══════════════════════════════════════════════════════════════════════════════

/// Parse a context file into a list of claims, based on its detected or specified type
pub fn parse_context_file(path: &Path, file_type: ContextFileType) -> Vec<ContextClaim> {
    let content = match read_file(path) {
        Some(c) => c,
        None => return vec![],
    };

    match file_type {
        ContextFileType::AgentsMd => parse_agents_md(&content, path, file_type),
        ContextFileType::ClaudeMd => parse_claude_md(&content, path, file_type),
        ContextFileType::CursorRules => parse_cursor_rules(&content, path, file_type),
    }
}

/// Parse all discovered context files into a combined list of claims
pub fn parse_all_context_files(root: &Path) -> Vec<ContextClaim> {
    let mut all_claims = Vec::new();
    let files = discover_context_files(root);

    for file_path in &files {
        if let Some(ft) = detect_file_type(file_path) {
            let claims = parse_context_file(file_path, ft);
            all_claims.extend(claims);
        }
    }

    all_claims
}

// ═══════════════════════════════════════════════════════════════════════════════
// Verification Engine
// ═══════════════════════════════════════════════════════════════════════════════

/// Main verification function — runs all claims against the codebase
pub fn verify_claims(claims: &[ContextClaim], root: &Path) -> Vec<DriftFinding> {
    let mut findings = Vec::new();
    let dep_cache = DependencyCache::new(root);

    for claim in claims {
        let result = verify_single_claim(claim, root, &dep_cache);
        let severity = drift_severity(&claim.category, &result);

        if matches!(
            result,
            VerificationResult::Drifted { .. } | VerificationResult::Unknown { .. }
        ) {
            findings.push(DriftFinding {
                claim: claim.clone(),
                verification: result,
                severity,
            });
        }
    }

    findings
}

/// Determine severity of a drift finding based on claim category and verification result
fn drift_severity(category: &ClaimCategory, result: &VerificationResult) -> Severity {
    match result {
        VerificationResult::Drifted { .. } => category.default_severity(),
        VerificationResult::Unknown { .. } => {
            // Unknown results are less severe than definite drift
            match category.default_severity() {
                Severity::Critical | Severity::High => Severity::Medium,
                Severity::Medium => Severity::Low,
                Severity::Low => Severity::Info,
                Severity::Info => Severity::Info,
            }
        }
        _ => Severity::Info, // Not drifted = info
    }
}

/// Verify a single claim against the codebase
fn verify_single_claim(
    claim: &ContextClaim,
    root: &Path,
    dep_cache: &DependencyCache,
) -> VerificationResult {
    match &claim.category {
        ClaimCategory::Dependency => verify_dependency_claim(claim, root, dep_cache),
        ClaimCategory::Path => verify_path_claim(claim, root),
        ClaimCategory::Architecture => verify_architecture_claim(claim, root),
        ClaimCategory::Convention => verify_convention_claim(claim, root),
        ClaimCategory::Security => verify_security_claim(claim, root),
        ClaimCategory::Command => verify_command_claim(claim, root),
        ClaimCategory::Semantic => verify_semantic_claim(claim, root),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Dependency Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

/// Cache of parsed dependency files to avoid re-parsing
struct DependencyCache {
    cargo_toml: Option<String>,
    package_json: Option<String>,
    pyproject_toml: Option<String>,
    go_mod: Option<String>,
}

impl DependencyCache {
    fn new(root: &Path) -> Self {
        Self {
            cargo_toml: read_file(&root.join("Cargo.toml")),
            package_json: read_file(&root.join("package.json")),
            pyproject_toml: read_file(&root.join("pyproject.toml")),
            go_mod: read_file(&root.join("go.mod")),
        }
    }

    fn has_dependency(&self, dep_name: &str, version_hint: Option<&str>) -> Option<String> {
        let lower = dep_name.to_lowercase();

        // Check Cargo.toml (Rust)
        if let Some(cargo) = &self.cargo_toml {
            let cargo_lower = cargo.to_lowercase();
            if cargo_lower.contains(&lower) {
                return Some("Cargo.toml".to_string());
            }
        }

        // Check package.json (Node.js)
        if let Some(pkg) = &self.package_json {
            // Parse all dependency sections
            for section_name in &[
                "dependencies",
                "devDependencies",
                "peerDependencies",
                "optionalDependencies",
            ] {
                if let Some(deps_section) = extract_json_section(pkg, section_name) {
                    let deps_lower = deps_section.to_lowercase();
                    if deps_lower.contains(&lower) {
                        if let Some(ver) = version_hint {
                            // Check version matches
                            if deps_section.contains(ver) {
                                return Some(format!("package.json ({})", section_name));
                            }
                            return Some(format!(
                                "package.json ({}) — version mismatch",
                                section_name
                            ));
                        }
                        return Some(format!("package.json ({})", section_name));
                    }
                }
            }
        }

        // Check pyproject.toml (Python)
        if let Some(pyproject) = &self.pyproject_toml {
            if pyproject.to_lowercase().contains(&lower) {
                return Some("pyproject.toml".to_string());
            }
        }

        // Check go.mod (Go)
        if let Some(go_mod) = &self.go_mod {
            if go_mod.to_lowercase().contains(&lower) {
                return Some("go.mod".to_string());
            }
        }

        None
    }
}

/// Crude JSON section extractor — finds the value of a top-level key
/// without pulling in a JSON parser dependency
fn extract_json_section(json: &str, key: &str) -> Option<String> {
    // Find `"key": {`
    let search = format!("\"{}\"\\s*:", key);
    let re = Regex::new(&search).ok()?;

    if let Some(m) = re.find(json) {
        let rest = &json[m.end()..];
        // Find the opening brace
        if let Some(start) = rest.find('{') {
            // Track brace depth
            let mut depth = 0;
            let mut end = start;
            for (i, ch) in rest[start..].char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = start + i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            return Some(rest[start..end].to_string());
        }
    }
    None
}

/// Extract a dependency name from a claim text
fn extract_dep_name(text: &str) -> Option<(String, Option<String>)> {
    // Try to match "<Technology> v<version>" or "<Technology> <version>"
    let tech = technology_keyword_pattern().find(text)?;
    let tech_name = tech.as_str().to_string();

    // Check for a version after the technology name
    // Use capture group to strip the optional 'v' prefix
    let after_tech = &text[tech.end()..];
    let version = version_pattern()
        .captures(after_tech)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    Some((tech_name, version))
}

fn verify_dependency_claim(
    claim: &ContextClaim,
    _root: &Path,
    dep_cache: &DependencyCache,
) -> VerificationResult {
    let text = &claim.original_text;

    // Try to extract dependency name from text (known technology keywords)
    if let Some((dep_name, version_hint)) = extract_dep_name(text) {
        // Try exact match first
        if dep_cache
            .has_dependency(&dep_name, version_hint.as_deref())
            .is_some()
        {
            return VerificationResult::Matched {
                evidence: format!("Found '{}' in project dependencies", dep_name),
            };
        }

        // Try lowercase match (some dependencies use different casing)
        if dep_cache
            .has_dependency(&dep_name.to_lowercase(), version_hint.as_deref())
            .is_some()
        {
            return VerificationResult::Matched {
                evidence: format!(
                    "Found '{}' in project dependencies (case-insensitive match)",
                    dep_name
                ),
            };
        }

        return VerificationResult::Drifted {
            expected: format!("'{}' should be a project dependency", dep_name),
            actual: format!(
                "'{}' not found in Cargo.toml, package.json, pyproject.toml, or go.mod",
                dep_name
            ),
        };
    }

    // Fallback: extract any word after "use/uses/depend on" patterns
    // This catches generic dependency names like "serde", "lodash", etc.
    let use_re = Regex::new(r"(?i)(?:we\s+)?(?:use|uses|depend\s+on)\s+(\w[\w\-]*)").ok();
    if let Some(re) = use_re {
        if let Some(caps) = re.captures(text) {
            if let Some(dep_word) = caps.get(1) {
                let dep_name = dep_word.as_str();

                if dep_cache.has_dependency(dep_name, None).is_some() {
                    return VerificationResult::Matched {
                        evidence: format!("Found '{}' in project dependencies", dep_name),
                    };
                }

                if dep_cache
                    .has_dependency(&dep_name.to_lowercase(), None)
                    .is_some()
                {
                    return VerificationResult::Matched {
                        evidence: format!(
                            "Found '{}' in project dependencies (case-insensitive match)",
                            dep_name
                        ),
                    };
                }

                return VerificationResult::Drifted {
                    expected: format!("'{}' should be a project dependency", dep_name),
                    actual: format!(
                        "'{}' not found in Cargo.toml, package.json, pyproject.toml, or go.mod",
                        dep_name
                    ),
                };
            }
        }
    }

    // Last resort: couldn't identify any dependency name in the text
    VerificationResult::Unknown {
        reason: format!("Could not extract a dependency name from: '{}'", text),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Path Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_path_claim(claim: &ContextClaim, root: &Path) -> VerificationResult {
    let text = &claim.original_text;

    // Try to extract a path from backticks or known patterns
    if let Some(path_str) = path_pattern().captures(text).and_then(|c| c.get(1)) {
        let claimed_path = path_str.as_str().trim().trim_matches('`');
        let full_path = root.join(claimed_path);

        if full_path.exists() {
            return VerificationResult::Matched {
                evidence: format!("Path '{}' exists", claimed_path),
            };
        }

        return VerificationResult::Drifted {
            expected: format!("Path '{}' should exist", claimed_path),
            actual: format!("Path '{}' does not exist on disk", claimed_path),
        };
    }

    // Check for backtick paths: `src/components/`
    let backtick_re = Regex::new(r"`([\w/\.\-_]+)`").expect("invalid backtick regex");
    if let Some(m) = backtick_re.captures(text) {
        let claimed_path = m
            .get(1)
            .expect("backtick_re must have capture group 1")
            .as_str();
        let full_path = root.join(claimed_path);

        if full_path.exists() {
            return VerificationResult::Matched {
                evidence: format!("Path '{}' exists", claimed_path),
            };
        }

        return VerificationResult::Drifted {
            expected: format!("Path '{}' should exist", claimed_path),
            actual: format!("Path '{}' does not exist on disk", claimed_path),
        };
    }

    VerificationResult::Unknown {
        reason: format!("Could not extract a path from: '{}'", text),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Architecture Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_architecture_claim(claim: &ContextClaim, root: &Path) -> VerificationResult {
    let text = &claim.original_text;

    // Check for technology mentions and verify they're imported/used
    let tech_matches: Vec<String> = technology_keyword_pattern()
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect();

    if tech_matches.is_empty() {
        return VerificationResult::Unknown {
            reason: format!("Could not extract architecture pattern from: '{}'", text),
        };
    }

    // For each mentioned technology, check if there's evidence in the codebase
    let mut confirmed = Vec::new();
    let mut missing = Vec::new();

    for tech in &tech_matches {
        let evidence = search_for_technology_usage(tech, root);
        if let Some(ev) = evidence {
            confirmed.push(ev);
        } else {
            missing.push(tech.clone());
        }
    }

    if missing.is_empty() && !confirmed.is_empty() {
        VerificationResult::Matched {
            evidence: format!("Architecture pattern confirmed: {}", confirmed.join(", ")),
        }
    } else if !missing.is_empty() && confirmed.is_empty() {
        VerificationResult::Drifted {
            expected: format!(
                "Technology '{}' should be used in the codebase",
                missing.join(", ")
            ),
            actual: format!("No usage found of: {}", missing.join(", ")),
        }
    } else if !missing.is_empty() {
        // Partial match
        VerificationResult::Matched {
            evidence: format!(
                "Partially matched: {} found. Missing: {}",
                confirmed.join(", "),
                missing.join(", ")
            ),
        }
    } else {
        VerificationResult::Unknown {
            reason: "No technology patterns could be verified".to_string(),
        }
    }
}

/// Search for evidence of a technology being used in the codebase
fn search_for_technology_usage(tech: &str, root: &Path) -> Option<String> {
    let lower_tech = tech.to_lowercase();

    // Check Cargo.toml
    if let Some(cargo) = read_file(&root.join("Cargo.toml")) {
        if cargo.to_lowercase().contains(&lower_tech) {
            return Some(format!("{} in Cargo.toml", tech));
        }
    }

    // Check package.json
    if let Some(pkg) = read_file(&root.join("package.json")) {
        if pkg.to_lowercase().contains(&lower_tech) {
            return Some(format!("{} in package.json", tech));
        }
    }

    // Check for npm/yarn/pnpm lock files
    for lockfile in &["package-lock.json", "yarn.lock", "pnpm-lock.yaml"] {
        if let Some(lock) = read_file(&root.join(lockfile)) {
            if lock.to_lowercase().contains(&lower_tech) {
                return Some(format!("{} in {}", tech, lockfile));
            }
        }
    }

    // Check Dockerfile
    for dockerfile in &["Dockerfile", "docker-compose.yml", "docker-compose.yaml"] {
        if let Some(df) = read_file(&root.join(dockerfile)) {
            if df.to_lowercase().contains(&lower_tech) {
                return Some(format!("{} in {}", tech, dockerfile));
            }
        }
    }

    // Check for .github/workflows
    let workflows_dir = root.join(".github/workflows");
    if workflows_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&workflows_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if content.to_lowercase().contains(&lower_tech) {
                        return Some(format!(
                            "{} in GitHub workflow {}",
                            tech,
                            entry.file_name().to_string_lossy()
                        ));
                    }
                }
            }
        }
    }

    // Check for imports in source files (common patterns)
    let import_patterns = match lower_tech.as_str() {
        "react" => vec!["import React", "from 'react'", "from \"react\""],
        "next.js" | "nextjs" => vec!["from 'next'", "from \"next\"", "next.config"],
        "express" => vec!["from 'express'", "require('express')", "express()"],
        "fastify" => vec!["from 'fastify'", "require('fastify')"],
        "django" => vec!["from django", "import django", "django.core"],
        "flask" => vec!["from flask", "import flask"],
        "fastapi" => vec!["from fastapi", "import fastapi"],
        "postgresql" | "postgres" => vec!["postgres", "postgresql", "psycopg", "pg::"],
        "redis" => vec!["redis", "ioredis", "redis-rs"],
        "docker" => vec!["FROM ", "docker-compose"],
        "typescript" => vec![".ts:", ".tsx:", "typescript", "tsconfig"],
        _ => vec![],
    };

    if !import_patterns.is_empty() {
        // Check a sample of source files for these patterns
        let src_dir = root.join("src");
        if src_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|e| {
                        matches!(
                            e.to_str(),
                            Some("ts" | "tsx" | "js" | "jsx" | "rs" | "py" | "go")
                        )
                    }) {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            for pat in &import_patterns {
                                if content.contains(pat) {
                                    return Some(format!(
                                        "{} found in {}",
                                        tech,
                                        path.file_name().unwrap_or_default().to_string_lossy()
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

// ═══════════════════════════════════════════════════════════════════════════════
// Convention Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_convention_claim(claim: &ContextClaim, _root: &Path) -> VerificationResult {
    // Convention verification is inherently heuristic.
    // We identify conventions from the claim text and try to verify them.
    let text = &claim.original_text;
    let lower = text.to_lowercase();

    let conventions: Vec<&str> = if lower.contains("prefer") || lower.contains("prefers") {
        vec!["preference detected"]
    } else if lower.contains("avoid") || lower.contains("never") {
        vec!["avoidance rule detected"]
    } else if lower.contains("always") {
        vec!["mandatory convention detected"]
    } else {
        vec![]
    };

    if conventions.is_empty() {
        return VerificationResult::Unknown {
            reason: format!("Could not determine specific convention from: '{}'", text),
        };
    }

    // Convention verification requires heuristic code analysis
    // For v1, we flag convention claims as "claim noted but verification is heuristic"
    VerificationResult::Unknown {
        reason: format!(
            "Convention claim '{}' — heuristic verification not yet implemented for v1. \
             Manual review recommended.",
            text
        ),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Security Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_security_claim(claim: &ContextClaim, root: &Path) -> VerificationResult {
    let text = &claim.original_text;
    let lower = text.to_lowercase();

    // Check for specific security practice claims
    let security_checks: Vec<(&str, Vec<&str>)> = vec![
        ("jwt", vec!["jsonwebtoken", "jwt", "JWT", "jwt-rs"]),
        ("oauth", vec!["oauth", "oauth2", "openid"]),
        (
            "rate limit",
            vec![
                "rate-limit",
                "rate_limit",
                "ratelimit",
                "express-rate-limit",
            ],
        ),
        ("cors", vec!["cors", "cors-rs", "django-cors"]),
        ("helmet", vec!["helmet"]),
        (
            "session",
            vec!["express-session", "cookie-session", "redis-session"],
        ),
        ("csrf", vec!["csrf", "csurf", "django.middleware.csrf"]),
        ("encryption", vec!["bcrypt", "argon2", "scrypt", "encrypt"]),
        ("tls", vec!["tls", "ssl", "https"]),
    ];

    for (keyword, patterns) in &security_checks {
        if lower.contains(keyword) {
            // Check if any of the patterns exist in dependency files
            let mut found_evidence = Vec::new();

            for cfg_file in &["Cargo.toml", "package.json", "pyproject.toml", "go.mod"] {
                if let Some(content) = read_file(&root.join(cfg_file)) {
                    for pat in patterns {
                        if content.contains(pat) {
                            found_evidence.push(format!("{} in {}", pat, cfg_file));
                        }
                    }
                }
            }

            if !found_evidence.is_empty() {
                return VerificationResult::Matched {
                    evidence: format!("Security practice verified: {}", found_evidence.join(", ")),
                };
            }

            return VerificationResult::Drifted {
                expected: format!("Security practice '{}' should be present", keyword),
                actual: format!(
                    "No evidence of '{}' found in Cargo.toml, package.json, pyproject.toml, or go.mod",
                    keyword
                ),
            };
        }
    }

    VerificationResult::Unknown {
        reason: format!(
            "Could not determine specific security practice from: '{}'",
            text
        ),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Command Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_command_claim(claim: &ContextClaim, _root: &Path) -> VerificationResult {
    // Command verification: check if the claimed command makes sense
    // For v1, we acknowledge the command but don't execute it
    let text = &claim.original_text;

    // Extract any command-like patterns
    let cmd_re = Regex::new(r"(?:`([^`]+)`|(\b\w+\s+\w+(?:\s+-\w+(?:\s+\w+)?)*))")
        .expect("invalid cmd regex");

    if let Some(m) = cmd_re.captures(text) {
        let cmd = m
            .get(1)
            .or_else(|| m.get(2))
            .map(|c| c.as_str())
            .unwrap_or("");
        if !cmd.is_empty() {
            // For v1, we only note the command — we don't execute it
            return VerificationResult::Unknown {
                reason: format!(
                    "Command claim '{}' — execute verification not available in v1. \
                     Manual validation recommended.",
                    cmd
                ),
            };
        }
    }

    VerificationResult::Unknown {
        reason: format!("Could not extract command from: '{}'", text),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Semantic Claim Verification
// ═══════════════════════════════════════════════════════════════════════════════

fn verify_semantic_claim(claim: &ContextClaim, _root: &Path) -> VerificationResult {
    // Semantic claims are the hardest to verify — they make broad statements
    // about the codebase. For v1, we note them as unverifiable.
    let text = &claim.original_text;

    // Heuristic: scan for any obvious falsehoods
    let lower = text.to_lowercase();

    if lower.contains("no dependencies") || lower.contains("zero dependencies") {
        // Quick check: if there's a Cargo.toml or package.json, this is suspicious
        // (handled elsewhere — semantic verification is best-effort)
    }

    VerificationResult::Unknown {
        reason: format!(
            "Semantic claim '{}' — automated verification not available in v1. \
             Manual review recommended.",
            text
        ),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Drift Finding → CanonicalFinding Conversion
// ═══════════════════════════════════════════════════════════════════════════════

/// Convert drift findings into ApeGuard CanonicalFindings for pipeline integration
pub fn drift_findings_to_canonical(findings: &[DriftFinding]) -> Vec<CanonicalFinding> {
    findings
        .iter()
        .enumerate()
        .map(|(i, df)| {
            let (description, evidence) = match &df.verification {
                VerificationResult::Drifted { expected, actual } => (
                    format!("Context drift detected: {}", df.claim.original_text),
                    format!("Expected: {}\nActual: {}", expected, actual),
                ),
                VerificationResult::Unknown { reason } => (
                    format!("Unverifiable context claim: {}", df.claim.original_text),
                    format!("Reason: {}", reason),
                ),
                _ => unreachable!(), // Matched findings are filtered out before this
            };

            let file_line = df.claim.source_file.line_number;
            let file_path = df.claim.source_file.file_path.clone();

            let mut tags = vec![
                String::from("context-drift"),
                df.claim.category.as_str().to_string(),
            ];
            tags.push(format!("file-type:{:?}", df.claim.source_file.file_type));

            CanonicalFinding {
                id: format!("CTX-{:04}", i + 1),
                scanner: ScannerType::ContextDrift,
                scanner_version: Some("0.1.0".to_string()),
                rule_id: format!("context-drift.{}", df.claim.category.as_str()),
                severity: df.severity,
                confidence: df.claim.extraction_confidence.clone(),
                title: format!(
                    "Context drift: {}",
                    df.claim.original_text.chars().take(80).collect::<String>()
                ),
                description,
                location: FindingLocation {
                    file: file_path,
                    line: file_line,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: Some(df.claim.original_text.clone()),
                },
                cwe: None,
                cvss: None,
                remediation: Some(format!(
                    "Update '{}' to reflect the current state of the codebase, \
                     or remove the out-of-date claim.",
                    df.claim.source_file.file_path.display()
                )),
                fix_effort: Some("1".to_string()),
                evidence: Some(evidence),
                tags,
                zt_pillars: vec![],
                cross_refs: vec![],
                grade: None,
                risk_score: None,
                reachable: None,
            }
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// ContextDriftScanner (Scanner trait implementation)
// ═══════════════════════════════════════════════════════════════════════════════

/// Context Drift Scanner — detects drift between agent context files and codebase reality
pub struct ContextDriftScanner {
    /// Root path to scan for context files and verify claims against
    root: PathBuf,
    /// Maximum number of findings to return
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

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
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

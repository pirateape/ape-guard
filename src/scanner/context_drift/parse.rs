// Context Drift Detection — Claim Parsing
// Extracts claims from AGENTS.md, CLAUDE.md, and .cursor/rules files.
use super::discover::{
    dep_pattern, detect_file_type, discover_context_files, read_file, technology_keyword_pattern,
};
use super::types::{ClaimCategory, ContextClaim, ContextFileRef, ContextFileType};
use crate::find::Confidence;
use std::path::Path;

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
pub(crate) fn classify_claim(text: &str, lower: &str, section: Option<&str>) -> ClaimCategory {
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

/// Parse an AGENTS.md file into extracted claims
pub(crate) fn parse_agents_md(
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

/// Parse a CLAUDE.md file into extracted claims
pub(crate) fn parse_claude_md(
    content: &str,
    source_path: &Path,
    file_type: ContextFileType,
) -> Vec<ContextClaim> {
    // CLAUDE.md has a similar structure to AGENTS.md but with more
    // structured known sections. Reuse the same parser — the section
    // context helps classify claims more accurately.
    parse_agents_md(content, source_path, file_type)
}

/// Parse a .cursor/rules file into extracted claims
pub(crate) fn parse_cursor_rules(
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

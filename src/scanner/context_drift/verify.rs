// Context Drift Detection — Verification Engine
// Verifies claims against the codebase, converting drift findings to CanonicalFindings.
use super::discover::{path_pattern, read_file, technology_keyword_pattern, version_pattern};
use super::types::{ClaimCategory, ContextClaim, DriftFinding, VerificationResult};
use crate::find::{CanonicalFinding, FindingLocation, ScannerType, Severity};
use regex::Regex;
use std::path::Path;

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
pub(crate) fn drift_severity(category: &ClaimCategory, result: &VerificationResult) -> Severity {
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
pub(crate) fn verify_single_claim(
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

// --- Dependency Claim Verification --------------------------------------------

/// Cache of parsed dependency files to avoid re-parsing
pub(crate) struct DependencyCache {
    cargo_toml: Option<String>,
    package_json: Option<String>,
    pyproject_toml: Option<String>,
    go_mod: Option<String>,
}

impl DependencyCache {
    pub(crate) fn new(root: &Path) -> Self {
        Self {
            cargo_toml: read_file(&root.join("Cargo.toml")),
            package_json: read_file(&root.join("package.json")),
            pyproject_toml: read_file(&root.join("pyproject.toml")),
            go_mod: read_file(&root.join("go.mod")),
        }
    }

    pub(crate) fn has_dependency(
        &self,
        dep_name: &str,
        version_hint: Option<&str>,
    ) -> Option<String> {
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
pub(crate) fn extract_json_section(json: &str, key: &str) -> Option<String> {
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
pub(crate) fn extract_dep_name(text: &str) -> Option<(String, Option<String>)> {
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

pub(crate) fn verify_dependency_claim(
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

// --- Path Claim Verification --------------------------------------------------

pub(crate) fn verify_path_claim(claim: &ContextClaim, root: &Path) -> VerificationResult {
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

// --- Architecture Claim Verification ------------------------------------------

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
pub(crate) fn search_for_technology_usage(tech: &str, root: &Path) -> Option<String> {
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

// --- Convention Claim Verification --------------------------------------------

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

// --- Security Claim Verification ----------------------------------------------

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

// --- Command Claim Verification -----------------------------------------------

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

// --- Semantic Claim Verification ----------------------------------------------

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

// --- Drift Finding → CanonicalFinding Conversion ------------------------------

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

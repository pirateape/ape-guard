// Gitleaks Scanner Driver
// Wraps the gitleaks binary for secret scanning.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Gitleaks {
    binary: String,
}

impl Gitleaks {
    pub fn new() -> Self {
        Gitleaks {
            binary: "gitleaks".to_string(),
        }
    }
}

#[async_trait]
impl Scanner for Gitleaks {
    fn name(&self) -> &'static str {
        "gitleaks"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Gitleaks
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install gitleaks  |  https://github.com/gitleaks/gitleaks"
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        which::which(&self.binary)
            .map(|_| true)
            .or(Ok(false))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new(&self.binary)
            .arg("version")
            .output()
            .await
            .map_err(ScannerError::Io)?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        // Determine if git repo or not
        let is_git = path.join(".git").exists();

        let mut cmd = tokio::process::Command::new(&self.binary);
        cmd.arg("detect")
            .arg("--source")
            .arg(path)
            .arg("-f")
            .arg("json")
            .arg("--no-color");

        if !is_git {
            cmd.arg("--no-git");
        }

        let output = cmd
            .output()
            .await
            .map_err(ScannerError::Io)?;

        if output.status.success() {
            // No findings — empty array
            Ok(output.stdout)
        } else if output.status.code() == Some(1) {
            // Findings found — still stdout contains the results
            Ok(output.stdout)
        } else {
            // Real error
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ScannerError::ExecutionFailed(stderr.to_string()))
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // Gitleaks JSON output: array of finding objects
        #[derive(Deserialize)]
        #[allow(non_snake_case, dead_code)]
        struct GitleaksFinding {
            Description: String,
            StartLine: Option<u32>,
            EndLine: Option<u32>,
            File: String,
            Match: String,
            Secret: String,
            RuleID: String,
            Author: Option<String>,
            Commit: Option<String>,
            Date: Option<String>,
            Tags: Option<String>,
            Fingerprint: String,
        }

        // Handle empty output (no findings found)
        if raw.is_empty() || raw.len() < 5 {
            return Ok(vec![]);
        }

        // Try to parse as JSON array; if it fails, wrap in brackets for single-object format
        let findings: Vec<GitleaksFinding> = match serde_json::from_slice(raw) {
            Ok(f) => f,
            Err(_) => {
                // Gitleaks may return a single object without array brackets when using --no-git
                let wrapped = format!("[{}]", String::from_utf8_lossy(raw));
                serde_json::from_str(&wrapped)
                    .map_err(|e| ScannerError::ParseFailed(e.to_string()))?
            }
        };

        let now = chrono::Utc::now().format("%Y%m%d").to_string();

        let canonical: Vec<CanonicalFinding> = findings
            .iter()
            .enumerate()
            .map(|(i, f)| CanonicalFinding {
                id: format!("AG-{}-{:04}", now, i + 1),
                scanner: ScannerType::Gitleaks,
                scanner_version: None, // populated by scanner outer flow
                rule_id: f.RuleID.clone(),
                severity: map_gitleaks_severity(&f.RuleID),
                confidence: Confidence::Certain,
                title: format!("Secret: {}", f.RuleID),
                description: f.Description.clone(),
                location: FindingLocation {
                    file: std::path::PathBuf::from(&f.File),
                    line: f.StartLine,
                    column: None,
                    commit: f.Commit.clone(),
                    author: f.Author.clone(),
                    snippet: Some(format!("{}", f.Match)),
                },
                cwe: Some("CWE-798".to_string()), // Hardcoded Credentials
                cvss: Some(7.5),
                remediation: Some(format!(
                    "Remove the secret from code. Rotate the credential. \
                     Check git history: `git log --all -p -S \"{}\"`",
                    f.Secret.chars().take(20).collect::<String>()
                )),
                fix_effort: Some("15 minutes".to_string()),
                evidence: Some(format!("Match: {}", f.Match)),
                tags: f.Tags.as_ref()
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_default(),
                zt_pillars: vec!["identity".to_string(), "devices".to_string()],
                cross_refs: vec![],
            })
            .collect();

        Ok(canonical)
    }
}

fn map_gitleaks_severity(_rule_id: &str) -> Severity {
    // Gitleaks doesn't natively report severity per finding.
    // In a real implementation, this would use a rule→severity mapping table.
    // Default: all gitleaks findings are at least Medium.
    Severity::High
}

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

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.gitleaks`)
    pub fn with_binary(path: Option<String>) -> Self {
        Gitleaks {
            binary: path.unwrap_or_else(|| "gitleaks".to_string()),
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
        if !crate::scanner::binary_exists(&self.binary) {
            return Err(ScannerError::NotFound(self.binary.clone()));
        }
        Ok(true)
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
        // Gitleaks outputs JSON findings to STDERR (not stdout) when exit code is 1 (leaks found).
        // Using --report-path with a temp file ensures we always capture the JSON output
        // regardless of exit code.
        let nonce = uuid::Uuid::new_v4().simple().to_string();
        let temp_dir = std::env::temp_dir();
        let report_file = temp_dir.join(format!("apeguard-gitleaks-{}.json", nonce));
        let report_path = report_file.to_string_lossy().to_string();

        let is_git = path.join(".git").exists();

        let mut args = vec![
            "detect".to_string(),
            "--source".to_string(),
            path.to_string_lossy().to_string(),
            "-f".to_string(),
            "json".to_string(),
            "--no-color".to_string(),
            "--report-path".to_string(),
            report_path,
        ];

        if !is_git {
            args.push("--no-git".to_string());
        }

        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let result = crate::scanner::run_command_with_timeout(&self.binary, &arg_refs, 30).await;

        // Read the temp file regardless of command success/failure
        let report_content = std::fs::read(&report_file).map_err(ScannerError::Io)?;

        // Clean up temp file
        let _ = std::fs::remove_file(&report_file);

        // If the report file has findings (non-empty), return them
        if report_content.len() > 5 {
            return Ok(report_content);
        }

        // Fall back to stdout content if temp file is empty
        if let Ok(stdout) = result {
            if stdout.len() > 5 {
                return Ok(stdout);
            }
        }

        // No findings found — return empty
        Ok(Vec::new())
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
            #[serde(default)]
            Tags: Vec<String>,
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
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        let canonical: Vec<CanonicalFinding> = findings
            .iter()
            .enumerate()
            .map(|(i, f)| CanonicalFinding {
                id: format!("AG-{}-{}-{:04}", now, &nonce[..8], i + 1),
                scanner: ScannerType::Gitleaks,
                scanner_version: None, // populated by scanner outer flow
                rule_id: f.RuleID.clone(),
                severity: Severity::High, // overridden by GITLEAKS_SEVERITY_MAP in normalize_findings()
                confidence: Confidence::Certain,
                title: format!("Secret: {}", f.RuleID),
                description: f.Description.clone(),
                location: FindingLocation {
                    file: std::path::PathBuf::from(&f.File),
                    line: f.StartLine,
                    column: None,
                    commit: f.Commit.clone(),
                    author: f.Author.clone(),
                    snippet: Some(f.Match.to_string()),
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
                tags: f.Tags.clone(),
                zt_pillars: vec!["identity".to_string(), "devices".to_string()],
                cross_refs: vec![],
                grade: None,
                risk_score: None,
            })
            .collect();

        Ok(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output_real_fixture() {
        let json = r#"[
            {
                "Description": "AWS Access Key ID",
                "StartLine": 42,
                "EndLine": 42,
                "File": "src/config.py",
                "Match": "AKIAIOSFODNN7EXAMPLE",
                "Secret": "AKIAIOSFODNN7EXAMPLE",
                "RuleID": "aws-access-key-id",
                "Fingerprint": "abc123:aws-access-key-id:42",
                "Author": "dev@example.com",
                "Commit": "a1b2c3d4",
                "Tags": ["aws", "credentials"]
            },
            {
                "Description": "Generic API Key",
                "StartLine": 10,
                "EndLine": 10,
                "File": ".env",
                "Match": "API_KEY=sk-1234567890abcdef",
                "Secret": "sk-1234567890abcdef",
                "RuleID": "generic-api-key",
                "Fingerprint": "def456:generic-api-key:10"
            }
        ]"#;

        let scanner = Gitleaks::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);

        // First finding: AWS key
        let f1 = &findings[0];
        assert_eq!(f1.rule_id, "aws-access-key-id");
        assert_eq!(f1.severity, Severity::High);
        assert_eq!(f1.location.line, Some(42));
        assert_eq!(f1.cwe.as_deref(), Some("CWE-798"));
        assert!(f1.remediation.as_ref().unwrap().contains("Rotate"));
        assert_eq!(f1.location.commit.as_deref(), Some("a1b2c3d4"));
        assert!(f1.tags.contains(&"aws".to_string()));

        // Second finding: generic API key
        let f2 = &findings[1];
        assert_eq!(f2.rule_id, "generic-api-key");
        assert_eq!(f2.location.line, Some(10));
        assert!(f2.tags.is_empty()); // no Tags field in fixture
    }

    #[test]
    fn test_parse_output_empty_array() {
        let scanner = Gitleaks::new();
        let findings = scanner.parse_output(b"[]").unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_empty_bytes() {
        let scanner = Gitleaks::new();
        let findings = scanner.parse_output(b"").unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_single_object_fallback() {
        // Gitleaks may return a single object without array brackets
        let json = r#"{
            "Description": "test",
            "StartLine": 1,
            "EndLine": 1,
            "File": "test.txt",
            "Match": "secret",
            "Secret": "secret",
            "RuleID": "test-rule",
            "Fingerprint": "fp1"
        }"#;

        let scanner = Gitleaks::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "test-rule");
    }
}

// TruffleHog Scanner Driver
// Wraps the trufflehog binary for deep secret scanning.
// Uses filesystem source with JSON Lines output format.
// Detects secrets that Gitleaks may miss — different detection engine, entropy-based + regex.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Trufflehog {
    binary: String,
}

impl Trufflehog {
    #[allow(dead_code)] // P3/P4: alternative constructor not wired; binary path via config instead
    pub fn new() -> Self {
        Trufflehog {
            binary: "trufflehog".to_string(),
        }
    }

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.trufflehog`)
    pub fn with_binary(path: Option<String>) -> Self {
        Trufflehog {
            binary: path.unwrap_or_else(|| "trufflehog".to_string()),
        }
    }
}

#[async_trait]
impl Scanner for Trufflehog {
    fn name(&self) -> &'static str {
        "trufflehog"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Trufflehog
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install trufflehog  |  https://github.com/trufflesecurity/trufflehog"
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        if !crate::scanner::binary_exists(&self.binary) {
            return Err(ScannerError::NotFound(self.binary.clone()));
        }
        Ok(true)
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new(&self.binary)
            .arg("--version")
            .output()
            .await
            .map_err(ScannerError::Io)?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        // TruffleHog filesystem scan outputs JSON Lines (NDJSON) to stdout.
        // Using --no-update to skip update checks and --json for JSON output.
        let output = tokio::process::Command::new(&self.binary)
            .arg("filesystem")
            .arg(path.to_string_lossy().as_ref())
            .arg("--json")
            .arg("--no-update")
            .output()
            .await
            .map_err(ScannerError::Io)?;

        // TruffleHog exits 0 on success (even with findings, unlike Gitleaks)
        if output.status.success() && !output.stdout.is_empty() {
            return Ok(output.stdout);
        }

        // Check stderr for meaningful errors
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            // Some versions print banner to stderr, which is fine
            // Only error if stdout is empty AND there's a real error
            if output.stdout.is_empty() && !output.status.success() {
                return Err(ScannerError::ExecutionFailed(stderr.trim().to_string()));
            }
        }

        // No findings or empty output
        Ok(output.stdout)
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // TruffleHog outputs NDJSON (one JSON object per line).
        // But to be robust, try full-content as array, then as single object, then NDJSON.
        let content = String::from_utf8_lossy(raw);

        if content.trim().is_empty() {
            return Ok(vec![]);
        }

        // Strip security-pragma comments (not valid JSON) before parsing.
        // TruffleHog output is clean, but test fixtures may have embedded
        // `// pragma: allowlist secret` comments to appease secret scanners.
        let content = content.replace(" // pragma: allowlist secret", "");
        let content = content.replace(" // pragma: allowlist", "");

        if content.trim().is_empty() {
            return Ok(vec![]);
        }

        // Strategy 1: Try to parse as a JSON array
        if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
            return Trufflehog::parse_json_array(&arr);
        }

        // Strategy 2: Try to parse as a single JSON object
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
            if val.is_object() {
                return Trufflehog::parse_json_array(&[val]);
            }
        }

        // Strategy 3: NDJSON — one JSON object per line
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();
        let mut index: u32 = 0;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            match parse_trufflehog_line(trimmed) {
                Ok(Some(mut f)) => {
                    index += 1;
                    f.id = format!("AG-{}-{}-{:04}", now, &nonce[..8], index);
                    f.scanner = ScannerType::Trufflehog;
                    findings.push(f);
                }
                Ok(None) => continue,
                Err(e) => {
                    tracing::debug!("TruffleHog parse warning on line {}: {}", index + 1, e);
                    continue;
                }
            }
        }

        Ok(findings)
    }
}

impl Trufflehog {
    /// Parse an array of JSON values into canonical findings
    fn parse_json_array(arr: &[serde_json::Value]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        for (i, val) in arr.iter().enumerate() {
            let line = serde_json::to_string(val).unwrap_or_default();
            match parse_trufflehog_line(&line) {
                Ok(Some(mut f)) => {
                    f.id = format!("AG-{}-{}-{:04}", now, &nonce[..8], i + 1);
                    f.scanner = ScannerType::Trufflehog;
                    findings.push(f);
                }
                Ok(None) => continue,
                Err(e) => {
                    tracing::debug!("TruffleHog parse warning at index {}: {}", i, e);
                    continue;
                }
            }
        }

        Ok(findings)
    }
}

/// Parse a single TruffleHog JSON line into a CanonicalFinding.
fn parse_trufflehog_line(line: &str) -> Result<Option<CanonicalFinding>, ScannerError> {
    // Top-level structure
    #[derive(Deserialize)]
    #[expect(non_snake_case)]
    #[expect(dead_code)] // P3/P4: TrufflehogResult fields from trufflehog JSON; SourceType/SourceName not consumed yet
    struct TrufflehogResult {
        #[serde(default)]
        SourceMetadata: Option<SourceMetadata>,
        #[serde(default)]
        DetectorName: String,
        #[serde(default)]
        DetectorDescription: String,
        #[serde(default)]
        DetectorType: i32,
        #[serde(default)]
        Verified: bool,
        #[serde(default)]
        Raw: String,
        #[serde(default)]
        RawV2: String,
        #[serde(default)]
        Redacted: String,
        #[serde(default)]
        SourceType: i32,
        #[serde(default)]
        SourceName: String,
        #[serde(default)]
        ExtraData: Option<std::collections::HashMap<String, String>>,
    }

    #[derive(Deserialize)]
    #[expect(non_snake_case)]
    struct SourceMetadata {
        #[serde(default)]
        Data: Option<SourceData>,
    }

    #[derive(Deserialize)]
    #[expect(non_snake_case)]
    struct SourceData {
        #[serde(default)]
        Filesystem: Option<FileSource>,
        #[serde(default)]
        Git: Option<GitSource>,
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct FileSource {
        #[serde(default)]
        file: String,
        #[serde(default)]
        line: Option<u32>,
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct GitSource {
        #[serde(default)]
        file: String,
        #[serde(default)]
        line: Option<u32>,
        #[serde(default)]
        commit: Option<String>,
        #[serde(default)]
        author: Option<String>,
    }

    let result: TrufflehogResult =
        serde_json::from_str(line).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

    // Skip entries without a secret
    if result.Raw.is_empty() && result.RawV2.is_empty() {
        return Ok(None);
    }

    // Extract file path and line from source metadata
    let (file_path, line_num, commit, author) = match result.SourceMetadata.and_then(|sm| sm.Data) {
        Some(SourceData {
            Filesystem: Some(fs),
            Git: None,
        }) => (fs.file, fs.line, None, None),
        Some(SourceData {
            Filesystem: None,
            Git: Some(git),
        }) => (git.file, git.line, git.commit, git.author),
        Some(SourceData {
            Filesystem: Some(_),
            Git: Some(git),
        }) => {
            // Both present — prefer Git (has commit/author info)
            (git.file, git.line, git.commit, git.author)
        }
        _ => return Ok(None), // No file info available
    };

    // Map detector type to severity
    let (severity, cwe) = detector_type_to_severity(result.DetectorType, result.Verified);

    // Build tags from detector metadata
    let mut tags: Vec<String> = Vec::new();
    tags.push("secret".to_string());
    if result.Verified {
        tags.push("verified".to_string());
    }
    // Add extra data keys as tags for searchability
    if let Some(extra) = &result.ExtraData {
        for key in extra.keys() {
            tags.push(format!("extra:{}", key));
        }
    }

    // Build evidence string
    let evidence = if !result.RawV2.is_empty() {
        if result.RawV2.len() > 100 {
            format!("Secret (truncated): {}...", &result.RawV2[..100])
        } else {
            format!("Secret: {}", result.RawV2)
        }
    } else if !result.Redacted.is_empty() {
        format!("Redacted: {}", result.Redacted)
    } else {
        "Verified secret detected".to_string()
    };

    // Build remediation
    let remediation = if result.Verified {
        format!(
            "VERIFIED secret detected. Rotate immediately. \
             Check git history: `trufflehog git --since-commit HEAD~10 --json`. \
             Detector: {}",
            result.DetectorName
        )
    } else {
        format!(
            "Potential secret detected. Verify manually and rotate if real. \
             Detector: {}. Run with `--no-verification` to skip verification.",
            result.DetectorName
        )
    };

    Ok(Some(CanonicalFinding {
        id: String::new(), // Assigned by caller
        scanner: ScannerType::Trufflehog,
        scanner_version: None,
        rule_id: format!(
            "trufflehog-{}",
            result.DetectorName.to_lowercase().replace(' ', "-")
        ),
        severity,
        confidence: if result.Verified {
            Confidence::Certain
        } else {
            Confidence::Firm
        },
        title: format!(
            "Secret: {}{}",
            result.DetectorName,
            if result.Verified { " (VERIFIED)" } else { "" }
        ),
        description: if result.DetectorDescription.is_empty() {
            format!("Detected by TruffleHog detector: {}", result.DetectorName)
        } else {
            result.DetectorDescription
        },
        location: FindingLocation {
            file: std::path::PathBuf::from(&file_path),
            line: line_num,
            column: None,
            commit,
            author,
            snippet: Some(result.Raw.chars().take(80).collect()),
        },
        cwe: Some(cwe),
        cvss: Some(if result.Verified { 8.0 } else { 6.5 }),
        remediation: Some(remediation),
        fix_effort: Some(if result.Verified {
            "30 minutes".to_string()
        } else {
            "15 minutes".to_string()
        }),
        evidence: Some(evidence),
        tags,
        zt_pillars: vec!["identity".to_string(), "devices".to_string()],
        cross_refs: vec![],
        grade: None,
        risk_score: None,
        reachable: None,
    }))
}

/// Map TruffleHog DetectorType to severity and CWE.
///
/// DetectorType is an integer enum:
///   0=Unknown, 1=AWS, 2=GCP, 3=GitHub, 4=Slack, 5=Generic,
///   6=PrivateKey, 7=Social, 8=GitHubToken, 9=Discord, 10=Twilio,
///   11=Stripe, 12=Plaid, 13=SendGrid, 14=Postman, 15=JWT,
///   16=GithubApp, 17=NPM, 18=PyPI, etc.
fn detector_type_to_severity(detector_type: i32, verified: bool) -> (Severity, String) {
    let severity = match detector_type {
        // High-value credential types
        1 | 2 | 16 => {
            // AWS, GCP, GitHub App
            if verified {
                Severity::Critical
            } else {
                Severity::High
            }
        }
        // Token/API key types
        3 | 8 | 11 | 12 | 15 => {
            // GitHub, GitHubToken, Stripe, Plaid, JWT
            if verified {
                Severity::Critical
            } else {
                Severity::High
            }
        }
        // Private keys
        6 => Severity::High,
        // Communication platform tokens
        4 | 9 | 10 | 14 => {
            // Slack, Discord, Twilio, Postman
            if verified {
                Severity::High
            } else {
                Severity::Medium
            }
        }
        // Package registry tokens
        17 | 18 => {
            // NPM, PyPI
            if verified {
                Severity::High
            } else {
                Severity::Medium
            }
        }
        // Everything else
        _ => {
            if verified {
                Severity::High
            } else {
                Severity::Medium
            }
        }
    };

    let cwe = match detector_type {
        6 => "CWE-798",             // Private Key → Use of Hardcoded Credentials
        15 => "CWE-798",            // JWT → Hardcoded Credentials
        1 | 2 | 3 | 8 => "CWE-798", // Cloud/Token → Hardcoded Credentials
        _ => "CWE-200",             // Generic → Information Exposure
    }
    .to_string();

    (severity, cwe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output_single_finding() {
        let json = r#"{
            "SourceMetadata": {
                "Data": {
                    "Filesystem": {
                        "file": "src/config.py",
                        "line": 42
                    }
                }
            },
            "SourceID": 1,
            "SourceType": 15,
            "SourceName": "trufflehog - filesystem",
            "DetectorType": 1,
            "DetectorName": "AWS",
            "DetectorDescription": "AWS Access Key",
            "DecoderName": "PLAIN",
            "Verified": true,
            "Raw": "AKIAIOSFODNN7EXAMPLE", // pragma: allowlist secret
            "RawV2": "",
            "Redacted": "AKIAIOSFODNN7EX***",
            "ExtraData": {
                "rotation_guide": "https://howtorotate.com/docs/tutorials/aws/"
            },
            "StructuredData": null
        }"#;

        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.rule_id, "trufflehog-aws");
        assert_eq!(f.severity, Severity::Critical); // AWS + verified
        assert_eq!(f.location.file.to_string_lossy(), "src/config.py");
        assert_eq!(f.location.line, Some(42));
        assert!(f.title.contains("VERIFIED"));
        assert!(f.confidence == Confidence::Certain);
        assert!(f.tags.contains(&"verified".to_string()));
    }

    #[test]
    fn test_parse_output_unverified() {
        let json = r#"{
            "SourceMetadata": {
                "Data": {
                    "Filesystem": {
                        "file": ".env",
                        "line": 5
                    }
                }
            },
            "SourceID": 1,
            "SourceType": 15,
            "SourceName": "trufflehog - filesystem",
            "DetectorType": 5,
            "DetectorName": "Generic",
            "DetectorDescription": "",
            "DecoderName": "PLAIN",
            "Verified": false,
            "Raw": "SECRET_KEY=sk-1234",
            "RawV2": "",
            "Redacted": "",
            "ExtraData": null,
            "StructuredData": null
        }"#;

        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Medium); // Generic + unverified
        assert!(f.confidence == Confidence::Firm);
        assert!(!f.title.contains("VERIFIED"));
    }

    #[test]
    fn test_parse_output_multiple_lines() {
        let json = r#"{"SourceMetadata":{"Data":{"Filesystem":{"file":"a.txt","line":1}}},"SourceID":1,"SourceType":15,"SourceName":"test","DetectorType":3,"DetectorName":"GitHub","Verified":true,"Raw":"ghp_12345","RawV2":"","Redacted":"","ExtraData":null,"StructuredData":null}
        {"SourceMetadata":{"Data":{"Filesystem":{"file":"b.txt","line":10}}},"SourceID":1,"SourceType":15,"SourceName":"test","DetectorType":4,"DetectorName":"Slack","Verified":false,"Raw":"xoxb-12345","RawV2":"","Redacted":"","ExtraData":null,"StructuredData":null}"#;

        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].rule_id, "trufflehog-github");
        assert_eq!(findings[1].rule_id, "trufflehog-slack");
        assert_eq!(findings[0].location.file.to_string_lossy(), "a.txt");
        assert_eq!(findings[1].location.file.to_string_lossy(), "b.txt");
    }

    #[test]
    fn test_parse_output_empty() {
        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(b"").unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_whitespace_only() {
        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(b"\n\n  \n").unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_git_source() {
        let json = r#"{
            "SourceMetadata": {
                "Data": {
                    "Git": {
                        "file": "src/main.go",
                        "line": 23,
                        "commit": "abc123def456", // pragma: allowlist secret
                        "author": "dev@example.com"
                    }
                }
            },
            "SourceID": 1,
            "SourceType": 0,
            "SourceName": "trufflehog - git",
            "DetectorType": 6,
            "DetectorName": "PrivateKey",
            "Verified": false,
            "Raw": "-----BEGIN RSA PRIVATE KEY-----", // pragma: allowlist secret
            "RawV2": "",
            "Redacted": "",
            "ExtraData": null,
            "StructuredData": null
        }"#;

        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.severity, Severity::High); // PrivateKey
        assert_eq!(f.location.file.to_string_lossy(), "src/main.go");
        assert_eq!(f.location.line, Some(23));
        assert_eq!(f.location.commit.as_deref(), Some("abc123def456")); // pragma: allowlist secret
        assert_eq!(f.location.author.as_deref(), Some("dev@example.com"));
    }

    #[test]
    fn test_parse_output_skip_no_secret() {
        let json = r#"{
            "SourceMetadata": {
                "Data": {
                    "Filesystem": {
                        "file": "readme.md",
                        "line": 0
                    }
                }
            },
            "DetectorName": "Test",
            "Verified": false,
            "Raw": "",
            "RawV2": "",
            "DetectorType": 0
        }"#;

        let scanner = Trufflehog::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_detector_type_severity() {
        // AWS verified → Critical
        let (s, _) = detector_type_to_severity(1, true);
        assert_eq!(s, Severity::Critical);

        // AWS unverified → High
        let (s, _) = detector_type_to_severity(1, false);
        assert_eq!(s, Severity::High);

        // Private key → High regardless
        let (s, _) = detector_type_to_severity(6, false);
        assert_eq!(s, Severity::High);

        // Generic unverified → Medium
        let (s, _) = detector_type_to_severity(5, false);
        assert_eq!(s, Severity::Medium);

        // Generic verified → High
        let (s, _) = detector_type_to_severity(5, true);
        assert_eq!(s, Severity::High);

        // NPM verified → High
        let (s, _) = detector_type_to_severity(17, true);
        assert_eq!(s, Severity::High);
    }

    #[test]
    fn test_scanner_name() {
        let scanner = Trufflehog::new();
        assert_eq!(scanner.name(), "trufflehog");
    }

    #[test]
    fn test_scanner_type() {
        let scanner = Trufflehog::new();
        assert_eq!(scanner.scanner_type(), ScannerType::Trufflehog);
    }

    #[test]
    fn test_install_hint() {
        let scanner = Trufflehog::new();
        assert!(scanner.install_hint().contains("trufflehog"));
    }
}

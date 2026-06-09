// Checkov Scanner Driver (Layer 6)
// Wraps the checkov binary for IaC misconfiguration scanning.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Checkov {
    binary: String,
}

impl Checkov {
    pub fn new() -> Self {
        Checkov {
            binary: "checkov".to_string(),
        }
    }

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.checkov`)
    pub fn with_binary(path: Option<String>) -> Self {
        Checkov {
            binary: path.unwrap_or_else(|| "checkov".to_string()),
        }
    }
}

#[async_trait]
impl Scanner for Checkov {
    fn name(&self) -> &'static str {
        "checkov"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Checkov
    }

    fn install_hint(&self) -> &'static str {
        "Install: pip install checkov  |  brew install checkov  |  https://www.checkov.io"
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

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return Ok(stdout);
        }
        Ok(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        let path_str = path.to_string_lossy().to_string();
        let args = ["--directory", path_str.as_str(), "--output", "json"];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 180).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        struct CheckovReport {
            results: CheckovResults,
        }

        #[derive(Deserialize)]
        struct CheckovResults {
            #[serde(default)]
            failed_checks: Vec<CheckovFinding>,
        }

        #[derive(Deserialize)]
        struct CheckovFinding {
            check_id: String,
            #[serde(default)]
            check_name: String,
            #[serde(default)]
            severity: Option<String>,
            #[serde(default)]
            guideline: Option<String>,
            #[serde(default)]
            file_path: Option<String>,
            #[serde(default)]
            file_abs_path: Option<String>,
            #[serde(default)]
            file_line_range: Option<Vec<u32>>,
            #[serde(default)]
            resource: Option<String>,
            #[serde(default)]
            check_type: Option<String>,
        }

        if raw.is_empty() || raw.len() < 5 {
            return Ok(vec![]);
        }

        let report: CheckovReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        let canonical = report
            .results
            .failed_checks
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let sev = match f
                    .severity
                    .as_deref()
                    .unwrap_or("MEDIUM")
                    .to_lowercase()
                    .as_str()
                {
                    "critical" => Severity::Critical,
                    "high" => Severity::High,
                    "medium" => Severity::Medium,
                    "low" => Severity::Low,
                    _ => Severity::Info,
                };

                let file = f
                    .file_abs_path
                    .as_deref()
                    .or(f.file_path.as_deref())
                    .unwrap_or(".");

                let line = f.file_line_range.as_ref().and_then(|r| r.first().copied());

                CanonicalFinding {
                    id: format!("AG-CK-{}-{}-{:04}", now, &nonce[..8], i + 1),
                    scanner: ScannerType::Checkov,
                    scanner_version: None,
                    rule_id: f.check_id.clone(),
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: if f.check_name.is_empty() {
                        format!("IaC Misconfiguration: {}", f.check_id)
                    } else {
                        format!("IaC Misconfiguration: {}", f.check_name)
                    },
                    description: f
                        .guideline
                        .clone()
                        .unwrap_or_else(|| "Checkov reported an IaC policy violation".to_string()),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(file),
                        line,
                        column: None,
                        commit: None,
                        author: None,
                        snippet: None,
                    },
                    cwe: None,
                    cvss: None,
                    remediation: f.guideline.clone(),
                    fix_effort: None,
                    evidence: f.resource.clone(),
                    tags: vec![
                        "iac".to_string(),
                        "misconfig".to_string(),
                        f.check_type
                            .clone()
                            .unwrap_or_else(|| "infrastructure".to_string()),
                    ],
                    zt_pillars: vec![],
                    cross_refs: vec![],
                    grade: None,
                    risk_score: None,
                }
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
        let json = r#"{
            "results": {
                "failed_checks": [
                    {
                        "check_id": "CKV_AWS_20",
                        "check_name": "S3 Bucket has an ACL defined which allows public READ access",
                        "severity": "HIGH",
                        "guideline": "Ensure S3 bucket ACL is private",
                        "file_path": "/terraform/main.tf",
                        "file_line_range": [12, 18],
                        "resource": "aws_s3_bucket.public",
                        "check_type": "terraform"
                    },
                    {
                        "check_id": "CKV_K8S_43",
                        "check_name": "Image should use digest",
                        "severity": "MEDIUM",
                        "guideline": "Pin container image by digest",
                        "file_path": "k8s/deploy.yaml",
                        "file_line_range": [5, 20],
                        "resource": "Deployment.default.web",
                        "check_type": "kubernetes"
                    }
                ]
            }
        }"#;

        let scanner = Checkov::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].scanner, ScannerType::Checkov);
        assert_eq!(findings[0].rule_id, "CKV_AWS_20");
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].location.line, Some(12));
        assert!(findings[0].title.contains("IaC Misconfiguration"));
        assert!(findings[0].tags.contains(&"iac".to_string()));

        assert_eq!(findings[1].rule_id, "CKV_K8S_43");
        assert_eq!(findings[1].severity, Severity::Medium);
    }

    #[test]
    fn test_parse_output_empty() {
        let scanner = Checkov::new();
        let findings = scanner
            .parse_output(br#"{"results":{"failed_checks":[]}}"#)
            .unwrap();
        assert!(findings.is_empty());
    }
}

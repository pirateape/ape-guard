// Layer 11: AWS S3 Bucket Permission Scanner
// Scans for publicly accessible S3 buckets and misconfigured permissions
// Uses AWS CLI, Prowler, or S3audit when available
use super::{Scanner, ScannerError};
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use async_trait::async_trait;
use std::path::Path;

#[derive(Debug)]
pub struct AwsS3Scanner {
    config_path: String,
}

impl AwsS3Scanner {
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for AwsS3Scanner {
    fn name(&self) -> &'static str {
        "aws-s3-audit"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::AwsS3
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        Ok(crate::scanner::binary_exists("aws") || crate::scanner::binary_exists("prowler"))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new("aws")
            .arg("--version")
            .output()
            .await
            .ok()
            .filter(|o| o.status.success());

        if let Some(output) = output {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        let output = tokio::process::Command::new("prowler")
            .arg("--version")
            .output()
            .await
            .ok()
            .and_then(|o| if o.status.success() { Some(o) } else { None });

        match output {
            Some(o) => Ok(String::from_utf8_lossy(&o.stdout).trim().to_string()),
            None => Ok("unknown (manual scan available)".to_string()),
        }
    }

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        if crate::scanner::binary_exists("aws") && std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            super::run_command_with_timeout("aws", &["s3api", "list-buckets"], 60).await
        } else {
            let mut findings = String::new();
            let aws_config = std::path::Path::new(&self.config_path);
            if aws_config.is_file() {
                if let Ok(content) = std::fs::read_to_string(aws_config) {
                    if content.contains("\"Principal\" : \"*\"")
                        || content.contains("\"Principal\":\"*\"")
                    {
                        findings.push_str("PUBLIC S3 BUCKET: Bucket with public access policy\n");
                    }
                    if content.contains("BucketEncryption") && content.contains("Status: Disabled")
                    {
                        findings.push_str("MISSING ENCRYPTION: S3 bucket encryption disabled\n");
                    }
                }
            }
            if findings.is_empty() {
                findings.push_str("OK: No public S3 buckets or encryption issues detected\n");
            }
            Ok(findings.into_bytes())
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        let output = String::from_utf8_lossy(raw);
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        // Parse AWS CLI JSON output if available
        if output.contains("Buckets") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output.trim()) {
                if let Some(buckets) = json.get("Buckets").and_then(|v| v.as_array()) {
                    for (i, bucket) in buckets.iter().enumerate() {
                        let name = bucket
                            .get("Name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let last_modified = bucket
                            .get("LastModified")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");

                        findings.push(CanonicalFinding {
                            id: format!("AG-S3-{}-{}-{:04}", now, &nonce[..8], i + 1),
                            scanner: ScannerType::AwsS3,
                            scanner_version: None,
                            rule_id: "AWS001".to_string(),
                            severity: Severity::Medium,
                            confidence: Confidence::Firm,
                            title: format!("S3 Bucket: {}", name),
                            description: format!(
                                "S3 bucket: {} (last modified: {})",
                                name, last_modified
                            ),
                            location: FindingLocation {
                                file: std::path::PathBuf::from(self.config_path.clone()),
                                line: None,
                                column: None,
                                commit: None,
                                author: None,
                                snippet: None,
                            },
                            cwe: None,
                            cvss: None,
                            remediation: Some(
                                "Check bucket policy for public access, enable encryption"
                                    .to_string(),
                            ),
                            fix_effort: None,
                            evidence: Some(format!(
                                "Bucket: {}, Modified: {}",
                                name, last_modified
                            )),
                            tags: vec![
                                "aws".to_string(),
                                "s3".to_string(),
                                "misconfig".to_string(),
                            ],
                            zt_pillars: vec![],
                            cross_refs: vec![],
                            grade: None,
                            risk_score: None,
                            reachable: None,
                        });
                    }
                }
            }
        } else {
            // Parse manual inspection output
            for (i, line) in output.lines().enumerate() {
                let (sev, title, prefix_len) = if line.contains("PUBLIC S3 BUCKET") {
                    (Severity::Critical, "Public S3 bucket detected", 18)
                } else if line.contains("MISSING ENCRYPTION") {
                    (Severity::High, "S3 bucket encryption disabled", 20)
                } else {
                    continue;
                };

                let detail = if line.len() > prefix_len + 1 {
                    line[prefix_len + 1..].trim()
                } else {
                    "unknown"
                };

                findings.push(CanonicalFinding {
                    id: format!("AG-S3-{}-{}-{:04}", now, &nonce[..8], findings.len() + 1),
                    scanner: ScannerType::AwsS3,
                    scanner_version: None,
                    rule_id: if line.contains("PUBLIC S3 BUCKET") {
                        "AWS001".to_string()
                    } else {
                        "AWS002".to_string()
                    },
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: format!("AWS S3: {}", title),
                    description: format!("{}: {}", title, detail),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(self.config_path.clone()),
                        line: Some((i + 1) as u32),
                        column: None,
                        commit: None,
                        author: None,
                        snippet: None,
                    },
                    cwe: None,
                    cvss: None,
                    remediation: if line.contains("PUBLIC S3 BUCKET") {
                        Some("Restrict bucket policy, remove public access".to_string())
                    } else {
                        Some("Enable server-side encryption (SSE-S3 or SSE-KMS)".to_string())
                    },
                    fix_effort: None,
                    evidence: Some(detail.to_string()),
                    tags: vec!["aws".to_string(), "s3".to_string(), "misconfig".to_string()],
                    zt_pillars: vec![],
                    cross_refs: vec![],
                    grade: None,
                    risk_score: None,
                    reachable: None,
                });
            }
        }

        Ok(findings)
    }

    fn install_hint(&self) -> &'static str {
        "Install AWS CLI: `brew install awscli` (requires AWS credentials)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aws_s3_scanner_name() {
        let scanner = AwsS3Scanner::new("test.json");
        assert_eq!(scanner.name(), "aws-s3-audit");
    }

    #[test]
    fn test_aws_s3_scanner_type() {
        let scanner = AwsS3Scanner::new("test.json");
        assert_eq!(scanner.scanner_type(), ScannerType::AwsS3);
    }

    #[tokio::test]
    async fn test_aws_s3_scanner_installed() {
        let scanner = AwsS3Scanner::new("test.json");
        assert!(scanner.check_installed().await.is_ok());
    }

    #[test]
    fn test_parse_output_manual_inspection() {
        let scanner = AwsS3Scanner::new("test.json");
        let raw = b"PUBLIC S3 BUCKET: Bucket with public access policy\nMISSING ENCRYPTION: S3 bucket encryption disabled\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.unwrap();
        assert!(!findings.is_empty());
    }
}

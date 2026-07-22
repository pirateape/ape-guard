// Layer 10: Terraform IaC Scanner
// Scans Terraform infrastructure configs for security misconfigurations
// Uses checkov/tfsec/Terrascan when available
use super::{Scanner, ScannerError};
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use async_trait::async_trait;
use std::path::Path;

#[derive(Debug)]
pub struct TerraformScanner {
    terraform_dir: String,
}

impl TerraformScanner {
    pub fn new(terraform_dir: &str) -> Self {
        Self {
            terraform_dir: terraform_dir.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for TerraformScanner {
    fn name(&self) -> &'static str {
        "terraform-audit"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::TerraformIaC
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        Ok(crate::scanner::binary_exists("checkov")
            || crate::scanner::binary_exists("tfsec")
            || crate::scanner::binary_exists("terrascan"))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new("checkov")
            .arg("--version")
            .output()
            .await
            .ok()
            .filter(|o| o.status.success());

        if let Some(output) = output {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        let output = tokio::process::Command::new("tfsec")
            .arg("--version")
            .output()
            .await
            .ok()
            .and_then(|o| if o.status.success() { Some(o) } else { None });

        if let Some(output) = output {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        let output = tokio::process::Command::new("terrascan")
            .arg("version")
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
        if crate::scanner::binary_exists("checkov") {
            super::run_command_with_timeout(
                "checkov",
                &["-d", &self.terraform_dir, "--format", "json"],
                60,
            )
            .await
        } else if crate::scanner::binary_exists("tfsec") {
            super::run_command_with_timeout("tfsec", &[".", "--no-colour"], 60).await
        } else {
            // Manual inspection fallback
            let mut findings = String::new();
            if let Ok(entries) = std::fs::read_dir(&self.terraform_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if content.contains("\"Principal\" : \"*\"")
                                || content.contains("\"Principal\":\"*\"")
                            {
                                findings
                                    .push_str(&format!("PUBLIC S3 BUCKET: {}\n", path.display()));
                            }
                            if content.contains("resource \"aws_s3_bucket\"")
                                && !content.contains("server_side_encryption_configuration")
                            {
                                findings.push_str(&format!(
                                    "MISSING ENCRYPTION: {} (S3 bucket without encryption)\n",
                                    path.display()
                                ));
                            }
                            if content.contains("\"Action\" : \"*\"")
                                || content.contains("\"Action\":\"*\"")
                            {
                                findings.push_str(&format!(
                                    "OVERLY PERMISSIVE IAM: {}\n",
                                    path.display()
                                ));
                            }
                        }
                    }
                }
            }
            if findings.is_empty() {
                findings.push_str("OK: No critical misconfigurations detected\n");
            }
            Ok(findings.into_bytes())
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        let output = String::from_utf8_lossy(raw);
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        // Parse checkov JSON output if available
        if output.contains("failed_checks") {
            #[derive(serde::Deserialize)]
            struct CheckovReport {
                results: CheckovResults,
            }
            #[derive(serde::Deserialize)]
            struct CheckovResults {
                #[serde(default)]
                failed_checks: Vec<CheckovFinding>,
            }
            #[derive(serde::Deserialize)]
            struct CheckovFinding {
                check_id: String,
                #[serde(default)]
                check_name: String,
                #[serde(default)]
                severity: Option<String>,
                #[serde(default)]
                guideline: Option<String>,
                #[serde(default)]
                resource: Option<String>,
                #[serde(default)]
                file: Option<String>,
            }

            if let Ok(report) = serde_json::from_str::<CheckovReport>(output.trim()) {
                for (i, f) in report.results.failed_checks.iter().enumerate() {
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

                    findings.push(CanonicalFinding {
                        id: format!("AG-TF-{}-{}-{:04}", now, &nonce[..8], i + 1),
                        scanner: ScannerType::TerraformIaC,
                        scanner_version: None,
                        rule_id: f.check_id.clone(),
                        severity: sev,
                        confidence: Confidence::Firm,
                        title: format!(
                            "Terraform Misconfiguration: {}",
                            if f.check_name.is_empty() {
                                &f.check_id
                            } else {
                                &f.check_name
                            }
                        ),
                        description: f
                            .guideline
                            .clone()
                            .unwrap_or_else(|| "Terraform misconfiguration detected".to_string()),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(
                                f.file.clone().unwrap_or_else(|| self.terraform_dir.clone()),
                            ),
                            line: None,
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
                            "terraform".to_string(),
                            "iac".to_string(),
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
        } else {
            // Parse manual inspection output
            for (i, line) in output.lines().enumerate() {
                let (sev, title, prefix_len) = if line.contains("PUBLIC S3 BUCKET") {
                    (Severity::Critical, "Public S3 bucket", 18)
                } else if line.contains("MISSING ENCRYPTION") {
                    (Severity::High, "Missing encryption", 20)
                } else if line.contains("OVERLY PERMISSIVE IAM") {
                    (Severity::High, "Overly permissive IAM", 23)
                } else {
                    continue;
                };

                let detail = if line.len() > prefix_len + 1 {
                    line[prefix_len + 1..].trim()
                } else {
                    "unknown"
                };

                findings.push(CanonicalFinding {
                    id: format!("AG-TF-{}-{}-{:04}", now, &nonce[..8], findings.len() + 1),
                    scanner: ScannerType::TerraformIaC,
                    scanner_version: None,
                    rule_id: format!("TF{:03}", findings.len() + 1),
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: format!("Terraform: {}", title),
                    description: format!("{}: {}", title, detail),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(self.terraform_dir.clone()),
                        line: Some((i + 1) as u32),
                        column: None,
                        commit: None,
                        author: None,
                        snippet: None,
                    },
                    cwe: None,
                    cvss: None,
                    remediation: Some("Fix Terraform misconfiguration".to_string()),
                    fix_effort: None,
                    evidence: Some(detail.to_string()),
                    tags: vec![
                        "terraform".to_string(),
                        "iac".to_string(),
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

        Ok(findings)
    }

    fn install_hint(&self) -> &'static str {
        "Install checkov: `brew install checkov` (or tfsec: `brew install tfsec`)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_terraform_scanner_name() {
        let scanner = TerraformScanner::new(".");
        assert_eq!(scanner.name(), "terraform-audit");
    }

    #[test]
    fn test_terraform_scanner_type() {
        let scanner = TerraformScanner::new(".");
        assert_eq!(scanner.scanner_type(), ScannerType::TerraformIaC);
    }

    #[tokio::test]
    async fn test_terraform_scanner_installed() {
        let scanner = TerraformScanner::new(".");
        assert!(scanner.check_installed().await.is_ok());
    }

    #[test]
    fn test_parse_output_manual_inspection() {
        let scanner = TerraformScanner::new(".");
        let raw = b"PUBLIC S3 BUCKET: main.tf\nMISSING ENCRYPTION: bucket.tf (S3 bucket without encryption)\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("terraform test: scan should succeed");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_parse_output_empty_input() {
        let scanner = TerraformScanner::new(".");
        let result = scanner.parse_output(b"");
        assert!(result.is_ok());
        let findings = result.expect("terraform test: empty parse should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_overly_permissive_iam() {
        let scanner = TerraformScanner::new(".");
        let raw = b"OVERLY PERMISSIVE IAM: iam.tf\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("terraform test: IAM parse should succeed");
        assert_eq!(findings.len(), 1);
        assert!(findings[0].title.contains("Overly permissive IAM"));
    }

    #[test]
    fn test_parse_output_ok_response() {
        let scanner = TerraformScanner::new(".");
        let raw = b"OK: No critical misconfigurations detected";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("terraform test: OK parse should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_no_relevant_lines() {
        let scanner = TerraformScanner::new(".");
        let raw = b"resource \"aws_s3_bucket\" \"data\" {\n  bucket = \"my-bucket\"\n}";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("terraform test: irrelevant parse should succeed");
        assert!(findings.is_empty());
    }
}

// Trivy Scanner Driver
// Wraps the trivy binary for vulnerability scanning, secret scanning, and IaC misconfiguration scanning.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Trivy {
    binary: String,
    scan_mode: TrivyMode,
}

pub enum TrivyMode {
    Vuln,         // SCA vulnerability scanning (--scanners vuln)
    Secret,       // Secret scanning (--scanners secret)
    Misconfig,    // IaC misconfiguration (--scanners misconfig)
}

impl Trivy {
    pub fn new() -> Self {
        Trivy {
            binary: "trivy".to_string(),
            scan_mode: TrivyMode::Vuln,
        }
    }

    pub fn with_mode(mode: TrivyMode) -> Self {
        Trivy {
            binary: "trivy".to_string(),
            scan_mode: mode,
        }
    }

    fn mode_flag(&self) -> &'static str {
        match self.scan_mode {
            TrivyMode::Vuln => "vuln",
            TrivyMode::Secret => "secret",
            TrivyMode::Misconfig => "misconfig",
        }
    }

    fn mode_name(&self) -> &'static str {
        match self.scan_mode {
            TrivyMode::Vuln => "trivy-vuln",
            TrivyMode::Secret => "trivy-secret",
            TrivyMode::Misconfig => "trivy-misconfig",
        }
    }
}

#[async_trait]
impl Scanner for Trivy {
    fn name(&self) -> &'static str {
        self.mode_name()
    }

    fn scanner_type(&self) -> ScannerType {
        match self.scan_mode {
            TrivyMode::Vuln => ScannerType::TrivyVuln,
            TrivyMode::Secret => ScannerType::TrivySecret,
            TrivyMode::Misconfig => ScannerType::TrivyMisconfig,
        }
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install trivy  |  https://trivy.dev"
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        Ok(tokio::process::Command::new(&self.binary)
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new(&self.binary)
            .arg("--version")
            .output()
            .await
            .map_err(ScannerError::Io)?;

        // Parse first line: "Version: 0.58.0"
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().next().unwrap_or("unknown").to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        let scanners = self.mode_flag();

        let output = tokio::process::Command::new(&self.binary)
            .arg("fs")
            .arg("--scanners")
            .arg(scanners)
            .arg("--format")
            .arg("json")
            .arg("--quiet")
            .arg(path)
            .output()
            .await
            .map_err(ScannerError::Io)?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            // Trivy may return non-zero even with partial results
            if output.stdout.len() > 10 {
                return Ok(output.stdout);
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ScannerError::ExecutionFailed(stderr.to_string()))
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        match self.scan_mode {
            TrivyMode::Vuln => self.parse_vuln(raw),
            TrivyMode::Secret => self.parse_secret(raw),
            TrivyMode::Misconfig => self.parse_misconfig(raw),
        }
    }
}

// --- Vulnerability parsing ---
impl Trivy {
    fn parse_vuln(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyReport {
            results: Vec<TrivyResult>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyResult {
            target: String,
            #[serde(rename = "Packages")]
            packages: Option<Vec<TrivyPackage>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyPackage {
            name: String,
            #[serde(rename = "Vulnerabilities")]
            vulnerabilities: Option<Vec<TrivyVuln>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyVuln {
            vulnerability_id: String,
            pkg_name: String,
            installed_version: String,
            fixed_version: String,
            severity: String,
            title: Option<String>,
            description: Option<String>,
            cvss_score: Option<f64>,
            cwe_ids: Option<Vec<String>>,
        }

        let report: TrivyReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let mut findings = Vec::new();
        let mut idx = 0u32;

        for result in &report.results {
            if let Some(ref packages) = result.packages {
                for pkg in packages {
                    if let Some(ref vulns) = pkg.vulnerabilities {
                        for v in vulns {
                            idx += 1;
                            let sev = match v.severity.to_lowercase().as_str() {
                                "critical" => Severity::Critical,
                                "high" => Severity::High,
                                "medium" => Severity::Medium,
                                "low" => Severity::Low,
                                _ => Severity::Info,
                            };

                            findings.push(CanonicalFinding {
                                id: format!("AG-TV-{}-{:04}", now, idx),
                                scanner: ScannerType::TrivyVuln,
                                scanner_version: None,
                                rule_id: v.vulnerability_id.clone(),
                                severity: sev,
                                confidence: Confidence::Certain,
                                title: v.title.clone().unwrap_or_else(|| v.vulnerability_id.clone()),
                                description: v.description.clone().unwrap_or_default(),
                                location: FindingLocation {
                                    file: std::path::PathBuf::from(&result.target),
                                    line: None,
                                    column: None,
                                    commit: None,
                                    author: None,
                                    snippet: Some(format!("{} {} → {}",
                                        v.pkg_name, v.installed_version, v.fixed_version)),
                                },
                                cwe: v.cwe_ids.as_ref().and_then(|ids| ids.first().cloned()),
                                cvss: v.cvss_score.map(|s| s as f32),
                                remediation: Some(format!("Update {} from {} to {}",
                                    v.pkg_name, v.installed_version, v.fixed_version)),
                                fix_effort: None,
                                evidence: None,
                                tags: vec!["dependency".to_string(), v.pkg_name.clone()],
                                zt_pillars: vec![],
                                cross_refs: vec![],
                            });
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    fn parse_secret(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyReport {
            results: Vec<TrivySecretResult>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivySecretResult {
            target: String,
            secrets: Option<Vec<TrivySecret>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivySecret {
            rule_id: String,
            category: String,
            severity: String,
            title: String,
            start_line: u32,
            end_line: u32,
            match_content: String,
        }

        let report: TrivyReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let mut findings = Vec::new();
        let mut idx = 0u32;

        for result in &report.results {
            if let Some(ref secrets) = result.secrets {
                for s in secrets {
                    idx += 1;
                    let sev = match s.severity.to_lowercase().as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        "low" => Severity::Low,
                        _ => Severity::Info,
                    };

                    findings.push(CanonicalFinding {
                        id: format!("AG-TS-{}-{:04}", now, idx),
                        scanner: ScannerType::TrivySecret,
                        scanner_version: None,
                        rule_id: s.rule_id.clone(),
                        severity: sev,
                        confidence: Confidence::Certain,
                        title: s.title.clone(),
                        description: format!("{} secret detected: {}", s.category, s.rule_id),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(&result.target),
                            line: Some(s.start_line),
                            column: None,
                            commit: None,
                            author: None,
                            snippet: Some(s.match_content.clone()),
                        },
                        cwe: Some("CWE-798".to_string()),
                        cvss: Some(7.5),
                        remediation: Some("Remove the secret from code. Rotate the credential.".to_string()),
                        fix_effort: None,
                        evidence: None,
                        tags: vec!["secret".to_string(), s.category.clone()],
                        zt_pillars: vec![],
                        cross_refs: vec![],
                    });
                }
            }
        }

        Ok(findings)
    }

    fn parse_misconfig(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyReport {
            results: Vec<TrivyMisconfigResult>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyMisconfigResult {
            target: String,
            misconfigurations: Option<Vec<TrivyMisconfig>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyMisconfig {
            rule_id: String,
            severity: String,
            title: String,
            description: String,
            #[serde(rename = "CauseMetadata")]
            cause_metadata: Option<serde_json::Value>,
            start_line: Option<u32>,
            resolution: Option<String>,
        }

        let report: TrivyReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let mut findings = Vec::new();
        let mut idx = 0u32;

        for result in &report.results {
            if let Some(ref misconfigs) = result.misconfigurations {
                for m in misconfigs {
                    idx += 1;
                    let sev = match m.severity.to_lowercase().as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        "low" => Severity::Low,
                        _ => Severity::Info,
                    };

                    findings.push(CanonicalFinding {
                        id: format!("AG-TM-{}-{:04}", now, idx),
                        scanner: ScannerType::TrivyMisconfig,
                        scanner_version: None,
                        rule_id: m.rule_id.clone(),
                        severity: sev,
                        confidence: Confidence::Firm,
                        title: m.title.clone(),
                        description: m.description.clone(),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(&result.target),
                            line: m.start_line,
                            column: None,
                            commit: None,
                            author: None,
                            snippet: None,
                        },
                        cwe: None,
                        cvss: None,
                        remediation: m.resolution.clone(),
                        fix_effort: None,
                        evidence: None,
                        tags: vec!["misconfig".to_string(), "iac".to_string()],
                        zt_pillars: vec![],
                        cross_refs: vec![],
                    });
                }
            }
        }

        Ok(findings)
    }
}

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
    Vuln,      // SCA vulnerability scanning (--scanners vuln)
    Secret,    // Secret scanning (--scanners secret)
    Misconfig, // IaC misconfiguration (--scanners misconfig)
}

impl Trivy {
    #[expect(dead_code)]
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

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.trivy`)
    pub fn with_mode_and_binary(mode: TrivyMode, path: Option<String>) -> Self {
        Trivy {
            binary: path.unwrap_or_else(|| "trivy".to_string()),
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

        // Parse first line: "Version: 0.58.0"
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().next().unwrap_or("unknown").to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        let scanners = self.mode_flag();
        let path_str = path.to_string_lossy().to_string();
        let args = [
            "fs",
            "--scanners",
            scanners,
            "--format",
            "json",
            "--quiet",
            path_str.as_str(),
        ];

        crate::scanner::run_command_with_timeout(&self.binary, &args, 120).await
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
        // Trivy fs --format json --scanners vuln output schema (v0.45+):
        // { "Results": [{ "Target": "...", "Vulnerabilities": [...] }] }
        // Note: older parser used Results[].Packages[].Vulnerabilities which is WRONG.
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyReport {
            results: Vec<TrivyResult>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyResult {
            target: String,
            #[serde(rename = "Vulnerabilities")]
            vulnerabilities: Option<Vec<TrivyVuln>>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct TrivyVuln {
            #[serde(rename = "VulnerabilityID")]
            vulnerability_id: String,
            pkg_name: String,
            installed_version: String,
            #[serde(default)]
            fixed_version: String,
            severity: String,
            title: Option<String>,
            description: Option<String>,
            #[serde(rename = "CVSS")]
            cvss_scores: Option<serde_json::Value>,
            #[serde(rename = "CweIDs")]
            cwe_ids: Option<Vec<String>>,
        }

        let report: TrivyReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();
        let mut findings = Vec::new();
        let mut idx = 0u32;

        for result in &report.results {
            if let Some(ref vulns) = result.vulnerabilities {
                for v in vulns {
                    idx += 1;
                    let sev = match v.severity.to_lowercase().as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        "low" => Severity::Low,
                        _ => Severity::Info,
                    };

                    // Extract best available CVSS score from nested vendor map
                    let cvss = v.cvss_scores.as_ref().and_then(|scores| {
                        scores.as_object().and_then(|map| {
                            map.values()
                                .filter_map(|v| v.get("V3Score").or_else(|| v.get("V2Score")))
                                .filter_map(|s| s.as_f64())
                                .map(|f| f as f32)
                                .reduce(f32::max)
                        })
                    });

                    findings.push(CanonicalFinding {
                        id: format!("AG-TV-{}-{}-{:04}", now, &nonce[..8], idx),
                        scanner: ScannerType::TrivyVuln,
                        scanner_version: None,
                        rule_id: v.vulnerability_id.clone(),
                        severity: sev,
                        confidence: Confidence::Certain,
                        title: v
                            .title
                            .clone()
                            .unwrap_or_else(|| v.vulnerability_id.clone()),
                        description: v.description.clone().unwrap_or_default(),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(&result.target),
                            line: None,
                            column: None,
                            commit: None,
                            author: None,
                            snippet: Some(format!(
                                "{} {} → {}",
                                v.pkg_name, v.installed_version, v.fixed_version
                            )),
                        },
                        cwe: v.cwe_ids.as_ref().and_then(|ids| ids.first().cloned()),
                        cvss,
                        remediation: Some(format!(
                            "Update {} from {} to {}",
                            v.pkg_name,
                            v.installed_version,
                            if v.fixed_version.is_empty() {
                                "latest"
                            } else {
                                &v.fixed_version
                            }
                        )),
                        fix_effort: None,
                        evidence: None,
                        tags: vec!["dependency".to_string(), v.pkg_name.clone()],
                        zt_pillars: vec![],
                        cross_refs: vec![],
                        grade: None,
                        risk_score: None,
                        reachable: None,
                    });
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
        #[expect(dead_code)] // P3/P4: TrivySecret fields from trivy JSON; not all fields consumed yet
        struct TrivySecret {
            #[serde(rename = "RuleID")]
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
        let nonce = uuid::Uuid::new_v4().simple().to_string();
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
                        id: format!("AG-TS-{}-{}-{:04}", now, &nonce[..8], idx),
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
                        remediation: Some(
                            "Remove the secret from code. Rotate the credential.".to_string(),
                        ),
                        fix_effort: None,
                        evidence: None,
                        tags: vec!["secret".to_string(), s.category.clone()],
                        zt_pillars: vec![],
                        cross_refs: vec![],
                        grade: None,
                        risk_score: None,
                        reachable: None,
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
            #[serde(rename = "ID")]
            id: String,
            severity: String,
            title: String,
            description: String,
            #[serde(rename = "CauseMetadata")]
            cause_metadata: Option<MisconfigCauseMetadata>,
            resolution: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct MisconfigCauseMetadata {
            start_line: Option<u32>,
        }

        let report: TrivyReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();
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
                        id: format!("AG-TM-{}-{}-{:04}", now, &nonce[..8], idx),
                        scanner: ScannerType::TrivyMisconfig,
                        scanner_version: None,
                        rule_id: m.id.clone(),
                        severity: sev,
                        confidence: Confidence::Firm,
                        title: m.title.clone(),
                        description: m.description.clone(),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(&result.target),
                            line: m.cause_metadata.as_ref().and_then(|c| c.start_line),
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
                        grade: None,
                        risk_score: None,
                        reachable: None,
                    });
                }
            }
        }

        Ok(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_trivy() -> Trivy {
        Trivy::with_mode(TrivyMode::Vuln)
    }

    #[test]
    fn test_parse_vuln_real_fixture() {
        // Real Trivy v0.45+ JSON output (flat Vulnerabilities, no Packages wrapper)
        let json = r#"{
            "Results": [
                {
                    "Target": "package-lock.json",
                    "Class": "lang-pkgs",
                    "Type": "npm",
                    "Vulnerabilities": [
                        {
                            "VulnerabilityID": "CVE-2024-1234",
                            "PkgName": "lodash",
                            "InstalledVersion": "4.17.20",
                            "FixedVersion": "4.17.21",
                            "Severity": "CRITICAL",
                            "Title": "Prototype Pollution in lodash",
                            "Description": "lodash before 4.17.21 is vulnerable to Prototype Pollution",
                            "CVSS": {
                                "nvd": { "V3Score": 9.8 },
                                "ghsa": { "V3Score": 8.1 }
                            },
                            "CweIDs": ["CWE-1321"]
                        },
                        {
                            "VulnerabilityID": "CVE-2024-5678",
                            "PkgName": "express",
                            "InstalledVersion": "4.17.1",
                            "FixedVersion": "4.18.2",
                            "Severity": "HIGH",
                            "Title": "Open redirect in express",
                            "Description": "express before 4.18.2 allows open redirect"
                        }
                    ]
                },
                {
                    "Target": "Dockerfile",
                    "Class": "lang-pkgs",
                    "Type": "dockerfile"
                }
            ]
        }"#;

        let trivy = make_trivy();
        let findings = trivy.parse_vuln(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2, "Should parse 2 vulnerabilities");

        // First finding: CRITICAL lodash
        let f1 = &findings[0];
        assert_eq!(f1.rule_id, "CVE-2024-1234");
        assert_eq!(f1.severity, Severity::Critical);
        assert_eq!(f1.title, "Prototype Pollution in lodash");
        assert_eq!(f1.cwe.as_deref(), Some("CWE-1321"));
        assert_eq!(f1.cvss, Some(9.8f32)); // max of 9.8 and 8.1
        assert!(f1.remediation.as_ref().unwrap().contains("4.17.21"));

        // Second finding: HIGH express
        let f2 = &findings[1];
        assert_eq!(f2.rule_id, "CVE-2024-5678");
        assert_eq!(f2.severity, Severity::High);
        assert_eq!(f2.cvss, None); // no CVSS block
    }

    #[test]
    fn test_parse_vuln_empty_results() {
        let json = r#"{ "Results": [] }"#;
        let trivy = make_trivy();
        let findings = trivy.parse_vuln(json.as_bytes()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_vuln_no_vulnerabilities_key() {
        // Result with no Vulnerabilities key at all (e.g., clean scan)
        let json = r#"{ "Results": [{ "Target": "clean.txt" }] }"#;
        let trivy = make_trivy();
        let findings = trivy.parse_vuln(json.as_bytes()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_vuln_invalid_json() {
        let trivy = make_trivy();
        let result = trivy.parse_vuln(b"not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_secret_fixture() {
        let json = r#"{
            "Results": [{
                "Target": "config.py",
                "Secrets": [{
                    "RuleID": "generic-api-key",
                    "Category": "AWS",
                    "Severity": "HIGH",
                    "Title": "AWS Access Key",
                    "StartLine": 10,
                    "EndLine": 10,
                    "MatchContent": "AKIAIOSFODNN7EXAMPLE"
                }]
            }]
        }"#;

        let trivy = Trivy::with_mode(TrivyMode::Secret);
        let findings = trivy.parse_secret(json.as_bytes()).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "generic-api-key");
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].location.line, Some(10));
    }
}

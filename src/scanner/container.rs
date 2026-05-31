// Container Image Scanner Driver
// Wraps trivy image command for scanning container images (Layer 4).
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct ContainerScanner {
    binary: String,
    image: String,
}

impl ContainerScanner {
    pub fn new(image: &str) -> Self {
        ContainerScanner {
            binary: "trivy".to_string(),
            image: image.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for ContainerScanner {
    fn name(&self) -> &'static str {
        "trivy-container"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::TrivyContainer
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

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().next().unwrap_or("unknown").to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        // `path` is unused for container scanning — we use self.image instead.
        let _ = path;

        let args = ["image", self.image.as_str(), "--format", "json", "--quiet"];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 120).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // The JSON output of `trivy image` has the same schema as `trivy fs` vuln output
        self.parse_container_vuln(raw)
    }
}

impl ContainerScanner {
    fn parse_container_vuln(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        // trivy image --format json output schema (v0.45+):
        // { "Results": [{ "Target": "...", "Vulnerabilities": [...] }] }
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
                        id: format!("AG-TC-{}-{}-{:04}", now, &nonce[..8], idx),
                        scanner: ScannerType::TrivyContainer,
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
                                v.pkg_name,
                                v.installed_version,
                                if v.fixed_version.is_empty() {
                                    "latest"
                                } else {
                                    &v.fixed_version
                                }
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
                        tags: vec![
                            "container".to_string(),
                            self.image.clone(),
                            v.pkg_name.clone(),
                        ],
                        zt_pillars: vec![],
                        cross_refs: vec![],
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

    #[test]
    fn test_parse_container_vuln_real_fixture() {
        // Real `trivy image` JSON output (same flat schema as trivy fs vuln)
        let json = r#"{
            "Results": [{
                "Target": "node:18-alpine (alpine 3.18.4)",
                "Class": "os-pkgs",
                "Type": "alpine",
                "Vulnerabilities": [{
                    "VulnerabilityID": "CVE-2023-44487",
                    "PkgName": "nghttp2",
                    "InstalledVersion": "1.55.1-r0",
                    "FixedVersion": "1.57.0-r0",
                    "Severity": "HIGH",
                    "Title": "HTTP/2 Rapid Reset Attack",
                    "Description": "The HTTP/2 protocol allows a denial of service",
                    "CVSS": { "nvd": { "V3Score": 7.5 } },
                    "CweIDs": ["CWE-400"]
                }]
            }]
        }"#;

        let scanner = ContainerScanner::new("node:18-alpine");
        let findings = scanner.parse_container_vuln(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.rule_id, "CVE-2023-44487");
        assert_eq!(f.severity, Severity::High);
        assert_eq!(f.cvss, Some(7.5f32));
        assert_eq!(f.cwe.as_deref(), Some("CWE-400"));
        assert!(f.remediation.as_ref().unwrap().contains("1.57.0-r0"));
        assert!(f.tags.contains(&"node:18-alpine".to_string()));
    }

    #[test]
    fn test_parse_container_vuln_empty() {
        let json = r#"{ "Results": [{ "Target": "clean:latest" }] }"#;
        let scanner = ContainerScanner::new("clean:latest");
        let findings = scanner.parse_container_vuln(json.as_bytes()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_container_vuln_invalid_json() {
        let scanner = ContainerScanner::new("test:latest");
        assert!(scanner.parse_container_vuln(b"{bad").is_err());
    }
}

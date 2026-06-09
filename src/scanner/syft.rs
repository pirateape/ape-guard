// Syft Scanner Driver (Layer 7)
// Wraps syft binary for SBOM generation and dependency inventory findings.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Syft {
    binary: String,
}

impl Syft {
    pub fn new() -> Self {
        Syft {
            binary: "syft".to_string(),
        }
    }

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.syft`)
    pub fn with_binary(path: Option<String>) -> Self {
        Syft {
            binary: path.unwrap_or_else(|| "syft".to_string()),
        }
    }
}

#[async_trait]
impl Scanner for Syft {
    fn name(&self) -> &'static str {
        "syft"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Syft
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install syft  |  https://github.com/anchore/syft"
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
        let path_str = path.to_string_lossy().to_string();
        let args = [path_str.as_str(), "-o", "json"];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 180).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        struct SyftReport {
            #[serde(default)]
            artifacts: Vec<SyftArtifact>,
        }

        #[derive(Deserialize)]
        struct SyftArtifact {
            #[serde(default)]
            name: String,
            #[serde(default)]
            version: String,
            #[serde(default)]
            purl: Option<String>,
            #[serde(default)]
            locations: Vec<SyftLocation>,
            #[serde(rename = "type", default)]
            kind: String,
        }

        #[derive(Deserialize)]
        struct SyftLocation {
            #[serde(default)]
            path: String,
        }

        if raw.is_empty() || raw.len() < 5 {
            return Ok(vec![]);
        }

        let report: SyftReport =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        let canonical = report
            .artifacts
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let file = a
                    .locations
                    .first()
                    .map(|l| l.path.as_str())
                    .filter(|p| !p.is_empty())
                    .unwrap_or(".");

                CanonicalFinding {
                    id: format!("AG-SY-{}-{}-{:04}", now, &nonce[..8], i + 1),
                    scanner: ScannerType::Syft,
                    scanner_version: None,
                    rule_id: format!("SBOM:{}", a.name),
                    severity: Severity::Info,
                    confidence: Confidence::Firm,
                    title: format!("Dependency discovered: {} {}", a.name, a.version),
                    description: format!(
                        "Syft cataloged package '{}' (type: {}) as part of SBOM inventory",
                        a.name, a.kind
                    ),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(file),
                        line: None,
                        column: None,
                        commit: None,
                        author: None,
                        snippet: a.purl.clone(),
                    },
                    cwe: None,
                    cvss: None,
                    remediation: None,
                    fix_effort: None,
                    evidence: a.purl.clone(),
                    tags: vec!["sbom".to_string(), "dependency".to_string(), a.kind.clone()],
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
            "artifacts": [
                {
                    "name": "serde",
                    "version": "1.0.215",
                    "type": "rust-crate",
                    "purl": "pkg:cargo/serde@1.0.215",
                    "locations": [{"path": "Cargo.lock"}]
                },
                {
                    "name": "openssl",
                    "version": "3.0.2",
                    "type": "deb",
                    "purl": "pkg:deb/debian/openssl@3.0.2",
                    "locations": [{"path": "rootfs/var/lib/dpkg/status"}]
                }
            ]
        }"#;

        let scanner = Syft::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].scanner, ScannerType::Syft);
        assert_eq!(findings[0].severity, Severity::Info);
        assert_eq!(findings[0].rule_id, "SBOM:serde");
        assert!(findings[0].tags.contains(&"sbom".to_string()));
        assert!(findings[0].title.contains("Dependency discovered"));
        assert!(findings[0].location.file.ends_with("Cargo.lock"));
    }

    #[test]
    fn test_parse_output_empty() {
        let scanner = Syft::new();
        let findings = scanner.parse_output(br#"{"artifacts":[]}"#).unwrap();
        assert!(findings.is_empty());
    }
}

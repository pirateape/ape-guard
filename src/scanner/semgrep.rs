// Semgrep Scanner Driver
// Wraps the semgrep binary for static analysis (SAST).
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct Semgrep {
    binary: String,
}

impl Semgrep {
    pub fn new() -> Self {
        Semgrep {
            binary: "semgrep".to_string(),
        }
    }
}

#[async_trait]
impl Scanner for Semgrep {
    fn name(&self) -> &'static str {
        "semgrep"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Semgrep
    }

    fn install_hint(&self) -> &'static str {
        "Install: pip install semgrep  |  brew install semgrep  |  https://semgrep.dev"
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

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError> {
        let output = tokio::process::Command::new(&self.binary)
            .arg("scan")
            .arg("--json")
            .arg("--quiet")
            .arg("--metrics")
            .arg("off")
            .arg("--config")
            .arg("p/security-audit")
            .arg("--config")
            .arg("p/owasp-top-ten")
            .arg("--use-git-ignore")
            .arg(path)
            .output()
            .await
            .map_err(ScannerError::Io)?;

        // Semgrep returns exit code 0 for no findings, 1 for findings found
        if output.status.success() || output.status.code() == Some(1) {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ScannerError::ExecutionFailed(stderr.to_string()))
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct SemgrepResults {
            results: Vec<SemgrepFinding>,
            #[serde(default)]
            errors: Vec<SemgrepError>,
        }

        #[derive(Deserialize)]
        struct SemgrepFinding {
            check_id: String,
            path: String,
            start: SemgrepLocation,
            end: SemgrepLocation,
            extra: SemgrepExtra,
        }

        #[derive(Deserialize)]
        struct SemgrepLocation {
            line: u32,
            col: u32,
        }

        #[derive(Deserialize)]
        struct SemgrepExtra {
            severity: String,
            message: String,
            metadata: Option<serde_json::Value>,
            fix: Option<String>,
            lines: Option<String>,
        }

        #[derive(Deserialize, Default)]
        #[serde(default)]
        struct SemgrepError {
            message: Option<String>,
        }

        let results: SemgrepResults =
            serde_json::from_slice(raw).map_err(|e| ScannerError::ParseFailed(e.to_string()))?;

        let now = chrono::Utc::now().format("%Y%m%d").to_string();

        let canonical: Vec<CanonicalFinding> = results
            .results
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let sev = match f.extra.severity.to_lowercase().as_str() {
                    "error" => Severity::High,
                    "warning" => Severity::Medium,
                    "info" => Severity::Low,
                    _ => Severity::Info,
                };

                // Extract CWE if present in metadata
                let cwe = f.extra.metadata.as_ref().and_then(|m| {
                    m.get("cwe")
                        .or_else(|| m.get("CWE"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                });

                CanonicalFinding {
                    id: format!("AG-SG-{}-{:04}", now, i + 1),
                    scanner: ScannerType::Semgrep,
                    scanner_version: None,
                    rule_id: f.check_id.clone(),
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: f.check_id.rsplit('.').next().unwrap_or(&f.check_id).to_string(),
                    description: f.extra.message.clone(),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(&f.path),
                        line: Some(f.start.line),
                        column: Some(f.start.col),
                        commit: None,
                        author: None,
                        snippet: f.extra.lines.clone(),
                    },
                    cwe,
                    cvss: None,
                    remediation: f.extra.fix.clone(),
                    fix_effort: None,
                    evidence: None,
                    tags: vec![],
                    zt_pillars: vec![],
                    cross_refs: vec![],
                }
            })
            .collect();

        Ok(canonical)
    }
}

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
    #[allow(dead_code)] // P3/P4: alternative constructor not wired; binary path via config instead
    pub fn new() -> Self {
        Semgrep {
            binary: "semgrep".to_string(),
        }
    }

    /// Use a custom binary path (e.g. from .apeguard.yaml `binaries.semgrep`)
    pub fn with_binary(path: Option<String>) -> Self {
        Semgrep {
            binary: path.unwrap_or_else(|| "semgrep".to_string()),
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
        let args = [
            "scan",
            "--json",
            "--quiet",
            "--metrics",
            "off",
            "--config",
            "p/default",
            path_str.as_str(),
        ];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 120).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        #[expect(dead_code)]
        struct SemgrepResults {
            results: Vec<SemgrepFinding>,
            #[serde(default)]
            errors: Vec<SemgrepError>,
        }

        #[derive(Deserialize)]
        #[expect(dead_code)] // P3/P4: SemgrepFinding fields from semgrep JSON; end field not consumed yet
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
        let nonce = uuid::Uuid::new_v4().simple().to_string();

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
                    id: format!("AG-SG-{}-{}-{:04}", now, &nonce[..8], i + 1),
                    scanner: ScannerType::Semgrep,
                    scanner_version: None,
                    rule_id: f.check_id.clone(),
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: f
                        .check_id
                        .rsplit('.')
                        .next()
                        .unwrap_or(&f.check_id)
                        .to_string(),
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
                    grade: None,
                    risk_score: None,
                    reachable: None,
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
            "results": [
                {
                    "check_id": "javascript.express.security.audit.xss.mustache-escape",
                    "path": "src/routes/api.js",
                    "start": { "line": 45, "col": 12 },
                    "end": { "line": 45, "col": 38 },
                    "extra": {
                        "severity": "ERROR",
                        "message": "User-controlled data is rendered without escaping",
                        "metadata": { "cwe": "CWE-79" },
                        "fix": "Use res.json() instead of res.send()",
                        "lines": "res.send(template.render(data))"
                    }
                },
                {
                    "check_id": "python.django.security.injection.sql.sql-format",
                    "path": "app/views.py",
                    "start": { "line": 102, "col": 5 },
                    "end": { "line": 102, "col": 60 },
                    "extra": {
                        "severity": "WARNING",
                        "message": "SQL injection via string formatting",
                        "metadata": { "CWE": "CWE-89" }
                    }
                }
            ],
            "errors": []
        }"#;

        let scanner = Semgrep::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);

        // First finding: ERROR → High
        let f1 = &findings[0];
        assert_eq!(
            f1.rule_id,
            "javascript.express.security.audit.xss.mustache-escape"
        );
        assert_eq!(f1.severity, Severity::High);
        assert_eq!(f1.location.line, Some(45));
        assert_eq!(f1.location.column, Some(12));
        assert_eq!(f1.cwe.as_deref(), Some("CWE-79"));
        assert_eq!(
            f1.remediation.as_deref(),
            Some("Use res.json() instead of res.send()")
        );
        assert_eq!(f1.title, "mustache-escape"); // last segment of check_id

        // Second finding: WARNING → Medium
        let f2 = &findings[1];
        assert_eq!(f2.severity, Severity::Medium);
        assert_eq!(f2.cwe.as_deref(), Some("CWE-89"));
    }

    #[test]
    fn test_parse_output_empty_results() {
        let json = r#"{ "results": [], "errors": [] }"#;
        let scanner = Semgrep::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_info_severity() {
        let json = r#"{
            "results": [{
                "check_id": "test.info-rule",
                "path": "test.py",
                "start": { "line": 1, "col": 1 },
                "end": { "line": 1, "col": 10 },
                "extra": { "severity": "INFO", "message": "info finding" }
            }]
        }"#;

        let scanner = Semgrep::new();
        let findings = scanner.parse_output(json.as_bytes()).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Low); // INFO → Low
    }

    #[test]
    fn test_parse_output_invalid_json() {
        let scanner = Semgrep::new();
        assert!(scanner.parse_output(b"not json").is_err());
    }
}

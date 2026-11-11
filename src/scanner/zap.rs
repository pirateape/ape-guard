// DAST Scanner Driver — OWASP ZAP (Zaproxy)
// Wraps the zap-cli binary for dynamic web application security scanning.
// ZAP provides both passive and active scanning with a rich rule set.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{generate_finding_id, is_empty_output, Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub struct ZapScanner {
    binary: String,
    target: String,
}

impl ZapScanner {
    pub fn new(target: &str) -> Self {
        ZapScanner {
            binary: "zap-cli".to_string(),
            target: target.to_string(),
        }
    }

    /// Use a custom binary path (e.g. from config)
    pub fn with_binary(target: &str, path: Option<String>) -> Self {
        ZapScanner {
            binary: path.unwrap_or_else(|| "zap-cli".to_string()),
            target: target.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for ZapScanner {
    fn name(&self) -> &'static str {
        "zap"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Zap
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install zap-cli  |  https://github.com/zaproxy/zap-cli"
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

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        // `path` is unused — we use self.target (the web URL) instead.
        crate::scanner::reject_flag_like_arg(&self.target)
            .map_err(|e| ScannerError::InvalidArgument(e.to_string()))?;

        // Run ZAP quick scan with JSON output
        // --self-contained: start and stop ZAP automatically
        // -f json: JSON output format
        let args = [
            "quick-scan",
            "--self-contained",
            "-f",
            "json",
            self.target.as_str(),
        ];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 300).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        self.parse_zap_json(raw)
    }
}

impl ZapScanner {
    fn parse_zap_json(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        if is_empty_output(raw) {
            return Ok(vec![]);
        }

        // ZAP quick-scan JSON output is an array of alert objects
        #[derive(Deserialize)]
        #[expect(dead_code)]
        struct ZapAlert {
            #[serde(rename = "alert")]
            name: String,
            #[serde(rename = "risk")]
            risk: String,
            #[serde(rename = "confidence")]
            confidence: String,
            #[serde(rename = "url")]
            url: String,
            #[serde(rename = "uri")]
            uri: Option<String>,
            #[serde(rename = "param")]
            param: Option<String>,
            #[serde(rename = "attack")]
            attack: Option<String>,
            #[serde(rename = "evidence")]
            evidence: Option<String>,
            #[serde(rename = "description")]
            description: Option<String>,
            #[serde(rename = "solution")]
            solution: Option<String>,
            #[serde(rename = "reference")]
            reference: Option<String>,
            #[serde(rename = "cweid")]
            cweid: Option<String>,
            #[serde(rename = "sourceid")]
            sourceid: Option<String>,
            #[serde(rename = "other")]
            other: Option<String>,
        }

        #[derive(Deserialize)]
        #[expect(dead_code)]
        struct ZapReport {
            #[serde(rename = "site")]
            site: Option<Vec<ZapSite>>,
        }

        #[derive(Deserialize)]
        #[expect(dead_code)]
        struct ZapSite {
            #[serde(rename = "alerts")]
            alerts: Option<Vec<ZapAlert>>,
        }

        // Try to parse as a ZAP report structure first
        let alerts: Vec<ZapAlert> = match serde_json::from_slice::<ZapReport>(raw) {
            Ok(report) => {
                let mut alerts = Vec::new();
                if let Some(sites) = report.site {
                    for site in sites {
                        if let Some(site_alerts) = site.alerts {
                            alerts.extend(site_alerts);
                        }
                    }
                }
                alerts
            }
            Err(_) => {
                // Fall back to parsing as a flat array of alerts
                match serde_json::from_slice::<Vec<ZapAlert>>(raw) {
                    Ok(a) => a,
                    Err(_) => {
                        // Try parsing as a single alert object
                        match serde_json::from_slice::<ZapAlert>(raw) {
                            Ok(a) => vec![a],
                            Err(e) => {
                                return Err(ScannerError::ParseFailed(e.to_string()));
                            }
                        }
                    }
                }
            }
        };

        let canonical: Vec<CanonicalFinding> = alerts
            .iter()
            .enumerate()
            .map(|(i, alert)| {
                let sev = match alert.risk.to_lowercase().as_str() {
                    "high" => Severity::High,
                    "medium" => Severity::Medium,
                    "low" => Severity::Low,
                    "informational" => Severity::Info,
                    _ => Severity::Info,
                };

                let cwe = alert.cweid.as_deref().and_then(|c| {
                    if c.starts_with("CWE-") {
                        Some(c.to_string())
                    } else if !c.is_empty() {
                        Some(format!("CWE-{}", c))
                    } else {
                        None
                    }
                });

                let file = alert.uri.as_deref().unwrap_or(&alert.url).to_string();

                CanonicalFinding {
                    id: generate_finding_id("AG-ZAP", i),
                    scanner: ScannerType::Zap,
                    scanner_version: None,
                    rule_id: alert.sourceid.clone().unwrap_or_else(|| alert.name.clone()),
                    severity: sev,
                    confidence: match alert.confidence.to_lowercase().as_str() {
                        "high" => Confidence::Certain,
                        "medium" => Confidence::Firm,
                        "low" => Confidence::Tentative,
                        _ => Confidence::Firm,
                    },
                    title: alert.name.clone(),
                    description: alert.description.clone().unwrap_or_default(),
                    location: FindingLocation {
                        file: PathBuf::from(file),
                        line: None,
                        column: None,
                        commit: None,
                        author: None,
                        snippet: alert.evidence.clone(),
                    },
                    cwe,
                    cvss: None,
                    remediation: alert.solution.clone(),
                    fix_effort: None,
                    evidence: alert.evidence.clone(),
                    tags: vec!["zap".to_string(), "dast".to_string()],
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
    use crate::find::{Confidence, Severity};

    #[test]
    fn test_parse_zap_json_empty() {
        let scanner = ZapScanner::new("http://example.com");
        let findings = scanner
            .parse_output(b"")
            .expect("zap test: parse_output empty should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_zap_json_array() {
        let json = r#"[
            {
                "alert": "XSS Vulnerability",
                "risk": "high",
                "confidence": "high",
                "url": "http://example.com/page",
                "uri": "/page",
                "param": "q",
                "attack": "<script>alert(1)</script>",
                "evidence": "<script>alert(1)</script> found in response",
                "description": "Reflected XSS found",
                "solution": "Encode user input",
                "reference": "https://owasp.org/www-community/attacks/xss/",
                "cweid": "CWE-79",
                "sourceid": "40012",
                "other": ""
            }
        ]"#;

        let scanner = ZapScanner::new("http://example.com");
        let findings = scanner
            .parse_output(json.as_bytes())
            .expect("zap test: parse_output should succeed");

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.title, "XSS Vulnerability");
        assert_eq!(f.severity, Severity::High);
        assert_eq!(f.confidence, Confidence::Certain);
        assert_eq!(f.cwe.as_deref(), Some("CWE-79"));
        assert_eq!(f.rule_id, "40012");
        assert!(f.location.file.to_string_lossy().contains("/page"));
        assert!(f.evidence.as_deref().unwrap_or("").contains("alert(1)"));
    }

    #[test]
    fn test_parse_zap_json_report_structure() {
        let json = r#"{
            "site": [
                {
                    "alerts": [
                        {
                            "alert": "SQL Injection",
                            "risk": "medium",
                            "confidence": "medium",
                            "url": "http://example.com/search",
                            "uri": "/search",
                            "param": "q",
                            "attack": "' OR 1=1--",
                            "evidence": "SQL syntax error in response",
                            "description": "SQL injection found",
                            "solution": "Use parameterized queries",
                            "reference": "",
                            "cweid": "CWE-89",
                            "sourceid": "40018",
                            "other": ""
                        }
                    ]
                }
            ]
        }"#;

        let scanner = ZapScanner::new("http://example.com");
        let findings = scanner
            .parse_output(json.as_bytes())
            .expect("zap test: parse_output should succeed");

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.title, "SQL Injection");
        assert_eq!(f.severity, Severity::Medium);
        assert_eq!(f.confidence, Confidence::Firm);
        assert_eq!(f.cwe.as_deref(), Some("CWE-89"));
    }

    #[test]
    fn test_parse_zap_json_unknown_risk() {
        let json = r#"[
            {
                "alert": "Info Leak",
                "risk": "unknown",
                "confidence": "low",
                "url": "http://example.com/info",
                "evidence": null,
                "description": null,
                "solution": null,
                "reference": null,
                "cweid": null,
                "sourceid": null,
                "other": null
            }
        ]"#;

        let scanner = ZapScanner::new("http://example.com");
        let findings = scanner
            .parse_output(json.as_bytes())
            .expect("zap test: parse_output should succeed");

        assert_eq!(findings.len(), 1);
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Info);
        assert_eq!(f.confidence, Confidence::Tentative);
        assert_eq!(f.cwe, None);
    }

    #[test]
    fn test_zap_scanner_name_and_type() {
        let scanner = ZapScanner::new("http://example.com");
        assert_eq!(scanner.name(), "zap");
        assert_eq!(scanner.scanner_type(), ScannerType::Zap);
    }
}

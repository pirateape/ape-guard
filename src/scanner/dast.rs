// DAST Scanner Driver (Layer 5)
// Wraps nuclei binary for dynamic web application security scanning.
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use crate::scanner::{Scanner, ScannerError};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;

pub struct DastScanner {
    binary: String,
    target: String,
}

impl DastScanner {
    pub fn new(target: &str) -> Self {
        DastScanner {
            binary: "nuclei".to_string(),
            target: target.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for DastScanner {
    fn name(&self) -> &'static str {
        "nuclei-dast"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::Nuclei
    }

    fn install_hint(&self) -> &'static str {
        "Install: brew install nuclei  |  https://github.com/projectdiscovery/nuclei"
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        if !crate::scanner::binary_exists(&self.binary) {
            return Err(ScannerError::NotFound(self.binary.clone()));
        }
        Ok(true)
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new(&self.binary)
            .arg("-version")
            .output()
            .await
            .map_err(ScannerError::Io)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().next().unwrap_or("unknown").to_string())
    }

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        // `path` is unused — we use self.target (the web URL) instead.
        // Run nuclei with JSON output
        let args = [
            "-target",
            self.target.as_str(),
            "-json",
            "-silent",
            "-no-color",
        ];
        crate::scanner::run_command_with_timeout(&self.binary, &args, 120).await
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        self.parse_nuclei_json(raw)
    }
}

impl DastScanner {
    fn parse_nuclei_json(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        #[derive(Deserialize)]
        struct NucleiResult {
            #[serde(default)]
            template_id: String,
            #[serde(default)]
            template_name: Option<String>,
            #[serde(default)]
            name: Option<String>,
            #[serde(default)]
            info: Option<NucleiInfo>,
            #[serde(default)]
            host: String,
            #[serde(default)]
            matched_at: Option<String>,
            #[serde(default)]
            severity: Option<String>,
            #[serde(default)]
            description: Option<String>,
            #[serde(default)]
            remediation: Option<String>,
            #[serde(default)]
            cwe: Option<String>,
            #[serde(default)]
            cvss_score: Option<f64>,
            #[serde(default)]
            tags: Option<Vec<String>>,
        }

        #[derive(Deserialize)]
        struct NucleiInfo {
            #[serde(default)]
            name: Option<String>,
            #[serde(default)]
            severity: Option<String>,
            #[serde(default)]
            description: Option<String>,
            #[serde(default)]
            remediation: Option<String>,
            #[serde(default)]
            cwe: Option<Vec<String>>,
            #[serde(default)]
            cvss: Option<NucleiCvss>,
        }

        #[derive(Deserialize)]
        struct NucleiCvss {
            #[serde(default)]
            score: Option<f64>,
        }

        // Nuclei outputs one JSON object per line (JSON Lines format)
        let body = String::from_utf8_lossy(raw);
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();
        let mut idx = 0u32;

        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let result: NucleiResult = match serde_json::from_str(line) {
                Ok(r) => r,
                Err(e) => {
                    // Skip malformed lines
                    tracing::debug!("Nuclei JSON parse error: {}", e);
                    continue;
                }
            };

            idx += 1;

            // Determine severity from info block or top-level
            let severity_str = result
                .info
                .as_ref()
                .and_then(|i| i.severity.as_deref())
                .or(result.severity.as_deref())
                .unwrap_or("info");

            let sev = match severity_str.to_lowercase().as_str() {
                "critical" => Severity::Critical,
                "high" => Severity::High,
                "medium" => Severity::Medium,
                "low" => Severity::Low,
                _ => Severity::Info,
            };

            // Determine title from info block or top-level
            let title = result
                .info
                .as_ref()
                .and_then(|i| i.name.as_deref())
                .or(result.name.as_deref())
                .unwrap_or(&result.template_id)
                .to_string();

            // Determine description
            let description = result
                .info
                .as_ref()
                .and_then(|i| i.description.as_deref())
                .or(result.description.as_deref())
                .unwrap_or("")
                .to_string();

            // Determine remediation
            let remediation = result
                .info
                .as_ref()
                .and_then(|i| i.remediation.as_deref())
                .or(result.remediation.as_deref())
                .map(|s| s.to_string());

            // Determine CWE
            let cwe = result
                .info
                .as_ref()
                .and_then(|i| i.cwe.as_ref())
                .and_then(|v| v.first().cloned())
                .or(result.cwe.clone());

            // Determine CVSS
            let cvss = result
                .info
                .as_ref()
                .and_then(|i| i.cvss.as_ref())
                .and_then(|c| c.score)
                .or(result.cvss_score);

            findings.push(CanonicalFinding {
                id: format!("AG-DS-{}-{}-{:04}", now, &nonce[..8], idx),
                scanner: ScannerType::Nuclei,
                scanner_version: None,
                rule_id: result.template_id.clone(),
                severity: sev,
                confidence: Confidence::Certain,
                title,
                description,
                location: FindingLocation {
                    file: std::path::PathBuf::from(&result.host),
                    line: None,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: result.matched_at,
                },
                cwe,
                cvss: cvss.map(|s| s as f32),
                remediation,
                fix_effort: None,
                evidence: None,
                tags: vec!["dast".to_string(), result.template_id],
                zt_pillars: vec![],
                cross_refs: vec![],
            });
        }

        Ok(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nuclei_json_real_fixture() {
        // Nuclei outputs one JSON object per line (JSON Lines)
        let jsonl = r#"{"template_id":"cves/2024/CVE-2024-1234","host":"https://example.com","matched_at":"https://example.com/admin","severity":"critical","info":{"name":"SQL Injection in /admin","severity":"critical","description":"The /admin endpoint is vulnerable to SQL injection","remediation":"Use parameterized queries","cwe":["CWE-89"],"cvss":{"score":9.8}},"tags":["sqli","cve"]}
{"template_id":"technologies/tech-detect","host":"https://example.com","matched_at":"https://example.com/","name":"Nginx Detected","severity":"info","tags":["tech","nginx"]}"#;

        let scanner = DastScanner::new("https://example.com");
        let findings = scanner.parse_nuclei_json(jsonl.as_bytes()).unwrap();

        assert_eq!(findings.len(), 2);

        // First finding: critical SQLi
        let f1 = &findings[0];
        assert_eq!(f1.rule_id, "cves/2024/CVE-2024-1234");
        assert_eq!(f1.severity, Severity::Critical);
        assert_eq!(f1.title, "SQL Injection in /admin");
        assert_eq!(f1.cwe.as_deref(), Some("CWE-89"));
        assert_eq!(f1.cvss, Some(9.8f32));
        assert!(f1.remediation.as_ref().unwrap().contains("parameterized"));
        assert_eq!(
            f1.location.snippet.as_deref(),
            Some("https://example.com/admin")
        );

        // Second finding: info tech detect
        let f2 = &findings[1];
        assert_eq!(f2.rule_id, "technologies/tech-detect");
        assert_eq!(f2.severity, Severity::Info);
        assert_eq!(f2.title, "Nginx Detected");
        assert_eq!(f2.cvss, None);
    }

    #[test]
    fn test_parse_nuclei_json_empty() {
        let scanner = DastScanner::new("https://example.com");
        let findings = scanner.parse_nuclei_json(b"").unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_nuclei_json_skips_malformed_lines() {
        let jsonl = "{\"template_id\":\"good\",\"host\":\"h\"}\nnot-json\n{\"template_id\":\"also-good\",\"host\":\"h\"}\n";
        let scanner = DastScanner::new("https://example.com");
        let findings = scanner.parse_nuclei_json(jsonl.as_bytes()).unwrap();
        assert_eq!(findings.len(), 2); // malformed line silently skipped
    }

    #[test]
    fn test_parse_nuclei_json_severity_from_info_block() {
        let jsonl = r#"{"template_id":"test","host":"h","info":{"severity":"high","name":"Test"}}
"#;
        let scanner = DastScanner::new("https://example.com");
        let findings = scanner.parse_nuclei_json(jsonl.as_bytes()).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::High);
    }
}

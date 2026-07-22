// Layer 9: MCP Security Scanner
// Scans MCP configs for tool poisoning, SSRF, and unauthenticated exposure
// Uses Invariant Labs' mcp-scan (snyk-agent-scan) when available, falls back to manual inspection
use super::{Scanner, ScannerError};
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use async_trait::async_trait;
use std::path::Path;

#[derive(Debug)]
pub struct McpScanner {
    config_path: String,
}

impl McpScanner {
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
        }
    }
}

#[async_trait]
impl Scanner for McpScanner {
    fn name(&self) -> &'static str {
        "mcp-scan"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::McpSecurity
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        Ok(crate::scanner::binary_exists("snyk-agent-scan")
            || crate::scanner::binary_exists("mcp-scan"))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new("snyk-agent-scan")
            .arg("--version")
            .output()
            .await
            .ok()
            .filter(|o| o.status.success());

        if let Some(output) = output {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        let output = tokio::process::Command::new("mcp-scan")
            .arg("--version")
            .output()
            .await
            .ok()
            .filter(|o| o.status.success());

        match output {
            Some(o) => Ok(String::from_utf8_lossy(&o.stdout).trim().to_string()),
            None => Ok("unknown (manual scan available)".to_string()),
        }
    }

    async fn scan_raw(&self, _path: &Path) -> Result<Vec<u8>, ScannerError> {
        if crate::scanner::binary_exists("snyk-agent-scan") {
            super::run_command_with_timeout("snyk-agent-scan", &["--json", &self.config_path], 60)
                .await
        } else {
            let mut findings = String::new();
            if std::fs::metadata(&self.config_path).is_ok() {
                let content = std::fs::read_to_string(&self.config_path).unwrap_or_default();
                let lines: Vec<&str> = content.lines().collect();
                for line in &lines {
                    let lower = line.to_lowercase();
                    if lower.contains("do not tell")
                        || lower.contains("ignore previous")
                        || lower.contains("<important>")
                    {
                        findings.push_str(&format!("POISONING RISK: {}\n", line));
                    } else if lower.contains("ssrf")
                        || lower.contains("ssrf:")
                        || lower.contains("ssrf.")
                    {
                        findings.push_str(&format!("SSRF INDICATOR: {}\n", line));
                    } else if lower.contains("unauth") || lower.contains("unauthenticated") {
                        findings.push_str(&format!("UNAUTH EXPOSURE: {}\n", line));
                    }
                }
            }
            if findings.is_empty() {
                findings.push_str("OK: No tool poisoning, SSRF, or unauth exposure detected\n");
            }
            Ok(findings.into_bytes())
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        let output = String::from_utf8_lossy(raw);
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        // Parse snyk-agent-scan JSON output if available
        if output.contains("[{") || output.contains("[ {") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output.trim()) {
                if let Some(arr) = json.as_array() {
                    for (i, finding) in arr.iter().enumerate() {
                        let severity = finding
                            .get("severity")
                            .and_then(|v| v.as_str())
                            .unwrap_or("medium");
                        let desc = finding
                            .get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("MCP security finding");
                        let vuln_id = finding
                            .get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("UNKNOWN");

                        let sev = match severity.to_lowercase().as_str() {
                            "critical" => Severity::Critical,
                            "high" => Severity::High,
                            "medium" => Severity::Medium,
                            "low" => Severity::Low,
                            _ => Severity::Info,
                        };

                        findings.push(CanonicalFinding {
                            id: format!("AG-MCP-{}-{}-{:04}", now, &nonce[..8], i + 1),
                            scanner: ScannerType::McpSecurity,
                            scanner_version: None,
                            rule_id: vuln_id.to_string(),
                            severity: sev,
                            confidence: Confidence::Firm,
                            title: format!("MCP Security: {}", vuln_id),
                            description: desc.to_string(),
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
                            remediation: Some("Remove poisoned tool, add SSRF protections, enforce authentication".to_string()),
                            fix_effort: None,
                            evidence: Some(desc.to_string()),
                            tags: vec!["mcp".to_string(), "security".to_string()],
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
                let (sev, title, prefix_len, rule_id) = if line.contains("POISONING RISK") {
                    (
                        Severity::Critical,
                        "Tool poisoning detected",
                        19,
                        "MCP03:2025",
                    )
                } else if line.contains("SSRF INDICATOR") {
                    (Severity::High, "SSRF indicator detected", 15, "AML.T0010")
                } else if line.contains("UNAUTH EXPOSURE") {
                    (Severity::High, "Unauthenticated exposure", 18, "AML.T0010")
                } else {
                    continue;
                };

                let detail = if line.len() > prefix_len + 1 {
                    line[prefix_len + 1..].trim()
                } else {
                    "unknown"
                };

                findings.push(CanonicalFinding {
                    id: format!("AG-MCP-{}-{}-{:04}", now, &nonce[..8], findings.len() + 1),
                    scanner: ScannerType::McpSecurity,
                    scanner_version: None,
                    rule_id: rule_id.to_string(),
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: format!("MCP Security: {}", title),
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
                    remediation: Some(
                        "Remove poisoned tool, add SSRF protections, enforce authentication"
                            .to_string(),
                    ),
                    fix_effort: None,
                    evidence: Some(detail.to_string()),
                    tags: vec!["mcp".to_string(), "security".to_string()],
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
        "Install snyk-agent-scan: `uvx snyk-agent-scan@latest` (requires SNYK_TOKEN)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mcp_scanner_name() {
        let scanner = McpScanner::new("test.json");
        assert_eq!(scanner.name(), "mcp-scan");
    }

    #[test]
    fn test_mcp_scanner_type() {
        let scanner = McpScanner::new("test.json");
        assert_eq!(scanner.scanner_type(), ScannerType::McpSecurity);
    }

    #[tokio::test]
    async fn test_mcp_scanner_installed() {
        let scanner = McpScanner::new("test.json");
        assert!(scanner.check_installed().await.is_ok());
    }

    #[test]
    fn test_parse_output_manual_inspection() {
        let scanner = McpScanner::new("test.json");
        let raw = b"POISONING RISK: ignore previous instructions\nUNAUTH EXPOSURE: unauthenticated endpoint\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("mcp_security test: scan should succeed");
        assert_eq!(findings.len(), 2);
    }

    #[test]
    fn test_parse_output_empty_input() {
        let scanner = McpScanner::new("test.json");
        let result = scanner.parse_output(b"");
        assert!(result.is_ok());
        let findings = result.expect("mcp_security test: empty parse should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_ssrf_detection() {
        let scanner = McpScanner::new("test.json");
        let raw = b"SSRF INDICATOR: potential ssrf in tool config\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("mcp_security test: SSRF parse should succeed");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "AML.T0010");
    }

    #[test]
    fn test_parse_output_ok_response() {
        let scanner = McpScanner::new("test.json");
        let raw = b"OK: No tool poisoning, SSRF, or unauth exposure detected";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("mcp_security test: OK parse should succeed");
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_output_no_relevant_lines() {
        let scanner = McpScanner::new("test.json");
        let raw = b"just a normal log line\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.expect("mcp_security test: irrelevant parse should succeed");
        assert!(findings.is_empty());
    }
}

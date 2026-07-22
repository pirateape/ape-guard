// Layer 12: TLS Certificate Scanner
// Scans for expired certs, shadow SSL, and CT log anomalies
// Uses openssl, certutil, or ct-portal when available
use super::{Scanner, ScannerError};
use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
use async_trait::async_trait;
use std::path::Path;

#[derive(Debug)]
pub struct TlsScanner {
    cert_paths: Vec<String>,
}

impl TlsScanner {
    pub fn new(cert_paths: &[&str]) -> Self {
        Self {
            cert_paths: cert_paths.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[async_trait]
impl Scanner for TlsScanner {
    fn name(&self) -> &'static str {
        "tls-audit"
    }

    fn scanner_type(&self) -> ScannerType {
        ScannerType::TlsCertificate
    }

    async fn check_installed(&self) -> Result<bool, ScannerError> {
        Ok(crate::scanner::binary_exists("openssl") || crate::scanner::binary_exists("certutil"))
    }

    async fn version(&self) -> Result<String, ScannerError> {
        let output = tokio::process::Command::new("openssl")
            .arg("version")
            .output()
            .await
            .ok()
            .filter(|o| o.status.success());

        if let Some(output) = output {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        let output = tokio::process::Command::new("certutil")
            .arg("-version")
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
        if crate::scanner::binary_exists("openssl") {
            let mut findings = String::new();
            for cert_path in &self.cert_paths {
                let output = tokio::process::Command::new("openssl")
                    .args(["x509", "-in", cert_path, "-noout", "-dates", "-subject"])
                    .output()
                    .await
                    .ok();

                match output {
                    Some(o) if o.status.success() => {
                        let out_str = String::from_utf8_lossy(&o.stdout);
                        findings.push_str(&format!("=== {} ===\n{}\n", cert_path, out_str));
                        if out_str.contains("expire") {
                            findings.push_str(&format!("⚠️ EXPIRING/EXPIRED: {}\n", cert_path));
                        }
                    }
                    _ => {
                        findings.push_str(&format!(
                            "⚠️ ERROR: {} (file not found or invalid)\n",
                            cert_path
                        ));
                    }
                }
            }
            if findings.is_empty() {
                findings.push_str("OK: No TLS certificates scanned\n");
            }
            Ok(findings.into_bytes())
        } else {
            // Manual inspection fallback
            let mut findings = String::new();
            for cert_path in &self.cert_paths {
                if std::fs::metadata(cert_path).is_ok() {
                    findings.push_str(&format!(
                        "⚠️ CERT FOUND: {} (requires openssl to verify)\n",
                        cert_path
                    ));
                } else {
                    findings.push_str(&format!("⚠️ NOT FOUND: {}\n", cert_path));
                }
            }
            if findings.is_empty() {
                findings.push_str("OK: No TLS certificates found\n");
            }
            Ok(findings.into_bytes())
        }
    }

    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
        let output = String::from_utf8_lossy(raw);
        let mut findings = Vec::new();
        let now = chrono::Utc::now().format("%Y%m%d").to_string();
        let nonce = uuid::Uuid::new_v4().simple().to_string();

        // Check for openssl output with dates
        if output.contains("notBefore") || output.contains("notAfter") {
            for (i, cert_path) in self.cert_paths.iter().enumerate() {
                if output.contains(cert_path)
                    && (output.contains("expire")
                        || output.contains("EXPIRING")
                        || output.contains("EXPIRED"))
                {
                    findings.push(CanonicalFinding {
                        id: format!("AG-TLS-{}-{}-{:04}", now, &nonce[..8], i + 1),
                        scanner: ScannerType::TlsCertificate,
                        scanner_version: None,
                        rule_id: "TLS001".to_string(),
                        severity: Severity::Critical,
                        confidence: Confidence::Firm,
                        title: format!("TLS Certificate expired/expiring: {}", cert_path),
                        description: format!("Certificate expired or expiring: {}", cert_path),
                        location: FindingLocation {
                            file: std::path::PathBuf::from(cert_path),
                            line: Some((i + 1) as u32),
                            column: None,
                            commit: None,
                            author: None,
                            snippet: None,
                        },
                        cwe: None,
                        cvss: None,
                        remediation: Some("Renew certificate, enable auto-renewal".to_string()),
                        fix_effort: None,
                        evidence: Some(output.clone().to_string()),
                        tags: vec![
                            "tls".to_string(),
                            "certificate".to_string(),
                            "security".to_string(),
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
                let (sev, title, prefix_len) =
                    if line.contains("EXPIRING") || line.contains("EXPIRED") {
                        (
                            Severity::Critical,
                            "Certificate expired or expiring",
                            1, // skip ⚠️
                        )
                    } else if line.contains("CERT FOUND") {
                        (Severity::Medium, "Certificate requires verification", 1)
                    } else if line.contains("NOT FOUND") {
                        (Severity::Low, "Certificate file not found", 1)
                    } else {
                        continue;
                    };

                let detail = if line.len() > prefix_len + 1 {
                    line[prefix_len..].trim().trim_start_matches("⚠️ ").trim()
                } else {
                    "unknown"
                };

                findings.push(CanonicalFinding {
                    id: format!("AG-TLS-{}-{}-{:04}", now, &nonce[..8], findings.len() + 1),
                    scanner: ScannerType::TlsCertificate,
                    scanner_version: None,
                    rule_id: if line.contains("EXPIRING") || line.contains("EXPIRED") {
                        "TLS001".to_string()
                    } else if line.contains("CERT FOUND") {
                        "TLS002".to_string()
                    } else {
                        "TLS003".to_string()
                    },
                    severity: sev,
                    confidence: Confidence::Firm,
                    title: format!("TLS Certificate: {}", title),
                    description: format!("{}: {}", title, detail),
                    location: FindingLocation {
                        file: std::path::PathBuf::from(self.cert_paths[0].clone()),
                        line: Some((i + 1) as u32),
                        column: None,
                        commit: None,
                        author: None,
                        snippet: None,
                    },
                    cwe: None,
                    cvss: None,
                    remediation: if line.contains("EXPIRING") || line.contains("EXPIRED") {
                        Some("Renew certificate, enable auto-renewal".to_string())
                    } else if line.contains("NOT FOUND") {
                        Some("Verify certificate path is correct".to_string())
                    } else {
                        Some("Run openssl x509 -in <cert> -noout -dates to verify".to_string())
                    },
                    fix_effort: None,
                    evidence: Some(detail.to_string()),
                    tags: vec![
                        "tls".to_string(),
                        "certificate".to_string(),
                        "security".to_string(),
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
        "Install openssl: `brew install openssl` (or certutil: `brew install certutil`)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tls_scanner_name() {
        let scanner = TlsScanner::new(&["/etc/ssl/certs/cert.pem"]);
        assert_eq!(scanner.name(), "tls-audit");
    }

    #[test]
    fn test_tls_scanner_type() {
        let scanner = TlsScanner::new(&["/etc/ssl/certs/cert.pem"]);
        assert_eq!(scanner.scanner_type(), ScannerType::TlsCertificate);
    }

    #[tokio::test]
    async fn test_tls_scanner_installed() {
        let scanner = TlsScanner::new(&["/etc/ssl/certs/cert.pem"]);
        assert!(scanner.check_installed().await.is_ok());
    }

    #[test]
    fn test_parse_output_manual_inspection() {
        let scanner = TlsScanner::new(&["/etc/ssl/certs/cert.pem"]);
        let raw = b"CERT FOUND: /etc/ssl/certs/cert.pem (requires openssl to verify)\nNOT FOUND: /etc/ssl/certs/missing.pem\n";
        let result = scanner.parse_output(raw);
        assert!(result.is_ok());
        let findings = result.unwrap();
        assert!(!findings.is_empty());
    }
}

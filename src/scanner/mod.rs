// Scanner Driver Trait
// Each scanner implements this trait. New scanners can be added via the plugin system.
use crate::find::{CanonicalFinding, ScannerType};
use async_trait::async_trait;
use std::path::Path;
use std::time::Duration;

/// Check if a binary exists on the system PATH
pub fn binary_exists(binary: &str) -> bool {
    which::which(binary).is_ok()
}

/// Run a command with a timeout, returning its stdout on success
pub async fn run_command_with_timeout(
    binary: &str,
    args: &[&str],
    timeout_secs: u64,
) -> Result<Vec<u8>, ScannerError> {
    let cmd = tokio::process::Command::new(binary).args(args).output();

    let timeout_dur = Duration::from_secs(timeout_secs);
    match tokio::time::timeout(timeout_dur, cmd).await {
        Ok(Ok(output)) => {
            if output.status.success() || output.stdout.len() > 10 {
                Ok(output.stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(ScannerError::ExecutionFailed(stderr.to_string()))
            }
        }
        Ok(Err(e)) => Err(ScannerError::Io(e)),
        Err(_) => Err(ScannerError::Timeout(timeout_secs)),
    }
}

#[async_trait]
pub trait Scanner: Send + Sync {
    /// Name of the scanner (e.g., "gitleaks", "semgrep")
    fn name(&self) -> &'static str;

    /// The scanner type enum variant
    fn scanner_type(&self) -> ScannerType;

    /// Check if this scanner is installed and at a compatible version
    async fn check_installed(&self) -> Result<bool, ScannerError>;

    /// Get the installed version string
    async fn version(&self) -> Result<String, ScannerError>;

    /// Run the scanner on the given path and return raw JSON output
    async fn scan_raw(&self, path: &Path) -> Result<Vec<u8>, ScannerError>;

    /// Parse raw scanner output into canonical findings
    fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>;

    /// Full scan: check installed + run + parse
    async fn scan(&self, path: &Path) -> Result<ScannerResult, ScannerError> {
        let installed = self.check_installed().await?;
        if !installed {
            return Ok(ScannerResult::NotInstalled {
                name: self.name(),
                hint: self.install_hint(),
            });
        }

        let version = self.version().await.ok();
        let raw = self.scan_raw(path).await?;
        let findings = self.parse_output(&raw)?;

        Ok(ScannerResult::Complete {
            name: self.name().to_string(),
            version,
            finding_count: findings.len(),
            findings,
        })
    }

    /// Install hint shown when scanner is missing
    fn install_hint(&self) -> &'static str;
}

#[derive(Debug)]
#[allow(dead_code)] // Error variant is P3 (DAST), rest used
pub enum ScannerResult {
    Complete {
        name: String,
        version: Option<String>,
        finding_count: usize,
        findings: Vec<CanonicalFinding>,
    },
    NotInstalled {
        name: &'static str,
        hint: &'static str,
    },
    Error {
        name: &'static str,
        error: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("Scanner not found: {0}")]
    NotFound(String),

    #[error("Scanner execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Output parsing failed: {0}")]
    ParseFailed(String),

    #[error("Timeout after {0}s")]
    Timeout(u64),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub mod checkov;
pub mod container;
pub mod dast;
pub mod gitleaks;
pub mod semgrep;
pub mod syft;
pub mod trivy;

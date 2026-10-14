// Scanner Driver Trait
// Each scanner implements this trait. New scanners can be added via the plugin system.
//
// ## Scanner Deduplication Pattern
//
// To reduce code duplication across scanner implementations, use these helper functions:
//
// - `binary_exists()` - Check if a binary exists on PATH
// - `generate_finding_id()` - Generate consistent finding IDs with timestamp/nonce
// - `is_empty_output()` - Check for empty/minimal scanner output
// - `run_command_with_timeout()` - Execute scanner binaries with timeout
//
// Example usage in parse_output():
// ```rust
// use crate::scanner::{generate_finding_id, is_empty_output};
//
// fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError> {
//     if is_empty_output(raw) {
//         return Ok(vec![]);
//     }
//     // ... parsing logic ...
//     let canonical = findings.iter().enumerate().map(|(i, f)| {
//         CanonicalFinding {
//             id: generate_finding_id("AG-CK", i), // AG-CK-20260828-abc123-0001
//             // ... other fields ...
//         }
//     }).collect();
//     Ok(canonical)
// }
// ```
use crate::find::{CanonicalFinding, ScannerType};
use async_trait::async_trait;
use std::path::Path;
use std::time::Duration;

/// Check if a binary exists on the system PATH
pub fn binary_exists(binary: &str) -> bool {
    which::which(binary).is_ok()
}

/// Helper function to generate finding IDs with timestamp and nonce
/// Reduces duplication in parse_output implementations
///
/// # Arguments
/// * `prefix` - Scanner-specific prefix (e.g., "AG-CK" for Checkov)
/// * `index` - Finding index for uniqueness
///
/// # Returns
/// Formatted ID like "AG-CK-20260828-abc123-0001"
pub fn generate_finding_id(prefix: &str, index: usize) -> String {
    let now = chrono::Utc::now().format("%Y%m%d").to_string();
    let nonce = uuid::Uuid::new_v4().simple().to_string();
    format!("{}-{}-{}-{:04}", prefix, now, &nonce[..8], index + 1)
}

/// Helper function to check for empty/minimal output
/// Common pattern across scanner parsers
///
/// # Arguments
/// * `raw` - Raw scanner output bytes
///
/// # Returns
/// true if output is empty or too small to be valid JSON
pub fn is_empty_output(raw: &[u8]) -> bool {
    raw.is_empty() || raw.len() < 5
}

/// Reject an argument that begins with `-` to prevent command **argument
/// injection**: a scanner target path such as `-noout` or `--help` would
/// otherwise be parsed by the wrapped CLI as a flag rather than a path.
/// Returns the argument unchanged when it is safe to use.
pub fn reject_flag_like_arg(arg: &str) -> anyhow::Result<&str> {
    if arg.starts_with('-') {
        anyhow::bail!("refusing to use argument that looks like a flag: {arg}");
    }
    Ok(arg)
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
            if output.status.success() {
                Ok(output.stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(ScannerError::ExecutionFailed(stderr.trim().to_string()))
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
    #[allow(dead_code)] // P3/P4: scanner_type() defined on trait but not yet called by any consumer
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
#[expect(dead_code)] // Error variant is P3 (DAST), rest used
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

    #[error("Invalid scanner argument: {0}")]
    InvalidArgument(String),

    #[error("Scanner execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Output parsing failed: {0}")]
    ParseFailed(String),

    #[error("Timeout after {0}s")]
    Timeout(u64),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_scanner_name() {
        use crate::scanner::arch::ArchitectureAnalyzer;
        use std::path::Path;

        let analyzer = ArchitectureAnalyzer::new(Path::new("."));
        assert_eq!(analyzer.name(), "architecture");
    }

    #[test]
    fn test_arch_scanner_type() {
        use crate::find::ScannerType;
        use crate::scanner::arch::ArchitectureAnalyzer;
        use std::path::Path;

        let analyzer = ArchitectureAnalyzer::new(Path::new("."));
        assert_eq!(analyzer.scanner_type(), ScannerType::Architecture);
    }

    #[tokio::test]
    async fn test_arch_scanner_installed() {
        use crate::scanner::arch::ArchitectureAnalyzer;
        use std::path::Path;

        let analyzer = ArchitectureAnalyzer::new(Path::new("."));
        assert!(analyzer.check_installed().await.is_ok());
    }

    #[tokio::test]
    async fn test_arch_scanner_version() {
        use crate::scanner::arch::ArchitectureAnalyzer;
        use std::path::Path;

        let analyzer = ArchitectureAnalyzer::new(Path::new("."));
        assert!(analyzer.version().await.is_ok());
    }
}

pub mod arch;
pub mod aws_s3;
pub mod checkov;
pub mod container;
pub mod context_drift;
pub mod dast;
pub mod gitleaks;
pub mod mcp_security;
pub mod semgrep;
pub mod syft;
pub mod terraform;
pub mod tls;
pub mod trivy;
pub mod trufflehog;

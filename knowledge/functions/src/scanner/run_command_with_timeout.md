---
type: Rust Function
title: run_command_with_timeout
resource: src/scanner/mod.rs#L14-L34
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/checkov/Checkov/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/container/ContainerScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/DastScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/gitleaks/Gitleaks/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/mcp_security/McpScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/semgrep/Semgrep/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/syft/Syft/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/terraform/TerraformScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn run_command_with_timeout( binary: &str, args: &[&str], timeout_secs: u64, ) -> Result<Vec<u8>, ScannerError>`

# Called by

- [scan_raw](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/checkov/Checkov/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/container/ContainerScanner/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/dast/DastScanner/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/mcp_security/McpScanner/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/semgrep/Semgrep/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/syft/Syft/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/terraform/TerraformScanner/scanner/scan_raw.md)
- [scan_raw](../../../functions/src/scanner/trivy/Trivy/scanner/scan_raw.md)
---
type: Rust Function
title: binary_exists
resource: src/scanner/mod.rs#L9-L11
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/checkov/Checkov/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/container/ContainerScanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/dast/DastScanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/gitleaks/Gitleaks/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/mcp_security/McpScanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/mcp_security/McpScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/semgrep/Semgrep/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/syft/Syft/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/terraform/TerraformScanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/terraform/TerraformScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/tls/TlsScanner/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/tls/TlsScanner/scanner/scan_raw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trufflehog/Trufflehog/scanner/check_installed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn binary_exists(binary: &str) -> bool`

# Called by

- [check_installed](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/check_installed.md)
- [scan_raw](../../../functions/src/scanner/aws_s3/AwsS3Scanner/scanner/scan_raw.md)
- [check_installed](../../../functions/src/scanner/checkov/Checkov/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/container/ContainerScanner/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/dast/DastScanner/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/gitleaks/Gitleaks/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/mcp_security/McpScanner/scanner/check_installed.md)
- [scan_raw](../../../functions/src/scanner/mcp_security/McpScanner/scanner/scan_raw.md)
- [check_installed](../../../functions/src/scanner/semgrep/Semgrep/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/syft/Syft/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/terraform/TerraformScanner/scanner/check_installed.md)
- [scan_raw](../../../functions/src/scanner/terraform/TerraformScanner/scanner/scan_raw.md)
- [check_installed](../../../functions/src/scanner/tls/TlsScanner/scanner/check_installed.md)
- [scan_raw](../../../functions/src/scanner/tls/TlsScanner/scanner/scan_raw.md)
- [check_installed](../../../functions/src/scanner/trivy/Trivy/scanner/check_installed.md)
- [check_installed](../../../functions/src/scanner/trufflehog/Trufflehog/scanner/check_installed.md)
---
type: Rust Function
title: handle_scan_tool
resource: src/mcp.rs#L284-L509
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/mcp/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/with_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/normalize_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/cross_reference
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/dedup/deduplicate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/record_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/gitleaks/Gitleaks/with_binary
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/trufflehog/Trufflehog/with_binary
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/semgrep/Semgrep/with_binary
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/container/ContainerScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/dast/DastScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/checkov/Checkov/with_binary
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/syft/Syft/with_binary
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/context_drift/ContextDriftScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/mcp_security/McpScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/terraform/TerraformScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/aws_s3/AwsS3Scanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/tls/TlsScanner/new
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/scanner/Scanner/scan
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/mcp/handle_call_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_scan_tool(args: &Value) -> anyhow::Result<Value>`

# Calls

- [load_effective_config](../../../functions/src/mcp/load_effective_config.md)
- [open](../../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../../functions/src/cache/ScanCache/enforce_ttl.md)
- [with_mode](../../../functions/src/scanner/trivy/Trivy/with_mode.md)
- [normalize_findings](../../../functions/src/normalize/normalize_findings.md)
- [cross_reference](../../../functions/src/dedup/cross_reference.md)
- [deduplicate](../../../functions/src/dedup/deduplicate.md)
- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)
- [record_scan](../../../functions/src/cache/ScanCache/record_scan.md)
- [with_binary](../../../functions/src/scanner/gitleaks/Gitleaks/with_binary.md)
- [with_binary](../../../functions/src/scanner/trufflehog/Trufflehog/with_binary.md)
- [with_binary](../../../functions/src/scanner/semgrep/Semgrep/with_binary.md)
- [new](../../../functions/src/scanner/container/ContainerScanner/new.md)
- [new](../../../functions/src/scanner/dast/DastScanner/new.md)
- [with_binary](../../../functions/src/scanner/checkov/Checkov/with_binary.md)
- [with_binary](../../../functions/src/scanner/syft/Syft/with_binary.md)
- [new](../../../functions/src/scanner/context_drift/ContextDriftScanner/new.md)
- [new](../../../functions/src/scanner/mcp_security/McpScanner/new.md)
- [new](../../../functions/src/scanner/terraform/TerraformScanner/new.md)
- [new](../../../functions/src/scanner/aws_s3/AwsS3Scanner/new.md)
- [new](../../../functions/src/scanner/tls/TlsScanner/new.md)
- [scan](../../../functions/src/scanner/Scanner/scan.md)

# Called by

- [handle_call_tool](../../../functions/src/mcp/handle_call_tool.md)
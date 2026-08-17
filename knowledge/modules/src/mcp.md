---
type: Rust Module
title: mcp
resource: src/mcp.rs#L1-L937
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-cache-scancache
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-bufread-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-scanner-aws-s3-awss3scanner-checkov-checkov-container-containerscanner-context-drift-dast-dastscanner-gitleaks-gitleaks-mcp-security-mcpscanner-semgrep-semgrep-syft-syft-terraform-terraformscanner-tls-tlsscanner-trivy-trivy-trufflehog-trufflehog-scanner-scannerresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [load_effective_config](../../functions/src/mcp/load_effective_config.md)
- [summarize_findings_by_severity](../../functions/src/mcp/summarize_findings_by_severity.md)
- [load_cached_findings](../../functions/src/mcp/load_cached_findings.md)
- [serve](../../functions/src/mcp/serve.md)
- [handle_request](../../functions/src/mcp/handle_request.md)
- [handle_initialize](../../functions/src/mcp/handle_initialize.md)
- [handle_list_tools](../../functions/src/mcp/handle_list_tools.md)
- [handle_call_tool](../../functions/src/mcp/handle_call_tool.md)
- [handle_scan_tool](../../functions/src/mcp/handle_scan_tool.md)
- [handle_findings_tool](../../functions/src/mcp/handle_findings_tool.md)
- [handle_scorecard_tool](../../functions/src/mcp/handle_scorecard_tool.md)
- [handle_chains_tool](../../functions/src/mcp/handle_chains_tool.md)
- [handle_arch_tool](../../functions/src/mcp/handle_arch_tool.md)
- [handle_resource_list](../../functions/src/mcp/handle_resource_list.md)
- [handle_resource_read](../../functions/src/mcp/handle_resource_read.md)
- [test_initialize_response](../../functions/src/mcp/test_initialize_response.md)
- [test_list_tools_response](../../functions/src/mcp/test_list_tools_response.md)
- [test_resource_list](../../functions/src/mcp/test_resource_list.md)
- [test_resource_read_missing_uri](../../functions/src/mcp/test_resource_read_missing_uri.md)
- [test_resource_read_unknown_uri](../../functions/src/mcp/test_resource_read_unknown_uri.md)
- [test_handle_resources_read_valid](../../functions/src/mcp/test_handle_resources_read_valid.md)
- [test_handle_initialize_valid](../../functions/src/mcp/test_handle_initialize_valid.md)
- [test_handle_list_tools_valid](../../functions/src/mcp/test_handle_list_tools_valid.md)
- [test_handle_unknown_method](../../functions/src/mcp/test_handle_unknown_method.md)
- [test_handle_invalid_json](../../functions/src/mcp/test_handle_invalid_json.md)

# Imports

- `crate::cache::ScanCache`
- `crate::find::*`
- `serde_json::{json, Value}`
- `std::io::{self, BufRead, Write}`
- `std::path::PathBuf`
- `crate::scanner::{
        aws_s3::AwsS3Scanner, checkov::Checkov, container::ContainerScanner, context_drift,
        dast::DastScanner, gitleaks::Gitleaks, mcp_security::McpScanner, semgrep::Semgrep,
        syft::Syft, terraform::TerraformScanner, tls::TlsScanner, trivy::Trivy,
        trufflehog::Trufflehog, Scanner, ScannerResult,
    }`
- `super::*`

# Member of

- [apeguard](../../packages/apeguard.md)
---
type: Rust Function
title: run_scan
resource: src/orchestrate.rs#L257-L731
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/cache/ScanCache/disabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/enforce_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/with_mode_and_binary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/load_completed_scanners
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/verify/drift_findings_to_canonical
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
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/apply_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/apply_fp_filters
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/filter/FilterStats/total_removed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/enhance_remediations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/grade_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/grade/count_verdicts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/filter_by_severity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/policy/evaluate_policies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/check_fail_on
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/build_attack_chains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/score/score_all_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stride/analyze_stride_coverage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/generate_summary_report
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
  - target: functions/src/score/ScoreWeights/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn run_scan( args: ScanArgs<'_>, cfg: &config::Config, ) -> anyhow::Result<ScanOutput>`

# Calls

- [disabled](../../../functions/src/cache/ScanCache/disabled.md)
- [open](../../../functions/src/cache/ScanCache/open.md)
- [enforce_ttl](../../../functions/src/cache/ScanCache/enforce_ttl.md)
- [with_mode_and_binary](../../../functions/src/scanner/trivy/Trivy/with_mode_and_binary.md)
- [load_completed_scanners](../../../functions/src/orchestrate/load_completed_scanners.md)
- [scan_drift](../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
- [drift_findings_to_canonical](../../../functions/src/scanner/context_drift/verify/drift_findings_to_canonical.md)
- [normalize_findings](../../../functions/src/normalize/normalize_findings.md)
- [cross_reference](../../../functions/src/dedup/cross_reference.md)
- [deduplicate](../../../functions/src/dedup/deduplicate.md)
- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
- [apply_reachability](../../../functions/src/reachability/apply_reachability.md)
- [apply_fp_filters](../../../functions/src/filter/apply_fp_filters.md)
- [total_removed](../../../functions/src/filter/FilterStats/total_removed.md)
- [enhance_remediations](../../../functions/src/llm/enhance_remediations.md)
- [grade_findings](../../../functions/src/grade/grade_findings.md)
- [count_verdicts](../../../functions/src/grade/count_verdicts.md)
- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)
- [filter_by_severity](../../../functions/src/orchestrate/filter_by_severity.md)
- [evaluate_policies](../../../functions/src/policy/evaluate_policies.md)
- [check_fail_on](../../../functions/src/orchestrate/check_fail_on.md)
- [build_attack_chains](../../../functions/src/chain/build_attack_chains.md)
- [score_all_findings](../../../functions/src/score/score_all_findings.md)
- [analyze_stride_coverage](../../../functions/src/stride/analyze_stride_coverage.md)
- [generate_summary_report](../../../functions/src/orchestrate/generate_summary_report.md)
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
- [default](../../../functions/src/score/ScoreWeights/default/default.md)

# Called by

- [main](../../../functions/src/main.md)
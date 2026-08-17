---
type: Rust Function
title: compute_zt_scorecard
resource: src/normalize.rs#L159-L243
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/normalize/compute_gap_analysis
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scorecard_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_resource_read
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_scorecard_no_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_scorecard_single_finding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_scorecard_multiple_findings_capped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_scorecard_maturity_levels
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_gap_analysis_no_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_gap_analysis_with_secrets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_zt_scorecard(findings: &[CanonicalFinding]) -> ZeroTrustScorecard`

# Calls

- [compute_gap_analysis](../../../functions/src/normalize/compute_gap_analysis.md)

# Called by

- [run_report](../../../functions/src/run_report.md)
- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [handle_scorecard_tool](../../../functions/src/mcp/handle_scorecard_tool.md)
- [handle_resource_read](../../../functions/src/mcp/handle_resource_read.md)
- [test_scorecard_no_findings](../../../functions/src/normalize/test_scorecard_no_findings.md)
- [test_scorecard_single_finding](../../../functions/src/normalize/test_scorecard_single_finding.md)
- [test_scorecard_multiple_findings_capped](../../../functions/src/normalize/test_scorecard_multiple_findings_capped.md)
- [test_scorecard_maturity_levels](../../../functions/src/normalize/test_scorecard_maturity_levels.md)
- [test_gap_analysis_no_findings](../../../functions/src/normalize/test_gap_analysis_no_findings.md)
- [test_gap_analysis_with_secrets](../../../functions/src/normalize/test_gap_analysis_with_secrets.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
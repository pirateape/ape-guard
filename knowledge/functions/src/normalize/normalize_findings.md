---
type: Rust Function
title: normalize_findings
resource: src/normalize.rs#L127-L156
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_secret
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_injection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_xss
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_dependency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_misconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_default_pillar
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_zt_mapping_no_duplicates
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
  - target: functions/src/normalize/test_multiple_zt_pillars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_dast_keyword_mappings_app_and_network
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

`pub fn normalize_findings(findings: &mut [CanonicalFinding])`

# Called by

- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [test_zt_mapping_secret](../../../functions/src/normalize/test_zt_mapping_secret.md)
- [test_zt_mapping_injection](../../../functions/src/normalize/test_zt_mapping_injection.md)
- [test_zt_mapping_xss](../../../functions/src/normalize/test_zt_mapping_xss.md)
- [test_zt_mapping_dependency](../../../functions/src/normalize/test_zt_mapping_dependency.md)
- [test_zt_mapping_misconfig](../../../functions/src/normalize/test_zt_mapping_misconfig.md)
- [test_zt_default_pillar](../../../functions/src/normalize/test_zt_default_pillar.md)
- [test_zt_mapping_no_duplicates](../../../functions/src/normalize/test_zt_mapping_no_duplicates.md)
- [test_scorecard_single_finding](../../../functions/src/normalize/test_scorecard_single_finding.md)
- [test_scorecard_multiple_findings_capped](../../../functions/src/normalize/test_scorecard_multiple_findings_capped.md)
- [test_scorecard_maturity_levels](../../../functions/src/normalize/test_scorecard_maturity_levels.md)
- [test_multiple_zt_pillars](../../../functions/src/normalize/test_multiple_zt_pillars.md)
- [test_dast_keyword_mappings_app_and_network](../../../functions/src/normalize/test_dast_keyword_mappings_app_and_network.md)
- [test_gap_analysis_with_secrets](../../../functions/src/normalize/test_gap_analysis_with_secrets.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
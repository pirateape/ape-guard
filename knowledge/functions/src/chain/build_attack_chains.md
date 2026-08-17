---
type: Rust Function
title: build_attack_chains
resource: src/chain.rs#L184-L298
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/chain/group_by_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/evaluate_rules_on_group
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/group_by_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/severity_to_score
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/group_by_zt_pillars
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/chain/test_credential_compromise_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_same_directory_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_single_finding_no_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_empty_findings_no_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_supply_chain_attack
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_multi_scanner_confirmation_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_infrastructure_escalation_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_risk_score_capped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_credential_chain_matches_by_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_credential_chain_matches_by_cwe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_no_chain_without_matching_attributes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/chain/test_zt_pillar_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_scan_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/handle_chains_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_attack_chains(findings: &[CanonicalFinding]) -> Vec<AttackChain>`

# Calls

- [group_by_file](../../../functions/src/chain/group_by_file.md)
- [evaluate_rules_on_group](../../../functions/src/chain/evaluate_rules_on_group.md)
- [group_by_directory](../../../functions/src/chain/group_by_directory.md)
- [severity_to_score](../../../functions/src/chain/severity_to_score.md)
- [group_by_zt_pillars](../../../functions/src/chain/group_by_zt_pillars.md)

# Called by

- [test_credential_compromise_chain](../../../functions/src/chain/test_credential_compromise_chain.md)
- [test_same_directory_chain](../../../functions/src/chain/test_same_directory_chain.md)
- [test_single_finding_no_chain](../../../functions/src/chain/test_single_finding_no_chain.md)
- [test_empty_findings_no_chain](../../../functions/src/chain/test_empty_findings_no_chain.md)
- [test_supply_chain_attack](../../../functions/src/chain/test_supply_chain_attack.md)
- [test_multi_scanner_confirmation_chain](../../../functions/src/chain/test_multi_scanner_confirmation_chain.md)
- [test_infrastructure_escalation_chain](../../../functions/src/chain/test_infrastructure_escalation_chain.md)
- [test_risk_score_capped](../../../functions/src/chain/test_risk_score_capped.md)
- [test_credential_chain_matches_by_tag](../../../functions/src/chain/test_credential_chain_matches_by_tag.md)
- [test_credential_chain_matches_by_cwe](../../../functions/src/chain/test_credential_chain_matches_by_cwe.md)
- [test_no_chain_without_matching_attributes](../../../functions/src/chain/test_no_chain_without_matching_attributes.md)
- [test_zt_pillar_chain](../../../functions/src/chain/test_zt_pillar_chain.md)
- [run_report](../../../functions/src/run_report.md)
- [handle_scan_tool](../../../functions/src/mcp/handle_scan_tool.md)
- [handle_chains_tool](../../../functions/src/mcp/handle_chains_tool.md)
- [run_scan](../../../functions/src/orchestrate/run_scan.md)
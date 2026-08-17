---
type: Rust Module
title: chain
resource: src/chain.rs#L1-L962
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ChainRule](../../classes/src/chain/ChainRule.md)
- [cwe_prefix](../../functions/src/chain/cwe_prefix.md)
- [has_any_tag](../../functions/src/chain/has_any_tag.md)
- [scanner_is](../../functions/src/chain/scanner_is.md)
- [severity_at_least](../../functions/src/chain/severity_at_least.md)
- [has_zt_pillar](../../functions/src/chain/has_zt_pillar.md)
- [keyword_fallback](../../functions/src/chain/keyword_fallback.md)
- [is_credential](../../functions/src/chain/is_credential.md)
- [is_injection](../../functions/src/chain/is_injection.md)
- [is_vulnerability](../../functions/src/chain/is_vulnerability.md)
- [is_misconfig](../../functions/src/chain/is_misconfig.md)
- [is_xss](../../functions/src/chain/is_xss.md)
- [group_by_file](../../functions/src/chain/group_by_file.md)
- [group_by_directory](../../functions/src/chain/group_by_directory.md)
- [build_attack_chains](../../functions/src/chain/build_attack_chains.md)
- [evaluate_rules_on_group](../../functions/src/chain/evaluate_rules_on_group.md)
- [severity_to_score](../../functions/src/chain/severity_to_score.md)
- [severity_tag](../../functions/src/chain/severity_tag.md)
- [compute_chain_risk](../../functions/src/chain/compute_chain_risk.md)
- [build_chain_steps](../../functions/src/chain/build_chain_steps.md)
- [generate_recommendation](../../functions/src/chain/generate_recommendation.md)
- [group_by_zt_pillars](../../functions/src/chain/group_by_zt_pillars.md)
- [make_finding](../../functions/src/chain/make_finding.md)
- [make_basic](../../functions/src/chain/make_basic.md)
- [test_credential_compromise_chain](../../functions/src/chain/test_credential_compromise_chain.md)
- [test_same_directory_chain](../../functions/src/chain/test_same_directory_chain.md)
- [test_single_finding_no_chain](../../functions/src/chain/test_single_finding_no_chain.md)
- [test_empty_findings_no_chain](../../functions/src/chain/test_empty_findings_no_chain.md)
- [test_supply_chain_attack](../../functions/src/chain/test_supply_chain_attack.md)
- [test_multi_scanner_confirmation_chain](../../functions/src/chain/test_multi_scanner_confirmation_chain.md)
- [test_infrastructure_escalation_chain](../../functions/src/chain/test_infrastructure_escalation_chain.md)
- [test_severity_to_score](../../functions/src/chain/test_severity_to_score.md)
- [test_risk_score_capped](../../functions/src/chain/test_risk_score_capped.md)
- [test_credential_chain_matches_by_tag](../../functions/src/chain/test_credential_chain_matches_by_tag.md)
- [test_credential_chain_matches_by_cwe](../../functions/src/chain/test_credential_chain_matches_by_cwe.md)
- [test_no_chain_without_matching_attributes](../../functions/src/chain/test_no_chain_without_matching_attributes.md)
- [test_zt_pillar_chain](../../functions/src/chain/test_zt_pillar_chain.md)

# Imports

- `crate::find::*`
- `std::collections::{HashMap, HashSet}`
- `super::*`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
---
type: Rust Module
title: normalize
resource: src/normalize.rs#L1-L704
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-gapanalysis-gaplevel-maturitytier-pillarscore-scannertype-severity-zerotrustscorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find
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

- [normalize_findings](../../functions/src/normalize/normalize_findings.md)
- [compute_zt_scorecard](../../functions/src/normalize/compute_zt_scorecard.md)
- [compute_gap_analysis](../../functions/src/normalize/compute_gap_analysis.md)
- [generate_pillar_recommendations](../../functions/src/normalize/generate_pillar_recommendations.md)
- [mitre_mapping](../../functions/src/normalize/mitre_mapping.md)
- [make_finding](../../functions/src/normalize/make_finding.md)
- [test_zt_mapping_secret](../../functions/src/normalize/test_zt_mapping_secret.md)
- [test_zt_mapping_injection](../../functions/src/normalize/test_zt_mapping_injection.md)
- [test_zt_mapping_xss](../../functions/src/normalize/test_zt_mapping_xss.md)
- [test_zt_mapping_dependency](../../functions/src/normalize/test_zt_mapping_dependency.md)
- [test_zt_mapping_misconfig](../../functions/src/normalize/test_zt_mapping_misconfig.md)
- [test_zt_default_pillar](../../functions/src/normalize/test_zt_default_pillar.md)
- [test_zt_mapping_no_duplicates](../../functions/src/normalize/test_zt_mapping_no_duplicates.md)
- [test_scorecard_no_findings](../../functions/src/normalize/test_scorecard_no_findings.md)
- [test_scorecard_single_finding](../../functions/src/normalize/test_scorecard_single_finding.md)
- [test_scorecard_multiple_findings_capped](../../functions/src/normalize/test_scorecard_multiple_findings_capped.md)
- [test_scorecard_maturity_levels](../../functions/src/normalize/test_scorecard_maturity_levels.md)
- [test_mitre_mapping_secret](../../functions/src/normalize/test_mitre_mapping_secret.md)
- [test_multiple_zt_pillars](../../functions/src/normalize/test_multiple_zt_pillars.md)
- [test_dast_keyword_mappings_app_and_network](../../functions/src/normalize/test_dast_keyword_mappings_app_and_network.md)
- [test_gap_analysis_no_findings](../../functions/src/normalize/test_gap_analysis_no_findings.md)
- [test_gap_analysis_with_secrets](../../functions/src/normalize/test_gap_analysis_with_secrets.md)
- [test_pillar_recommendations](../../functions/src/normalize/test_pillar_recommendations.md)
- [test_recommendations_missing_findings](../../functions/src/normalize/test_recommendations_missing_findings.md)

# Imports

- `crate::find::{
    CanonicalFinding, GapAnalysis, GapLevel, MaturityTier, PillarScore, ScannerType, Severity,
    ZeroTrustScorecard,
}`
- `std::collections::HashMap`
- `super::*`
- `crate::find::*`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
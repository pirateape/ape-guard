---
type: Rust Module
title: find
resource: src/find/mod.rs#L1-L442
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
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

- [CanonicalFinding](../../classes/src/find/CanonicalFinding.md)
- [FindingLocation](../../classes/src/find/FindingLocation.md)
- [CrossReference](../../classes/src/find/CrossReference.md)
- [RejectReason](../../classes/src/find/RejectReason.md)
- [GradeVerdict](../../classes/src/find/GradeVerdict.md)
- [RiskDimensions](../../classes/src/find/RiskDimensions.md)
- [UnifiedRiskScore](../../classes/src/find/UnifiedRiskScore.md)
- [ScannerType](../../classes/src/find/ScannerType.md)
- [fmt](../../functions/src/find/ScannerType/std-fmt-display/fmt.md)
- [Severity](../../classes/src/find/Severity.md)
- [Confidence](../../classes/src/find/Confidence.md)
- [ZeroTrustScorecard](../../classes/src/find/ZeroTrustScorecard.md)
- [GapAnalysis](../../classes/src/find/GapAnalysis.md)
- [GapLevel](../../classes/src/find/GapLevel.md)
- [PillarScore](../../classes/src/find/PillarScore.md)
- [MaturityTier](../../classes/src/find/MaturityTier.md)
- [AttackChain](../../classes/src/find/AttackChain.md)
- [ScanSummary](../../classes/src/find/ScanSummary.md)
- [FindingsBySeverity](../../classes/src/find/FindingsBySeverity.md)
- [test_severity_ordering](../../functions/src/find/test_severity_ordering.md)
- [test_canonical_finding_defaults](../../functions/src/find/test_canonical_finding_defaults.md)
- [test_attack_chain_creation](../../functions/src/find/test_attack_chain_creation.md)
- [test_zt_scorecard_defaults](../../functions/src/find/test_zt_scorecard_defaults.md)
- [test_gap_analysis_struct](../../functions/src/find/test_gap_analysis_struct.md)
- [test_findings_by_severity](../../functions/src/find/test_findings_by_severity.md)
- [test_maturity_tier_debug](../../functions/src/find/test_maturity_tier_debug.md)
- [test_cross_reference_default](../../functions/src/find/test_cross_reference_default.md)

# Imports

- `serde::{Deserialize, Serialize}`
- `std::path::PathBuf`
- `super::*`

# Member of

- [apeguard](../../packages/apeguard.md)
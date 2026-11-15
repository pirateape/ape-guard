---
id: apeguard-find
type: module-design
title: find/ — canonical finding schema & Zero Trust scorecard
status: draft
parent: apeguard-architecture
depends-on: []
references: []
implements: []
covers: []
tags: []
---

# find/ — canonical finding schema & Zero Trust scorecard

## Responsibility

`src/find/mod.rs` defines the canonical finding schema and all shared security types.
Every scanner's raw output is normalized into `CanonicalFinding` before any analysis,
deduplication, scoring, or reporting. It also defines the Unified Zero Trust Framework
(UZTF v2.0) types: `ZeroTrustScorecard`, `PillarScore`, `MaturityTier`,
`PostureClassification`, `GapAnalysis`, plus `Severity`, `Confidence`, `ScannerType`,
`AttackChain`, `ScanSummary`, `FindingsBySeverity`, `GradeVerdict`, `RejectReason`,
`UnifiedRiskScore`, `RiskDimensions`, and `CrossReference`.

This is the crate's dependency root: it imports nothing from the rest of apeguard.

## Boundary

- **Allowed deps:** `serde`, `std::path::PathBuf` only.
- **Forbidden reaches:** must not import any other apeguard module (cli, scanner,
  orchestrate, report, etc.). Kept dependency-free so every other module can depend on it.

## Key types

- `CanonicalFinding` — normalized finding with id, scanner, rule_id, severity, confidence,
  title, description, location, cwe, cvss, remediation, evidence, tags, zt_pillars,
  cross_refs, grade, risk_score, reachable.
- `ScannerType` — Gitleaks, Trufflehog, Semgrep, Trivy{_Vuln,_Secret,_Misconfig,_Container},
  Checkov, Syft, Nuclei, Zap, Architecture, ContextDrift, McpSecurity, TerraformIaC,
  AwsS3, TlsCertificate, Custom.
- `Severity` / `Confidence` — ordered enums (Info < Low < Medium < High < Critical).
- `ZeroTrustScorecard` — 12 UZTF pillars, per-pillar score (0–100), overall (0–1200),
  maturity tier per pillar, posture classification, gap analysis.
- `GradeVerdict` / `RejectReason` — adversarial AI grading outcome (Confirmed / Rejected /
  NeedsReview).

## Tests

Inline unit tests cover severity ordering, finding defaults, attack-chain creation,
scorecard defaults, gap analysis, and severity-summary arithmetic (all in
`src/find/mod.rs`).

## Open questions

- `ScannerType::Zap` is declared but no `zap.rs` scanner impl is wired into
  `orchestrate.rs` — confirm whether it's a stub or dead variant (P3/P4 marker).

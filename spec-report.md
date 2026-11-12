---
id: apeguard-report
type: module-design
title: report/ — report generation (tech / exec / roadmap)
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# report/ — report generation (tech / exec / roadmap)

## Responsibility

`src/report/mod.rs` generates the three audience reports from a `ScanSummary` + findings:
`technical.md` (engineers — every finding with location, severity, CWE/CVSS, ZT pillars,
MITRE ATT&CK, attack-chain cross-refs, remediation), `executive.md` (leadership — posture
summary, severity breakdown, ZT scorecard, top risks, trends), `roadmap.md` (engineering
managers — prioritized remediation by horizon Immediate/Short-term/Long-term, by ZT pillar
and severity). Output formats: Markdown, JSON, SARIF 2.1, HTML (self-contained dark theme
with severity badges, ZT scorecard cards, Mermaid architecture diagram). `ReportType`:
Technical / Executive / Roadmap.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ScanSummary, ZeroTrustScorecard, ...}`,
  `tera` (templates), `serde_json`, `std::path::{Path, PathBuf}`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `cli`, `main`.

## Notes

- `generate_all_reports` accepts optional `stride_result` and `policy_result` for report
  enrichment; report regeneration from cache passes `None` for those.
- HTML compare output is "not yet implemented" (`CompareFormat::Html` in cli.rs bails).

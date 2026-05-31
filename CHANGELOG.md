# Changelog

All notable changes to ApeGuard are documented here.

## [0.2.0] — 2026-05-31

### Added

- **Parallel scanner execution** — All 5 scanners (Gitleaks, Semgrep, Trivy-vuln, Trivy-secret, Trivy-misconfig) launch simultaneously via `futures::join_all`. ~2x faster on multi-finding targets. ([#11](https://github.com/pirateape/ape-guard/pull/11))

- **HTML report with interactive charts** — New `apeguard-report.html` output format featuring:
  - ZT Radar Chart (SVG) — 8-pillar hexagon overlay showing maturity at a glance
  - Severity Bar Chart — C/H/M/L/I distribution with color-coded bars
  - Pillar Progress Cards — score, maturity, gap count, and progress bar per pillar
  - Responsive dark theme, 0 external JS dependencies

- **Severity-weighted ZT Scorecard** — Replaced raw-count scoring with severity-weighted:
  - Critical=10, High=5, Medium=3, Low=1, Info=0
  - `gap_count = min(weight, 10)`, `score = max(0, 100 - gap_count × 10)`
  - Maturity tiers: 0→Adaptive, ≤3→Advanced, >3→Baseline
  - More honest reflection of security posture

- **`--ci` flag for CI/CD pipelines** — Machine-readable exit codes:
  - Exit 0: no findings above threshold
  - Exit 1: findings at or above High (auto-sets `--fail-on high`)
  - Clean `FAILED:` message (no emoji), works with `--quiet`
  - Compatible with GitHub Actions, GitLab CI, CircleCI

### Changed

- **Dockerfile** — Overhauled with multi-arch support (amd64/arm64), updated tool versions (Gitleaks 8.24.2, Trivy 0.70.0, Nuclei 3.8.0), PEP 668 venv fix for Semgrep
- **Report templates** — Fixed blank lines between table rows (Tera whitespace trimming)

### Fixed

- Removed leftover duplicate code in `normalize.rs` from `compute_gap_analysis` refactor
- Updated test assertions for new severity-weighted scoring model

## [0.1.0] — 2026-05-31

### Added

- **5-layer scanning pipeline**: Secrets (Gitleaks) → SAST (Semgrep) → SCA (Trivy vuln) → Container (Trivy image) → DAST (Nuclei)
- **Unified Zero Trust Framework mapping**: Every finding mapped to 8 pillars with maturity scoring (Baseline → Advanced → Adaptive)
- **Multi-audience reports**: Technical (engineers), Executive (leadership), Roadmap (EMs)
- **Multi-format output**: Markdown, JSON, SARIF, HTML
- **Attack chain analysis**: Cross-references findings across scanners
- **Architecture risk diagram**: Auto-generated Mermaid diagram of component risks
- **SQLite-backed scan cache** with TTL-based pruning
- **LLM remediation**: Ollama-powered remediation suggestions (local, no exfiltration)
- **MCP server**: Model Context Protocol server for AI agent integration
- **100% local**: No SaaS, no telemetry
- **Zero-cost toolchain**: Gitleaks (MIT) + Semgrep CE (LGPL-2.1) + Trivy (Apache 2.0) + Nuclei (MIT)

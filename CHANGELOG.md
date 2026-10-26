# Changelog

All notable changes to ApeGuard are documented here.

## [0.5.3] — 2026-10-26

### Added

- **UZTF v2.0 Zero Trust scorecard** — migrated from 8 pillars (0-800) to 12 pillars (0-1200) with severity-weighted scoring (Critical 20, High 15, Medium 10, Low 5, Info 1), gap analysis and posture classification (`MostlyAdaptive` / `MostlyAdvanced` / `MostlyBaseline` / `Initial`)
- **Expanded attack chain analysis** — broader cross-scanner correlation with 4 detection strategies
- **CI hardening** — DCO sign-off check (grandfathered commits authored before 2026-10-26), coverage job (cargo-tarpaulin), supply-chain security gate (`cargo deny` + `cargo machete`), reproducible-build verification, Dependabot docker ecosystem
- **OpenSSF readiness docs** — Best Practices badge criteria map, security self-audit checklist, dependency sweep report, project GOVERNANCE

### Changed

- `reqwest` pinned to 0.12 with `rustls-tls` only (default features dropped)
- Config merge now honors all `.apeguard.yaml` sections; ZT scorecard computed before severity filtering
- Docs refreshed for the 12-layer pipeline and UZTF v2.0 wording; CONTRIBUTING/SECURITY updates

### Security

- **MCP server** — scan/arch target path validation (flag injection, `..` traversal) and DAST web-URL validation (SSRF to loopback/private/link-local/CGNAT hosts); JSON-RPC line length bounded to 1 MiB
- **Policy engine** — canonicalized policy paths block symlink escapes outside `.apeguard/policies/`
- **LLM remediation** — prompt-injection guardrail; pipe-to-shell download rejection without false-positives on benign English ("curl the docs and share")
- **HTML report** — Content-Security-Policy meta header
- **CLI** — non-zero exit when the fail threshold is reached
- **Dependencies** — cleared RUSTSEC-2026-0381/0382/0386 (`webpki-roots` 1.0.11, `rustls-webpki` 0.103.16, `tokio` 1.54.1); deny unknown registries/sources

### Fixed

- Homebrew formula sha256 corrected for the v0.5.2 tarball; Checkov/Syft optional deps added
- `Cargo.toml` repository URL corrected to `pirateape/ape-guard`
- Duplicate `[0.5.0]` changelog heading removed

## [0.5.2] — 2026-07-22

### Fixed

- **MCP protocol version rejection** — `handle_initialize` was hard-rejecting protocol versions != "2025-03-26" with `-32603` error. Now accepts both "2025-11-25" and "2025-03-26", responding with the supported version per spec.
- **MCP scan tool truncated to layers 1-7** — Extended layer matching to cover all 12 layers including Trufflehog, Context Drift, MCP Security, Terraform, AWS S3, and TLS Certificate.
- **CLI scan never recorded to cache** — `run_scan()` returned a `cache` field in `ScanOutput` but `main.rs` discarded it. Added `cache.record_scan()` call after scan completion, fixing `apeguard report` always returning "No cached scans found".
- **Removed unused imports** in 4 scanner test modules; fixed clippy `needless_borrows_for_generic_args` in tls.rs.
- Build: 336 tests (324 unit + 12 int), 0 clippy warnings, 0 format issues.

## [0.5.1] — 2026-07-05

### Fixed

- **5 production issues from ALG audit** — Resolved all items from the Anchored Learning Graph review:
  - Replaced 5 `unwrap()` calls with descriptive `.expect()` in reachability, report, context_drift, and cache modules.
  - Converted 39 `#[allow]` attributes to `#[expect]` (Rust 2024 convention); reverted 15 unfulfilled expectations back to `#[allow]`.
  - Fixed 2 failing TruffleHog unit tests by stripping security-pragma comments before JSON parsing.
  - Added `///` doc-comments to ~27 public items across arch/graph, cache, scoring, and finding modules.
  - Optimized BFS hot loop in reachability analysis using `Rc<PathBuf>` to eliminate redundant path data cloning.
  - Build: 308 unit tests, 0 clippy warnings, 0 format issues across 16 files.

## [0.5.0] — 2026-06-23

### Added

- **Context Drift Detection (Layer 8)** — New internal scanner that verifies agent context files (AGENTS.md, CLAUDE.md, .cursor/rules) against the actual codebase state. Detects when documented dependencies, paths, architecture patterns, security practices, and conventions have drifted from reality.
  - 7 claim categories: Dependency, Path, Architecture, Convention, Security, Semantic, Command
  - 7 verification strategies: Dependency manifest check (Cargo.toml/package.json/pyproject.toml/go.mod), Path existence (glob), Architecture import scanning, Security tool presence, Convention heuristic, Command extraction, Semantic fallback
  - Three-layer dependency matching: (1) known technology keywords, (2) generic "use/depend on <name>" regex, (3) Unknown fallback
  - Enabled with `--context-drift` flag (off by default for projects without context files)
  - Findings flow through the standard pipeline: scan → normalize → dedup → LLM → GRADE → severity → chains → SCORE → ZT → report
  - Finding IDs: `CTX-XXXX`, rule IDs: `context-drift.{category}`
  - 26 tests covering parsers, verifiers, scanner integration, and edge cases

## [0.3.1] — 2026-06-01

### Added

- **Issue templates** — Bug report, feature request, and config.yml for structured community issue management.
- **Code of Conduct** — Contributor Covenant v2.1 with enforcement contact.
- **Contributing guide** — Quick-start for new contributors linking to issue templates and CoC.

### Fixed

- **`--no-color` flag now works** — Was parsed by the CLI but never wired to `tracing_subscriber`. Now disables ANSI colour in stderr output when set.
- **`Report` command respects `--reports` and `--format`** — Was silently generating all report types and only Markdown. Now filters by selected report types and generates JSON/SARIF/HTML output when requested.
- **MCP server honours configured binary paths** — Gitleaks and Semgrep layers in the MCP runtime were always using `::new()` (hardcoded binary name). Now use `::with_binary()` from config, matching Checkov/Syft pattern.
- **Safer test assertion** — Replaced bare `unwrap()` with `expect()` in `arch.rs` test for better failure messages.

### Removed

- **AUDIT.md** — Internal audit document. All 30 findings resolved; removed from repo to avoid confusing new contributors with stale statuses.

## [0.3.0] — 2026-05-31

### Added

- **Checkov (Layer 6) — IaC misconfiguration scanning** — Wraps `checkov --directory <target> --output json` for Terraform, Kubernetes, Helm, CloudFormation, and ARM template analysis. Maps findings to UZTF pillars (Infrastructure, Network, Data). Install: `pip install checkov` or `brew install checkov`.

- **Syft (Layer 7) — SBOM inventory** — Wraps `syft <target> -o json` to catalog dependencies (Rust crates, npm packages, Python wheels, Docker images, etc.). Reports Info-level findings per package with PURL evidence. Install: `brew install syft`.

- **All 7 layers now available** — `1=secrets, 2=SAST, 3=SCA, 4=container, 5=DAST, 6=IaC, 7=SBOM`. Use `--layers 1,2,3,4,5,6,7` to run the full suite.

### Changed

- **CLI help text** — Updated `--layers` description to include layers 6 (Checkov) and 7 (Syft).
- **Config defaults** — Added `checkov` and `syft` to `ScannerBinaries` struct for custom path support.
- **MCP tool schema** — Extended `scan` tool input schema to document new layers.

### Fixed

- **Gitleaks capture reliability** — Uses `--report-path` with UUID temp file to prevent stdout/stderr pollution and race conditions (D49).
- **HTML/MD/JSON/SARIF reports** — All formats now derive from the same `ZeroTrustScorecard` and `CanonicalFinding` data for consistency (D50).
- **Tera template whitespace** — Uses `{%-` (whitespace trimming) to prevent blank lines in Markdown tables (D51).
- **CI mode safety** — `--ci` flag implicitly upgrades `--fail-on` to `high` if not explicitly set (D52).

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

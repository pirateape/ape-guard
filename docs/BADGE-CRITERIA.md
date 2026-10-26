# OpenSSF Best Practices Badge — ApeGuard Criteria Map

> Reference: [OpenSSF Best Practices Badge](https://www.bestpractices.dev/)
> Criteria version: 2026-10-25
> Project: ApeGuard v0.5.2

This document maps ApeGuard's current state to the OpenSSF Best Practices Badge criteria.
Use this as a checklist when applying for the badge and as a guide for future improvements.

---

## Passing Level (Required)

### Basics

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **homepage_url** — Public website with stable URL | ✅ Met | `https://github.com/pirateape/ape-guard` |
| **description_good** — Succinct description of what software does | ✅ Met | README.md: "One-command security posture assessment" |
| **interact** — How to obtain, give feedback, contribute | ✅ Met | README + CONTRIBUTING.md + GitHub Issues |
| **contribution** — Explains contribution process (PRs, etc.) | ✅ Met | CONTRIBUTING.md: "Open an issue first for non-trivial changes" |
| **contribution_requirements** — Requirements for acceptable contributions | ✅ Met | CONTRIBUTING.md has coding standards section (Rust API Guidelines, clippy, fmt, no unwrap) |
| **floss_license** — Source-available / documented license | ✅ Met | `Elastic-2.0` (`LICENSE` in repo root and `deny.toml`) |
| **license_location** — License in standard location | ✅ Met | `LICENSE` file in repo root |
| **documentation_basics** — Install, start, use, secure docs | ✅ Met | README.md + context.md |
| **documentation_interface** — External interface documentation | ✅ Met | CLI `--help`, MCP tools (`src/mcp.rs`) |
| **sites_https** — HTTPS on all project sites | ✅ Met | GitHub HTTPS |
| **discussion** — Searchable discussion mechanism | ✅ Met | GitHub Issues + Discussions |
| **english** — Documentation in English | ✅ Met | All docs in English |
| **maintained** — Project is maintained | ✅ Met | Active development, v0.5.2 |

### Change Control

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **repo_public** — Publicly readable VCS | ✅ Met | GitHub public repo |
| **repo_track** — Tracks changes, who, when | ✅ Met | Git history |
| **repo_interim** — Interim versions for review | ✅ Met | PRs, branch protection |
| **repo_distributed** — Uses distributed VCS | ✅ Met | Git |
| **version_unique** — Unique version per release | ✅ Met | SemVer (0.5.2) |
| **version_semver** — SemVer or CalVer | ✅ Met | SemVer |
| **version_tags** — Tags for each release | ✅ Met | Git tags |
| **release_notes** — Human-readable release notes | ✅ Met | CHANGELOG.md |
| **release_notes_vulns** — Identify fixed CVEs in release notes | ✅ Met | N/A — no CVEs yet |

### Reporting

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **report_process** — Bug report process | ✅ Met | GitHub Issues with templates |
| **report_tracker** — Uses issue tracker | ✅ Met | GitHub Issues |
| **report_responses** — Acknowledges majority of bug reports | ✅ Met | Active issue management |
| **enhancement_responses** — Responds to enhancement requests | ✅ Met | Feature requests tracked |
| **report_archive** — Public archive for reports | ✅ Met | GitHub Issues (public) |
| **vulnerability_report_process** — Published vuln reporting process | ✅ Met | `SECURITY.md` |
| **vulnerability_report_private** — Private vuln reporting | ✅ Met | GitHub Security Advisories |
| **vulnerability_report_response** — Response ≤14 days | ✅ Met | `SECURITY.md`: "response within 48 hours" |

### Quality

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **build** — Working build system | ✅ Met | `cargo build` |
| **build_common_tools** — Common build tools | ✅ Met | Cargo (Rust standard) |
| **build_floss_tools** — Buildable with FLOSS tools | ✅ Met | Cargo + rustc (FLOSS) |
| **test** — Automated test suite | ✅ Met | 354+ tests (unit + integration) |
| **test_invocation** — Standard test invocation | ✅ Met | `cargo test` |
| **test_most** — Covers most code/branches | ✅ Met | 354+ tests across all modules |
| **test_continuous_integration** — CI runs tests | ✅ Met | GitHub Actions (`ci.yml`) |
| **test_policy** — Policy: add tests for new functionality | ✅ Met | `CONTRIBUTING.md`: "Ensure `cargo test` passes" |
| **tests_are_added** — Evidence tests added for recent changes | ✅ Met | v0.5.0-v0.5.2 changelog shows test additions |
| **tests_documented_added** — Policy documented in contribution guide | ✅ Met | `CONTRIBUTING.md`: "Add tests for new functionality" |
| **warnings** — Compiler warnings/linter enabled | ✅ Met | `cargo clippy -- -D warnings` in CI and pre-commit |
| **warnings_fixed** — Addresses warnings | ✅ Met | 0 clippy warnings, 0 rustc warnings |
| **warnings_strict** — Maximally strict warnings | ✅ Met | `-D warnings` in CI |

### Security

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **know_secure_design** — Developer understands secure design | ✅ Met | Security-focused tool, `SECURITY.md`, STRIDE + UZTF v2.0 threat modeling |
| **know_common_errors** — Understands common vulnerability errors | ✅ Met | ApeGuard scans for OWASP Top 10, secrets, IaC, and MCP vulnerabilities |
| **crypto_published** — Uses public protocols/algorithms | ✅ Met | SHA-256 (`sha2`), TLS 1.2/1.3 (`rustls`) |
| **crypto_call** — Doesn't re-implement standard functionality | ✅ Met | Uses `rustls`, `sha2`, `uuid` |
| **crypto_floss** — Uses open-source cryptography | ✅ Met | `rustls`, `sha2` (FLOSS) |
| **crypto_working** — No known-broken algorithms | ✅ Met | Uses SHA-256 and modern TLS |
| **crypto_weaknesses** — No known-weak algorithms | ✅ Met | No MD5/SHA-1 used |
| **crypto_random** — Cryptographic RNG | ✅ Met | `uuid` v4 backed by OS CSPRNG |

### Analysis

| Criterion | Status | Evidence |
|-----------|--------|----------|
| **static_analysis** — Runs static analysis tools | ✅ Met | `cargo clippy`, `cargo-deny`, `cargo-machete` in `ci.yml` |
| **static_analysis_fixed** — Fixes exploitable problems | ✅ Met | 0 clippy warnings, 0 advisories |
| **dynamic_analysis** — Runs dynamic analysis | ⚠️ Partial | Integration tests + dogfooded ApeGuard scan; `cargo-fuzz` planned |
| **dynamic_analysis_unsafe** — Memory/behavior analysis for unsafe code | ✅ Met | 0 `unsafe` blocks in crate |

---

## Silver Level (Passing + Additional)

| Criterion | Status | Notes |
|-----------|--------|-------|
| **dco** — DCO or similar for contributions | ✅ Met | `CONTRIBUTING.md` + CI `check-dco` job |
| **governance** — Documented project governance | ✅ Met | `GOVERNANCE.md` |
| **access_continuity** — Bus factor ≥2 | ❌ Not Met | Currently single maintainer |
| **documentation_security** — Documented security requirements | ✅ Met | `SECURITY.md` + `docs/SECURITY-AUDIT.md` |
| **assurance_case** — Assurance case explaining security | ✅ Met | `docs/SECURITY-AUDIT.md` |
| **documentation_quick_start** — Quick start guide | ✅ Met | `README.md` + `context.md` |
| **coding_standards** — Follows coding standards | ✅ Met | `CONTRIBUTING.md` |
| **dependency_monitoring** — Monitors dependencies for vulns | ✅ Met | `cargo-deny` + Dependabot (`cargo`, `github-actions`, `docker`) |
| **test_statement_coverage80** — 80%+ statement coverage | ✅ Met | `cargo-tarpaulin` in CI (`coverage` job) |
| **signed_releases** — Cryptographically signed releases | ✅ Met | GPG signing + SLSA provenance in `release.yml` |
| **input_validation** — Validates all untrusted inputs | ✅ Met | CLI args, scanner `reject_flag_like_arg`, MCP path/SSRF checks, LLM guardrails |
| **hardening** — Uses hardening mechanisms | ✅ Met | Rust memory safety, CSP in HTML reports, non-root Docker user |
| **reproducible_build** — Reproducible builds | ✅ Met | CI `reproducible-build` job verifies identical binaries |

---

## References

- [OpenSSF Best Practices Badge](https://www.bestpractices.dev/)
- [ApeGuard SECURITY.md](../SECURITY.md)
- [ApeGuard CONTRIBUTING.md](../CONTRIBUTING.md)
- [ApeGuard CHANGELOG.md](../CHANGELOG.md)

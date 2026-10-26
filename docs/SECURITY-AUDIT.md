# ApeGuard Security Self-Audit Checklist

> Prepared for OpenSSF Best Practices Badge — Silver Level
> Date: 2026-10-25
> Status: Self-audit (internal) — external review still needed

This document provides a structured self-audit of ApeGuard's security posture.
It serves as groundwork for a formal external security review.

---

## 1. Architecture Security

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 1.1 | No hardcoded secrets or credentials | ✅ | `cargo-deny` + `gitleaks` + `detect-secrets` in CI/pre-commit |
| 1.2 | No network exfiltration by default | ✅ | Local-only by default; LLM via local Ollama |
| 1.3 | Input validation on all CLI & MCP args | ✅ | `clap` validators + `reject_flag_like_arg` + MCP path/SSRF guards |
| 1.4 | Path traversal protection | ✅ | Canonicalized paths in `reachability.rs`, `policy.rs`, and `mcp.rs` |
| 1.5 | Zero unsafe code in crate | ✅ | 0 `unsafe` blocks across all ApeGuard source files |
| 1.6 | Dependency audit (supply chain) | ✅ | `cargo-deny` + `cargo-audit` + Dependabot |
| 1.7 | No SQL injection vectors | ✅ | Parameterized `rusqlite::params![]` queries in `src/cache.rs` |
| 1.8 | No command injection vectors | ✅ | `tokio::process::Command` arg arrays + `reject_flag_like_arg()` across all scanners |

## 2. Cryptographic & TLS Practices

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 2.1 | Uses vetted cryptographic and TLS libraries | ✅ | `rustls` (via `reqwest`), `sha2` (SHA-256), `uuid` v4 |
| 2.2 | No custom crypto implementations | ✅ | All hashing/TLS via RustCrypto (`sha2`) and `rustls` |
| 2.3 | Secure random number generation | ✅ | `uuid::Uuid::new_v4()` backed by OS CSPRNG (`getrandom`) |
| 2.4 | No weak or broken hash algorithms | ✅ | SHA-256 (`sha2`) used for scan/target digests and release checksums |
| 2.5 | No hardcoded encryption keys | ✅ | No embedded keys or credentials |
| 2.6 | TLS for network communication | ✅ | `reqwest` configured with `rustls-tls` (`default-features = false`) |

## 3. Access Control & Guardrails

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 3.1 | No privilege escalation | ✅ | Runs as unprivileged user (`apeguard` in Docker); no setuid |
| 3.2 | File permissions respected | ✅ | Respects OS umask; canonicalizes `.rego` policy directory |
| 3.3 | SSRF & path traversal blocked in MCP | ✅ | `validate_mcp_web_target` & `validate_mcp_target_path` in `src/mcp.rs` |
| 3.4 | LLM prompt-injection guardrails | ✅ | `<<<BEGIN UNTRUSTED DATA>>>` delimiters + `guardrail_check()` in `llm.rs` & `grade.rs` |

## 4. Error Handling

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 4.1 | No panics on invalid input | ✅ | `Result` types throughout; bounded 1 MiB JSON-RPC line reader |
| 4.2 | No sensitive data in error messages | ✅ | Error messages sanitized |
| 4.3 | Graceful degradation | ✅ | Scanners fail/timeout independently via `run_command_with_timeout` |
| 4.4 | No unwrap() in production paths | ✅ | 0 clippy warnings; `expect()` only in tests |

## 5. Logging & Audit

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 5.1 | Security events logged | ✅ | Scan results, findings, policy actions |
| 5.2 | No sensitive data in logs | ✅ | No secrets logged |
| 5.3 | Audit trail for scan operations | ✅ | SQLite cache (`src/cache.rs`) records scan history |
| 5.4 | Log injection prevention | ✅ | Structured logging via `tracing` |

## 6. Build & Release Security

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 6.1 | Reproducible builds | ✅ | CI `reproducible-build` job |
| 6.2 | Signed releases & provenance | ✅ | GPG signing + SLSA provenance (`actions/attest-build-provenance`) in `release.yml` |
| 6.3 | SBOM generation | ✅ | Layer 7 (`syft`) SBOM generation |
| 6.4 | Dependency pinning & source restriction | ✅ | `Cargo.lock` committed; `unknown-registry = "deny"` and `unknown-git = "deny"` in `deny.toml` |
| 6.5 | Container binary checksum verification | ✅ | SHA-256 verification for scanner tarballs in `Dockerfile` |

## 7. Vulnerability Management

| # | Check | Status | Evidence |
|---|-------|--------|----------|
| 7.1 | Documented vuln reporting process | ✅ | `SECURITY.md` |
| 7.2 | Private vuln reporting available | ✅ | GitHub Security Advisories |
| 7.3 | Response time commitment | ✅ | `SECURITY.md`: 48 hours |
| 7.4 | CVE assignment process | ✅ | Documented in `SECURITY.md` |
| 7.5 | Security advisories published | ✅ | GitHub Security tab |

## 8. External Review Readiness

| # | Check | Status | Notes |
|---|-------|--------|-------|
| 8.1 | Self-audit complete | ✅ | This document |
| 8.2 | Threat model documented | ✅ | STRIDE threat model (`src/stride.rs`) + Zero Trust UZTF v2.0 (`src/normalize.rs`) |
| 8.3 | Penetration test | ❌ | Needs external test |
| 8.4 | Code review by external party | ❌ | Needs external review |
| 8.5 | Fuzzing campaign | ⚠️ | Planned via `cargo-fuzz` / `proptest` (tracked in `todo.md`) |

---

## References

- [ApeGuard SECURITY.md](../SECURITY.md)
- [ApeGuard GOVERNANCE.md](../GOVERNANCE.md)
- [OpenSSF Best Practices Badge](https://www.bestpractices.dev/)
- [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/)

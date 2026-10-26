# Dependency sweep — ApeGuard (2026-10-26)

**Mode:** L1 report-only. No manifest, lockfile, or source file was modified.
**Repo:** `/Users/pirateape/Documents/src/projects/ApeGuard` @ `b015a3e` (branch `fix/deep-dive-improvements`)
**Manifest:** `Cargo.toml` (single crate, 21 deps + 3 dev-deps). Lockfile exists on disk (318 packages) but is **gitignored and untracked**.

## Method

| Check | Tool | Result |
|---|---|---|
| Outdated (semver-compatible) | `cargo update --dry-run` | 125 updates, 8 additions, 20 removals (133 locked) |
| Outdated (latest majors) | crates.io API vs `Cargo.lock` | 4 majors + 2 breaking 0.x bumps |
| Vulnerabilities | OSV batch query on all 318 locked packages | 8 advisories across 6 packages |
| Yanked | crates.io sparse index, all 318 packages | 0 yanked |
| Deprecations | RustSec unmaintained advisories + crate status | 1 unmaintained crate (`unsafe-libyaml`), 1 deprecated direct dep (`serde_yaml`) |
| Dependabot | `gh api .../dependabot/alerts` | `[]` — GitHub reports zero alerts (lockfile absent from default branch) |

## Security

| package | current | fixed in | advisory | severity / impact | breaking | effort |
|---|---|---|---|---|---|---|
| `rustls-webpki` | 0.103.15 | 0.103.16 | RUSTSEC-2026-0382 (CVE-2026-111943) | **High** — accepts certificate paths through distrusted emSign intermediates whose private keys are public. Directly weakens every `reqwest` TLS call (LLM API traffic). Transitive via `rustls` ← `hyper-rustls` ← `reqwest`. | no | 1 min |
| `webpki-roots` | 1.0.9 | 1.0.11 | RUSTSEC-2026-0381 (same CVE) | **High** — bundled root store trusts the distrusted emSign intermediates. Same fix family as above. | no | 1 min |
| `tokio` | 1.52.3 | 1.54.1 (backport 1.51.6) | RUSTSEC-2026-0386 | Medium — `AsyncFd::{new,with_interest,try_new}` accept any `AsRawFd` without ownership proof (I/O safety). **ApeGuard never uses `AsyncFd`** (grep: 0 hits), so not exploitable here; patch anyway. Direct dep. | no | 1 min |
| `lru` | 0.18.0 | 0.18.5 | RUSTSEC-2026-0253 | Low-Medium — `LruCache::pop()` not panic-safe → dangling pointers on next op. Reached via `regorus` policy eval only. Transitive. | no | 1 min |
| `quinn-proto` | 0.11.17 | 0.11.19 | RUSTSEC-2026-0338 / -0340 / -0343 | Low here — remote-triggerable panics (zero-length STREAM frames, ACK_FREQUENCY, datagram accounting). **Crate is unreachable**: `cargo tree -i quinn-proto --target all` prints nothing — orphaned lock entry. | no | 1 min |
| `unsafe-libyaml` | 0.2.11 | **none** | RUSTSEC-2024-0449 | Informational — unmaintained. Only removable by dropping `serde_yaml`. | yes (see blocked) | 30 min |

All five fixable advisories resolve with a lockfile refresh only — **no `Cargo.toml` change**.
The GitHub Dependabot alert feed is empty because `Cargo.lock` is not committed, so GitHub never sees the resolved graph (`git ls-files Cargo.lock` → not tracked; `.gitignore:3`).

## Maintenance

Direct deps, locked → latest:

| package | current | latest | type | files likely touched | risk | effort |
|---|---|---|---|---|---|---|
| `rusqlite` | 0.31.0 | 0.41.0 | major-equivalent (0.x, 10 minors) | `src/cache.rs` (only user) | Medium — API surface used is `Connection`, `params!`, `query_row/query_map/execute`; stable since 0.31. **MSRV 1.89** | 45–60 min |
| `tera` | 1.20.1 | 2.4.0 | major | `src/report/mod.rs` (2 `Tera::default()` + 2 inline `add_raw_template`) | **High** — v2 is a rewrite: undefined variables now error, `{{ x or 1 }}` semantics changed, `truncate` requires `length`, ISO-8601 offsets rejected. Runtime breakage in report rendering, not compile-time. | 2–4 h incl. template audit |
| `regorus` | 0.10.1 | 0.13.0 | 0.x minor (3) | `src/policy.rs` | Medium — 0.11 changed `Engine::add_data` to deep-merge (OPA semantics), 0.13 canonicalises package paths and caps import depth at 32. Core `Engine::new` / eval API unchanged. Behavioural, not structural. | 30–60 min |
| `reqwest` | 0.12.28 | 0.13.5 | 0.x minor (major-equivalent) | `Cargo.toml`, `src/llm.rs`, `src/grade.rs` | Low-Medium — 0.13 renames feature `rustls-tls` → `rustls`, `rustls-platform-verifier` becomes the default root source, and `query`/`form` become opt-in features. ApeGuard only uses `Client::builder` + `.json()`, so the diff is 1 feature line + 2 files untouched. **MSRV 1.85**. | 30 min |
| `sha2` | 0.10.9 | 0.11.1 | 0.x minor (breaking) | `src/main.rs`, `src/orchestrate.rs` (2 digest call sites) | Low — `Digest`/`Sha256::digest` unchanged for these uses. **MSRV 1.88**. | 15 min |
| `serde_yaml` | 0.9.34+deprecated | deprecated, no successor in-crate | blocked | `src/config.rs` | Deprecated upstream (last release 2024-03-25) and drags in unmaintained `unsafe-libyaml`. Needs a fork swap. | 30–45 min |
| `uuid` | 1.23.2 | 1.29.0 | minor | — | Low. **MSRV 1.89**. | in batch |
| `regex` | 1.12.3 | 1.13.1 | minor | — | Low. | in batch |
| `clap` / `clap_complete` / `serde` / `serde_json` / `futures` / `async-trait` / `chrono` / `glob` / `thiserror` / `anyhow` / `which` | see lock refresh | +patch | patch/minor | — | None (compiler-verified). | in batch |

Lock refresh shape (`cargo update --dry-run`): 125 updates, 8 adds, 20 removals. Only semver-breaking entry is transitive `synstructure 0.13.2 → 0.14.0` (proc-macro helper, no runtime surface). Removals are mostly `wasmparser` / `wit-*` / `wasm-*` and `hashbrown 0.15`, i.e. graph shrink. Additions: `base64 0.23.1`, `jsonschema-regex 0.46.10`, `psm`, `stacker`, `rand 0.10.3`.

Open Dependabot PRs (11 total, newest only reaches `regorus 0.11` / `rusqlite 0.40`, both now stale):

- CI actions, pinned SHAs, `CLEAN`: #39 attest-build-provenance 4.1.0→4.2.2, #38 rust-cache 2.9.1→2.9.2, #37 rust-toolchain SHA bump, #30 action-gh-release 3.0.0→3.0.2.
- CI actions, floating major tags, need a green run: #35 upload-artifact 4→7, #34 codeql-action 3→4, #33 setup-node 4→7.
- Cargo majors, `BEHIND`: #36 regorus 0.10.1→0.11.0 (superseded), #27 tera 1→2, #21 checkout 6→7, #19 rusqlite 0.31→0.40 (superseded).

Note: `.github/workflows/*` mixes SHA-pinned and floating-major actions (`actions/checkout@v4` and `@v6` both present, `codeql-action@v3` in `security-integration.yml`).

## Skipped / blocked

- **`Cargo.lock` is not in version control** (`.gitignore:3`, `git ls-files` fails). Builds are unpinned, GitHub Dependabot sees no Cargo graph (explains the empty alert feed), and no CVE fix can be delivered as a reviewable lock diff. Blocks batches 1–2 as *committed* changes. Human decision required.
- **`unsafe-libyaml` 0.2.11** — unmaintained, no fixed release. Only exits via the `serde_yaml` swap.
- **`serde_yaml` 0.9.34+deprecated** — no upstream fix. Candidates: `serde_norway 0.9.42` (drop-in fork, Rust 1.71) or `serde_yaml_ng 0.10.0` (Rust 1.64). Human choice required; API-compatible for `from_str`/`to_string` use in `src/config.rs`.
- **`tera` 2** — blocked on the v1→v2 template migration review (`MIGRATION.md`), not a mechanical bump.
- **MSRV drift** — `AGENTS.md` claims MSRV 1.75; `rusqlite 0.41` / `uuid 1.29` need 1.89, `sha2 0.11` needs 1.88, `reqwest 0.13` needs 1.85. CI uses `toolchain: stable`, so nothing breaks today, but the documented MSRV becomes false after batches 3–4. Either raise the claim or hold the bumps.
- **`quinn-proto`** — advisory-bearing but unreachable from the build graph; do not spend review effort, it vanishes on lock regeneration.

## Recommended plan

| # | Batch | Scope | Gate | Effort |
|---|---|---|---|---|
| 0 | Unblock | Commit `Cargo.lock`, add `.dependency-cache/` to `.gitignore` | Human approval (policy) | 5 min |
| 1 | **S1 security** | `cargo update -p tokio -p lru -p rustls-webpki -p webpki-roots -p quinn-proto` then test / clippy / fmt | none | 10 min |
| 2 | **P1 routine** | Remaining 125-package lock refresh; verify `synstructure 0.14` | CI green | 20 min |
| 3 | **M1 low-risk majors** | `reqwest 0.13.5` + `sha2 0.11.1` (manifest + feature rename) | compile + 1 live LLM call | 45 min |
| 4 | **M2 data layer** | `rusqlite 0.41.0` (`src/cache.rs`) | `cargo test` cache suite | 1 h |
| 5 | **M3 policy engine** | `regorus 0.13.0` (`src/policy.rs`) + Rego policy regression fixture | 1 h |
| 6 | **S2 deprecation** | `serde_yaml` → `serde_norway 0.9.42` (`src/config.rs`) | 45 min |
| 7 | **M4 report layer** | `tera 2.4.0` — last, highest risk | 2–4 h |
| 8 | **CI actions** | Merge #37/#38/#39/#30 as-is; land #33/#34/#35 on one branch with a full workflow run; close #36/#19 as superseded | 20 min |

Order rationale: every step is independently revertible; security first (batch 1 is 5 one-line lock bumps), template rewrite last because it is the only change that fails at runtime rather than at compile time.

## Evidence

```
cargo update --dry-run                 → 125 Updating / 8 Adding / 20 Removing
OSV querybatch (318 pkgs)              → 6 vulnerable packages, 8 advisories
crates.io sparse index (318 pkgs)      → 0 yanked
cargo tree -i quinn-proto --target all → nothing depends on it (orphan lock entry)
gh api .../dependabot/alerts           → []
git ls-files --error-unmatch Cargo.lock→ not tracked (.gitignore:3)
```

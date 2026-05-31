# ApeGuard Deep Audit Report
**Date:** 2026-05-30  
**Auditor:** Deep Research Agent (full codebase pass — all 18 source files)  
**Build status:** ✅ 0 warnings, 84 tests passing (before fixes)

---

## Severity Tiers

- 🔴 **CRITICAL** — Functional breakage / silent data loss
- 🟠 **HIGH** — Feature is claimed but not wired / silently ignored
- 🟡 **MEDIUM** — Code smell, inconsistency, or latent bug
- 🟢 **LOW / IMPROVEMENT** — Cleanup, performance, test coverage

---

## 🔴 CRITICAL: Silent Breakages

### C-1 — Trivy vuln parser reads wrong JSON key (zero CVEs ever parsed)
**File:** `src/scanner/trivy.rs::parse_vuln` (and `container.rs::parse_container_vuln`)  
**Bug:** The parser looks for `Results[].Packages[].Vulnerabilities` but actual Trivy `fs` JSON output
(v0.45+) is `Results[].Vulnerabilities` flat — no `Packages` wrapper exists in fs output.  
**Impact:** Every `trivy fs --scanners vuln` run returns **zero findings**. All CVE data is silently dropped.

```rust
// CURRENT (wrong): Results[].Packages[].Vulnerabilities
struct TrivyResult {
    target: String,
    #[serde(rename = "Packages")]
    packages: Option<Vec<TrivyPackage>>,  // ← this key doesn't exist in real output
}

// FIX: Results[].Vulnerabilities
struct TrivyResult {
    target: String,
    #[serde(rename = "Vulnerabilities")]
    vulnerabilities: Option<Vec<TrivyVuln>>,  // ← flat, direct
}
```
Same fix required in `container.rs`.

---

### C-2 — `generate_json_report` creates a dummy summary (empty scan_id, target, counts)
**File:** `src/report/mod.rs:328–341`  
**Bug:** `generate_json_report` doesn't accept a `ScanSummary` parameter. It creates a blank one internally with empty `scan_id`, empty `target`, and all counts as zero. The actual summary from `run_scan` is never passed through.

```rust
// report/mod.rs — called from main.rs:294 as:
report::generate_json_report(&final_findings, &zt_scorecard, &output_path)?

// But signature is:
pub fn generate_json_report(findings: &[CanonicalFinding], zt_scorecard: &ZeroTrustScorecard, output_dir: &Path)
// → creates dummy ScanSummary { scan_id: "", target: "", ... } internally
```
**Fix:** Add `summary: &ScanSummary` parameter and pass `&summary` from `run_scan`.

---

## 🟠 HIGH: Wired But Broken / Silently Ignored

### H-1 — LLM module built but never called
**File:** `src/llm.rs` — fully implemented  
**Caller:** Nowhere. `enhance_remediations()` is never invoked in `run_scan`, `run_report`, or `handle_scan_tool`.  
**Fix:** Add after normalization/dedup in `run_scan`:
```rust
let llm_cfg = crate::llm::LlmConfig::default();
let enhanced = crate::llm::enhance_remediations(&mut final_findings, &llm_cfg).await.unwrap_or(0);
if enhanced > 0 { tracing::info!("LLM enhanced {} remediations", enhanced); }
```
Also needs `LlmConfig` fields wired to config.yaml (endpoint, model, enabled).

---

### H-2 — `config.ScannerBinaries` parsed but never read by any scanner
**File:** `src/config.rs:31–35`, `src/scanner/gitleaks.rs:16`, `semgrep.rs:16`, `trivy.rs:24`  
**Bug:** Config supports custom binary paths (`gitleaks`, `semgrep`, `trivy`) but every scanner hardcodes its binary name as a string literal. The config values are merged in and stored but never consulted.  
**Fix:** Pass `cfg.binaries` into each scanner constructor. E.g.:
```rust
// In run_scan:
Gitleaks::with_binary(cfg.binaries.gitleaks.as_deref().unwrap_or("gitleaks"))
```

---

### H-3 — `--reports` CLI flag has zero effect
**File:** `src/cli.rs:59–60`, `src/main.rs:37`  
**Bug:** `Scan` and `Report` commands both accept `--reports tech,exec,roadmap` but `main.rs` destructures with `..` ignoring the `reports` field entirely. All three report types are always generated.  
**Fix:** Read `reports` in the match arm and pass to `generate_all_reports`, which should then filter by requested types.

---

### H-4 — `--quiet`, `--no-color`, `--log-level` global flags never applied
**File:** `src/cli.rs:15–24`, `src/main.rs:26–28`  
**Bug:**  
- `--log-level` is parsed but `tracing_subscriber` uses `EnvFilter::from_default_env()` (reads `RUST_LOG` env var). The CLI value is never applied.  
- `--quiet` flag: all `println!` in `run_scan`, `run_report`, etc. fire regardless.  
- `--no-color`: never consulted.

**Fix for log-level:**
```rust
tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&args.log_level))
    )
    .init();
```
**Fix for quiet:** Pass `args.quiet` into output functions and gate `println!` calls.

---

### H-5 — `config.cache.path`, `cache.ttl_hours` not propagated in `merge()`
**File:** `src/config.rs:149–154`  
**Bug:** `merge()` only updates `cache.enabled`. If a user sets `cache.path: /var/cache/apeguard` in their `.apeguard.yaml`, it's silently discarded:
```rust
if overlay.cache.enabled != base.cache.enabled {
    base.cache.enabled = overlay.cache.enabled;
}
// cache.path and cache.ttl_hours never merged!
```
**Fix:** Add:
```rust
if overlay.cache.path != PathBuf::from(".apeguard/cache") {
    base.cache.path = overlay.cache.path;
}
if overlay.cache.ttl_hours != 24 {
    base.cache.ttl_hours = overlay.cache.ttl_hours;
}
```

---

### H-6 — `handle_scan_tool` in MCP ignores layers 4 (container) and 5 (DAST)
**File:** `src/mcp.rs:250–261`  
**Bug:** The MCP scan tool only builds scanners for layers 1, 2, 3. `_ => {}` silently swallows layers 4 and 5. There's also no support for passing `--container` or `--web` arguments via MCP.

---

### H-7 — MCP `resources/read` method not implemented
**File:** `src/mcp.rs:68–85`  
**Bug:** `handle_list_tools` advertises two resources (`apeguard://reports/latest`, `apeguard://scorecard/latest`) but `handle_request` has no handler for `resources/read`. Any MCP client requesting these resources gets a "Method not found" error.

---

### H-8 — `run_report` always passes `None` for arch_diagram
**File:** `src/main.rs:431`  
**Bug:** When regenerating reports from cache, the arch diagram is always blank even if the project has architecture files:
```rust
let report_paths = report::generate_all_reports(&summary, &findings, &zt_scorecard, &output_path, None)?;
// Should re-run: crate::arch::discover_artifacts(&target_path) and pass diagram
```

---

## 🟡 MEDIUM: Logic Bugs & Inconsistencies

### M-1 — Dedup key is `(file, line)` — different CVEs at same location silently dropped
**File:** `src/dedup.rs:10–23`  
**Bug:** Two findings at the same file+line but with different `rule_id` (e.g., two CVEs in the same Dockerfile line) get deduplicated, keeping only the first one discovered. The key should include `rule_id`:
```rust
// Current key: (file, line)
// Fix: (file, line, rule_id) — only exact duplicates should be dropped
let key = (
    finding.location.file.to_string_lossy().to_string(),
    Some(line),
    finding.rule_id.clone(),
);
```

---

### M-2 — `gitleaks.rs::map_gitleaks_severity` duplicates and overrides `GITLEAKS_SEVERITY_MAP`
**File:** `src/scanner/gitleaks.rs:167–172`  
**Bug:** The scanner calls `map_gitleaks_severity()` which always returns `Severity::High`. Then `normalize_findings()` immediately overrides this with the full 65-rule `GITLEAKS_SEVERITY_MAP`. The first assignment is wasted. Also, the comment "In a real implementation, this would use a rule→severity mapping table" is stale — we have exactly that table in normalize.rs.  
**Fix:** Remove `map_gitleaks_severity()` entirely and use `Severity::High` directly (or `Severity::Medium` as a safer default since normalize.rs will override it anyway).

---

### M-3 — Typo in env var name: `APEGARD_OUTPUT_DIR` (missing 'U')
**File:** `src/config.rs:110`  
```rust
if let Ok(val) = std::env::var("APEGARD_OUTPUT_DIR") {   // ← bug: APEGARD not APEGUARD
```
**Fix:** Change to `"APEGUARD_OUTPUT_DIR"`.

---

### M-4 — `config.rs::generate_init` has duplicate arms in `if/else` branch
**File:** `src/config.rs:162–166`  
```rust
} else if target.is_dir() {
    target.join(".apeguard.yaml")   // ← same as next arm
} else {
    target.join(".apeguard.yaml")   // ← identical
}
```
Both arms produce the same result. This is harmless but misleading (likely a paste error).

---

### M-5 — `_severity` and `_arch_diagram` parameters use underscore but ARE used
**Files:** `src/main.rs:114` (`_severity`), `src/report/mod.rs:57` (`_arch_diagram`)  
Underscore prefix means "intentionally unused" in Rust, but both are actually read. This causes clippy `used-underscore-binding` warnings and is misleading.  
**Fix:** Remove the underscore prefix.

---

### M-6 — `run_command_with_timeout` helper unused — inline duplication in 4 scanners
**File:** `src/scanner/mod.rs:14–36`  
The helper exists but all four scanner `scan_raw()` implementations copy-paste their own inline `tokio::time::timeout(...)` logic. The helper function is dead.  
**Fix:** Refactor all scanner `scan_raw()` implementations to call `run_command_with_timeout()`.

---

### M-7 — `ScanCache::record_scan` timestamps: both `started_at` and `completed_at` = `now`
**File:** `src/cache.rs:309–316`  
Scan duration is tracked in the summary but never stored in the cache. Both timestamps are set to the current moment. Queries `ORDER BY started_at DESC` work, but historical duration data is lost.  
**Fix:** Pass `started_at` and `completed_at` as parameters to `record_scan`, or at minimum store duration separately.

---

### M-8 — `ScanCache::ttl_hours` parsed but never enforced
**File:** `src/cache.rs`, `src/config.rs:42`  
The `ttl_hours` config field exists, is merged, stored, but no code ever expires cache entries based on age. Stale data can persist indefinitely.  
**Fix:** In `ScanCache::open` or `record_scan`, prune entries older than `ttl_hours`.

---

### M-9 — ROADMAP template copy-paste error in "No results" text
**File:** `src/report/mod.rs:286`  
```tera
{% else %}
*No low-severity findings.*   ← appears in the "High" findings section
{% endfor %}
```
Should be `*No high-severity findings.*`.

---

### M-10 — MCP server and `load_cached_findings` use `Config::default()` ignoring user config
**File:** `src/mcp.rs:12–18, 240`  
Both `load_cached_findings()` and `handle_scan_tool()` construct `Config::default()` from scratch. Any `.apeguard.yaml` the user configured is ignored when using the MCP server.  
**Fix:** Either accept `Arc<Config>` or reload from filesystem; minimally pass the cache path as a parameter.

---

### M-11 — `ContainerScanner::scanner_type()` returns `TrivyVuln` (wrong type)
**File:** `src/scanner/container.rs:30`  
Container image scan findings are tagged as `ScannerType::TrivyVuln`, making them indistinguishable from filesystem vuln findings in reports and scorecard.  
**Fix:** Add `ScannerType::TrivyContainer` variant in `find/mod.rs` and return it.

---

### M-12 — Finding IDs collide across scans on the same day
**File:** `src/scanner/gitleaks.rs:130`, `semgrep.rs:142`, etc.  
IDs use `chrono::Utc::now().format("%Y%m%d")` + sequential index. Two scans on the same day produce identical IDs (e.g., both produce `AG-20260530-0001`). When stored in cache or compared across scans, these IDs clash.  
**Fix:** Include scan UUID prefix: `AG-{scan_id[..8]}-GL-{:04}`.

---

## 🟢 LOW / IMPROVEMENTS

### L-1 — Unused dependencies in Cargo.toml: `indicatif`, `termcolor`
Never imported anywhere. Remove to shrink binary and speed compilation.

### L-2 — `ScanCache` per-file caching table (`cache`) is fully dead code
`hash_file()`, `store()`, `get_cached()`, `get_all_cached_findings()` — all exist, none called. The original per-file incremental scan strategy was abandoned in favor of whole-scan snapshots. Either wire it or remove the table and these methods to eliminate confusion.

### L-3 — `arch.rs` has multiple unsafe `unwrap()` calls on regex captures
Lines 374–375, 427–428, 747 call `.unwrap()` on `caps.get(N)`. Malformed input could panic.  
**Fix:** Use `ok_or` / `?` pattern or skip the line.

### L-4 — `chain.rs::ScannerType::to_string` should be `impl Display`
The local `to_string()` method shadows naming conventions. Use `impl std::fmt::Display for ScannerType`.

### L-5 — `mcp.rs` MCP protocol version mismatch risk
`"protocolVersion": "2025-03-26"` — verify this matches the version your AI host expects. Some MCP clients enforce exact version matching.

### L-6 — `version` command missing nuclei and container tool checks
`print_version()` checks gitleaks, semgrep, trivy but not nuclei (layer 5 DAST). Should include all 5 tools.

### L-7 — `report/mod.rs::generate_report` parameter name `_arch_diagram` is misleading
The parameter IS used (line 83). Remove underscore prefix.

### L-8 — ZT mappings don't cover DAST-specific findings
Nuclei findings start with empty `zt_pillars` and always fall to `"applications"` default. Web-layer findings should map to `"applications"` (XSS, IDOR) and `"networks"` (misconfig, SSRF). Add DAST-specific keywords to `ZT_MAPPINGS`.

### L-9 — `run_scan` computes `arch_diagram` then only passes it to markdown reports
The arch diagram is computed on every scan but only wired into markdown (`.md`) reports via `generate_all_reports`. It's NOT included in JSON or SARIF outputs, even though the SARIF properties block would be a good fit.

### L-10 — `scan.layers` default in config is `[1, 2, 3]` but CLI default is `[1, 2, 3, 4]`
**File:** `src/config.rs:53` vs `src/cli.rs:35`  
The config default layers are `[1, 2, 3]` but the CLI default is `[1, 2, 3, 4]`. If a user runs with a config file but no `--layers` flag, they get a different default than with no config.

---

## Test Coverage Gaps

| Module | Unit Tests | Notes |
|---|---|---|
| `cache.rs` | ❌ None | Complex SQLite — 0 tests for `record_scan`, `stats`, `prune`, `get_latest_scan_findings` |
| `llm.rs` | ❌ None | No tests for prompt building or Ollama response parsing |
| `scanner/gitleaks.rs` | ❌ None | No `parse_output()` tests |
| `scanner/semgrep.rs` | ❌ None | No `parse_output()` tests |
| `scanner/trivy.rs` | ❌ None | No `parse_output()` tests (critical given C-1 above) |
| `scanner/container.rs` | ❌ None | No tests |
| `scanner/dast.rs` | ❌ None | No `parse_nuclei_json()` tests |
| `report/mod.rs` | ❌ None | No template rendering tests |
| `main.rs` (integration) | ✅ 12 tests | CLI integration coverage OK |

---

## Priority Fix Order

| Priority | Issue | Effort |
|---|---|---|
| 🔴 1 | **C-1: Trivy parser wrong key** → zero CVEs ever parsed | 10 min |
| 🔴 2 | **C-2: JSON report has dummy summary** | 5 min |
| 🟠 3 | **H-1: LLM enhance_remediations never called** | 15 min |
| 🟠 4 | **H-2: ScannerBinaries config ignored by scanners** | 20 min |
| 🟠 5 | **H-3: `--reports` flag has zero effect** | 20 min |
| 🟠 6 | **H-4: `--log-level` / `--quiet` never applied** | 15 min |
| 🟠 7 | **H-5: `cache.path` not merged from config** | 5 min |
| 🟡 8 | **M-1: Dedup key drops different CVEs at same line** | 10 min |
| 🟡 9 | **M-2: Dead `map_gitleaks_severity()` + double-remap** | 5 min |
| 🟡 10 | **M-3: Typo APEGARD → APEGUARD in env var** | 1 min |
| 🟡 11 | **M-6: `run_command_with_timeout` unused / duplication** | 30 min |
| 🟡 12 | **M-9: Roadmap template wrong "no findings" text** | 1 min |
| 🟢 13 | **L-1: Remove unused indicatif + termcolor deps** | 5 min |
| 🟢 14 | **L-2: Remove dead per-file cache methods** | 20 min |
| 🟢 15 | **Test coverage: cache.rs, scanner parsers, report** | 2–3h |

// ApeGuard CLI — Security Posture Assessment
// One command. Three reports. Zero Trust mapped.
//
// Architecture: docs/03-Projects/ApeGuard/ApeGuard_Architecture.md
// P3/P4 stubs and future features — applied per-module, not globally
// e.g. #[allow(dead_code)] on specific unused pub items in arch.rs, stride.rs
pub(crate) mod arch;
pub(crate) mod cache;
pub(crate) mod chain;
mod cli;
mod config;
pub(crate) mod dedup;
pub(crate) mod filter;
pub(crate) mod find;
pub(crate) mod grade;
pub(crate) mod mcp;
pub(crate) mod normalize;
pub(crate) mod reachability;
pub(crate) mod report;
pub(crate) mod scanner;
pub(crate) mod score;
pub(crate) mod stride;

pub(crate) mod orchestrate;

pub(crate) mod llm;
pub(crate) mod policy;

use chrono::Utc;
use sha2::Digest;

macro_rules! quiet_println {
    ($quiet:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        if !$quiet {
            println!($fmt $(, $arg)*);
        }
    };
}

/// Spawn a background task that cleans up child scanner processes on SIGINT/SIGTERM.
/// Uses `pkill -P` on Unix to kill all child processes before exiting.
fn install_signal_handler() {
    tokio::spawn(async {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut term =
                signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = term.recv() => {},
            }
        }
        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c().await.ok();
        }

        cleanup_child_processes();
        std::process::exit(130);
    });
}

/// Kill child scanner processes by parent PID using `pkill -P`.
#[cfg(unix)]
fn cleanup_child_processes() {
    let ppid = std::process::id();
    let _ = std::process::Command::new("pkill")
        .args(["-P", &ppid.to_string()])
        .output();
}

#[cfg(not(unix))]
fn cleanup_child_processes() {
    // On non-Unix (Windows), tokio drops Command handles which kills children
}
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI args first so --log-level is available for tracing init
    let args = cli::parse();

    // Initialize tracing — honour --log-level and --no-color flags
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&args.log_level)),
        )
        .with_writer(std::io::stderr) // keep tracing on stderr so stdout stays clean
        .with_ansi(!args.no_color)
        .init();

    // Install signal handler for graceful child process cleanup on Ctrl+C / SIGTERM
    install_signal_handler();

    // Load config (merge defaults + file + env + flags)
    let cfg = config::load(&args)?;

    match &args.command {
        cli::Command::Scan {
            path,
            layers,
            severity,
            output_dir,
            no_cache,
            format,
            web,
            container,
            fail_on,
            reports,
            resume,
            grade,
            context_drift,
            stride,
            policy,
            policy_dir,
            ..
        } => {
            let target = path.clone().unwrap_or_else(|| ".".to_string());
            // CI mode: auto-upgrade fail_on to high (unless explicitly set)
            let effective_fail_on = if args.ci && matches!(fail_on, cli::FailOnThreshold::Never) {
                &cli::FailOnThreshold::High
            } else {
                fail_on
            };
            let scan_args = orchestrate::ScanArgs {
                target: &target,
                layers,
                severity,
                fail_on: effective_fail_on,
                output_dir,
                no_cache: *no_cache,
                quiet: args.quiet,
                ci: args.ci,
                formats: format.clone(),
                web_target: web.clone(),
                containers: container.clone(),
                report_types: reports.clone(),
                resume: *resume,
                grade: *grade,
                context_drift: *context_drift,
                stride: *stride,
                policy: *policy,
                policy_dir: policy_dir.clone(),
            };
            let scan_output = orchestrate::run_scan(scan_args, &cfg).await?;
            if cfg.cache.enabled {
                let scan_id = format!("scan_{}", Utc::now().format("%Y%m%d_%H%M%S"));
                let now = Utc::now().to_rfc3339();
                if let Err(e) = scan_output.cache.record_scan(cache::RecordScanInput {
                    scan_id: &scan_id,
                    target: &target,
                    started_at: &scan_output.started_at,
                    completed_at: &now,
                    total_findings: scan_output.findings.len() as u32,
                    scanners_used: &scan_output.scanners_used,
                    findings: &scan_output.findings,
                }) {
                    tracing::warn!("Failed to record scan in cache: {}", e);
                }
            }
        }
        cli::Command::Report {
            path,
            snapshot,
            output_dir,
            format,
            reports,
        } => {
            let target = path.clone().unwrap_or_else(|| ".".to_string());
            run_report(
                &target,
                snapshot.as_deref(),
                output_dir,
                &cfg,
                args.quiet,
                reports,
                format,
            )
            .await?;
        }
        cli::Command::Compare { a, b, format } => {
            run_compare(a, b, format, &cfg, args.quiet).await?;
        }
        cli::Command::Init { path, template } => {
            let t = template.clone();
            config::generate_init(path.clone(), t)?;
        }
        cli::Command::Config { subcommand } => {
            handle_config(subcommand, &cfg)?;
        }
        cli::Command::Version => {
            print_version(args.quiet).await?;
        }
        cli::Command::Completions { shell } => {
            cli::generate_completions(*shell);
        }
        cli::Command::Serve => {
            mcp::serve().await?;
        }
        cli::Command::Cache { subcommand } => {
            handle_cache(subcommand, &cfg, args.quiet).await?;
        }
    }

    Ok(())
}

/// Regenerate reports from cached scan
async fn run_report(
    target: &str,
    snapshot: Option<&str>,
    output_dir: &str,
    cfg: &config::Config,
    quiet: bool,
    selected_reports: &[cli::ReportType],
    formats: &[cli::OutputFormat],
) -> anyhow::Result<()> {
    tracing::info!("Report regeneration from cache: snapshot={:?}", snapshot);

    let output_path = PathBuf::from(output_dir);
    std::fs::create_dir_all(&output_path)?;

    // Open cache
    let cache = if cfg.cache.enabled {
        let cache = cache::ScanCache::open(&cfg.cache.path)?;
        let _ = cache.enforce_ttl(cfg.cache.ttl_hours);
        cache
    } else {
        anyhow::bail!("Cache is disabled. Cannot regenerate reports without cache.");
    };

    // Load findings from cache
    let (scan_id, findings) = if let Some(snap) = snapshot {
        if snap == "latest" || snap == "last" {
            // Load latest scan
            let (sid, findings) = cache.get_latest_scan_findings()?.ok_or_else(|| {
                anyhow::anyhow!("No cached scans found. Run `apeguard scan` first.")
            })?;
            (sid, findings)
        } else {
            // Load specific snapshot by ID
            let findings = cache
                .get_scan_findings(snap)?
                .ok_or_else(|| anyhow::anyhow!("Scan snapshot '{}' not found in cache", snap))?;
            (snap.to_string(), findings)
        }
    } else {
        // Load latest scan
        let (sid, findings) = cache
            .get_latest_scan_findings()?
            .ok_or_else(|| anyhow::anyhow!("No cached scans found. Run `apeguard scan` first."))?;
        (sid, findings)
    };

    // Get scan record metadata
    let scan_record = cache
        .get_latest_scan_record()?
        .ok_or_else(|| anyhow::anyhow!("No scan records found in cache."))?;

    // Rebuild scan summary
    let mut by_sev = find::FindingsBySeverity {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        info: 0,
    };
    for f in &findings {
        match f.severity {
            find::Severity::Critical => by_sev.critical += 1,
            find::Severity::High => by_sev.high += 1,
            find::Severity::Medium => by_sev.medium += 1,
            find::Severity::Low => by_sev.low += 1,
            find::Severity::Info => by_sev.info += 1,
        }
    }

    // Save severity counts before moving by_sev into summary
    let (c_sev, h_sev, m_sev, l_sev, i_sev) = (
        by_sev.critical,
        by_sev.high,
        by_sev.medium,
        by_sev.low,
        by_sev.info,
    );

    // Recompute scores and chains
    let zt_scorecard = normalize::compute_zt_scorecard(&findings);
    let attack_chains = chain::build_attack_chains(&findings);

    let summary = find::ScanSummary {
        scan_id: scan_id.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        target: target.to_string(),
        target_hash: format!("{:x}", sha2::Sha256::digest(target.as_bytes())),
        duration_seconds: 0.0, // Unknown from cache
        total_findings: findings.len() as u32,
        findings_by_severity: by_sev,
        scanners_used: scan_record
            .scanners_used
            .split(',')
            .map(|s| s.to_string())
            .collect(),
        zt_scorecard: Some(zt_scorecard.clone()),
        attack_chains,
    };

    // Rebuild architecture diagram for regenerated reports (if artifacts exist)
    let target_path = PathBuf::from(target);
    let arch_artifacts = crate::arch::discover_artifacts(&target_path);
    let arch_diagram = if !arch_artifacts.is_empty() {
        let risks = crate::arch::assess_component_risks(&findings, &arch_artifacts);
        if risks.is_empty() {
            None
        } else {
            Some(crate::arch::generate_mermaid_diagram(
                &arch_artifacts,
                &risks,
            ))
        }
    } else {
        None
    };

    // Convert CLI report-type flags to report module enum
    let selected_report_types: Vec<report::ReportType> = selected_reports
        .iter()
        .map(|r| match r {
            cli::ReportType::Tech => report::ReportType::Technical,
            cli::ReportType::Exec => report::ReportType::Executive,
            cli::ReportType::Roadmap => report::ReportType::Roadmap,
        })
        .collect();

    // Generate reports
    let mut report_paths = report::generate_all_reports(
        &summary,
        &findings,
        &zt_scorecard,
        &output_path,
        arch_diagram.as_deref(),
        &selected_report_types,
        None, // stride_result not available during report regeneration
        None, // policy_result not available during report regeneration
    )?;

    // Generate additional output formats
    for fmt in formats {
        let path = match fmt {
            cli::OutputFormat::Md => continue, // Already generated
            cli::OutputFormat::Json => report::generate_json_report(
                &summary,
                &findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
                None,
                None,
            )?,
            cli::OutputFormat::Sarif => report::generate_sarif_report(
                &summary,
                &findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
                None,
                None,
            )?,
            cli::OutputFormat::Html => report::generate_html_report(
                &summary,
                &findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
                None,
                None,
            )?,
            cli::OutputFormat::Pdf => {
                // PDF not yet implemented, skip
                tracing::warn!("PDF output format not yet implemented");
                continue;
            }
        };
        report_paths.push(path);
    }

    quiet_println!(quiet, "");
    quiet_println!(quiet, "═══ ApeGuard Report Regeneration ═══");
    quiet_println!(quiet, "  Scan ID: {}", scan_id);
    quiet_println!(quiet, "  Target:  {}", target);
    quiet_println!(
        quiet,
        "  Findings: {} (C:{}, H:{}, M:{}, L:{}, I:{})",
        findings.len(),
        c_sev,
        h_sev,
        m_sev,
        l_sev,
        i_sev
    );
    quiet_println!(quiet, "  Reports:");
    for p in &report_paths {
        quiet_println!(quiet, "    📋 {}", p.display());
    }
    quiet_println!(quiet, "");

    Ok(())
}

/// Compare two scan snapshots by their scan IDs
async fn run_compare(
    a: &str,
    b: &str,
    format: &cli::CompareFormat,
    cfg: &config::Config,
    quiet: bool,
) -> anyhow::Result<()> {
    tracing::info!("Scan comparison: {} vs {} ({:?})", a, b, format);

    // Open cache
    let cache = if cfg.cache.enabled {
        let cache = cache::ScanCache::open(&cfg.cache.path)?;
        let _ = cache.enforce_ttl(cfg.cache.ttl_hours);
        cache
    } else {
        anyhow::bail!("Cache is disabled. Cannot compare scans without cache.");
    };

    // Load both scan findings
    let findings_a = cache.get_scan_findings(a)?.ok_or_else(|| {
        anyhow::anyhow!(
            "Scan '{}' not found in cache. Run `apeguard scan` first.",
            a
        )
    })?;

    let findings_b = cache.get_scan_findings(b)?.ok_or_else(|| {
        anyhow::anyhow!(
            "Scan '{}' not found in cache. Run `apeguard scan` first.",
            b
        )
    })?;

    // Build lookup maps for comparison
    use std::collections::HashMap;

    let map_a: HashMap<&str, &find::CanonicalFinding> =
        findings_a.iter().map(|f| (f.id.as_str(), f)).collect();
    let map_b: HashMap<&str, &find::CanonicalFinding> =
        findings_b.iter().map(|f| (f.id.as_str(), f)).collect();

    // Find added, removed, and changed findings
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut severity_changed = Vec::new();

    for f in &findings_b {
        match map_a.get(f.id.as_str()) {
            None => added.push(f),
            Some(old) if old.severity != f.severity => severity_changed.push((old, f)),
            Some(_) => {} // Unchanged
        }
    }

    for f in &findings_a {
        if !map_b.contains_key(f.id.as_str()) {
            removed.push(f);
        }
    }

    // Count by severity for summary
    fn count_by_sev(findings: &[&find::CanonicalFinding]) -> find::FindingsBySeverity {
        let mut counts = find::FindingsBySeverity {
            critical: 0,
            high: 0,
            medium: 0,
            low: 0,
            info: 0,
        };
        for f in findings {
            match f.severity {
                find::Severity::Critical => counts.critical += 1,
                find::Severity::High => counts.high += 1,
                find::Severity::Medium => counts.medium += 1,
                find::Severity::Low => counts.low += 1,
                find::Severity::Info => counts.info += 1,
            }
        }
        counts
    }

    let added_counts = count_by_sev(&added.to_vec());
    let removed_counts = count_by_sev(&removed.to_vec());

    match format {
        cli::CompareFormat::Text => {
            quiet_println!(quiet, "");
            quiet_println!(quiet, "═══ ApeGuard Scan Comparison ═══");
            quiet_println!(quiet, "  A: {} ({} findings)", a, findings_a.len());
            quiet_println!(quiet, "  B: {} ({} findings)", b, findings_b.len());
            quiet_println!(quiet, "");

            quiet_println!(quiet, "── Summary ──");
            quiet_println!(
                quiet,
                "  Added:   {} (C:{}, H:{}, M:{}, L:{}, I:{})",
                added.len(),
                added_counts.critical,
                added_counts.high,
                added_counts.medium,
                added_counts.low,
                added_counts.info
            );
            quiet_println!(
                quiet,
                "  Removed: {} (C:{}, H:{}, M:{}, L:{}, I:{})",
                removed.len(),
                removed_counts.critical,
                removed_counts.high,
                removed_counts.medium,
                removed_counts.low,
                removed_counts.info
            );
            quiet_println!(quiet, "  Severity changed: {}", severity_changed.len());
            quiet_println!(quiet, "");

            if !added.is_empty() {
                quiet_println!(quiet, "── New Findings (in B but not A) ──");
                for f in &added {
                    quiet_println!(
                        quiet,
                        "  🔴 [{:?}] {} — {}:{}",
                        f.severity,
                        f.title,
                        f.location.file.display(),
                        f.location.line.map_or("-".to_string(), |l| l.to_string())
                    );
                }
                quiet_println!(quiet, "");
            }

            if !removed.is_empty() {
                quiet_println!(quiet, "── Removed Findings (in A but not B) ──");
                for f in &removed {
                    quiet_println!(
                        quiet,
                        "  🟢 [{:?}] {} — {}:{}",
                        f.severity,
                        f.title,
                        f.location.file.display(),
                        f.location.line.map_or("-".to_string(), |l| l.to_string())
                    );
                }
                quiet_println!(quiet, "");
            }

            if !severity_changed.is_empty() {
                quiet_println!(quiet, "── Severity Changes ──");
                for (old, new) in &severity_changed {
                    quiet_println!(
                        quiet,
                        "  ⚠ {:?} → {:?}: {} — {}:{}",
                        old.severity,
                        new.severity,
                        new.title,
                        new.location.file.display(),
                        new.location.line.map_or("-".to_string(), |l| l.to_string())
                    );
                }
                quiet_println!(quiet, "");
            }

            if added.is_empty() && removed.is_empty() && severity_changed.is_empty() {
                quiet_println!(quiet, "  No differences found between the two scans.");
                quiet_println!(quiet, "");
            }
        }
        cli::CompareFormat::Json => {
            #[derive(serde::Serialize)]
            struct CompareResult<'a> {
                scan_a: &'a str,
                scan_b: &'a str,
                findings_a: usize,
                findings_b: usize,
                added: usize,
                removed: usize,
                severity_changed: usize,
                // For JSON, include full finding details of diff
                new_findings: Vec<&'a find::CanonicalFinding>,
                removed_findings: Vec<&'a find::CanonicalFinding>,
            }

            let result = CompareResult {
                scan_a: a,
                scan_b: b,
                findings_a: findings_a.len(),
                findings_b: findings_b.len(),
                added: added.len(),
                removed: removed.len(),
                severity_changed: severity_changed.len(),
                new_findings: added,
                removed_findings: removed,
            };

            quiet_println!(quiet, "{}", serde_json::to_string_pretty(&result)?);
        }
        cli::CompareFormat::Html => {
            anyhow::bail!(
                "HTML comparison output is not yet implemented. Use --format text or json."
            );
        }
    }

    Ok(())
}

/// Handle config subcommand
fn handle_config(
    subcommand: &Option<cli::ConfigSubcommand>,
    cfg: &config::Config,
) -> anyhow::Result<()> {
    match subcommand {
        Some(cli::ConfigSubcommand::Validate) => {
            println!("Config valid: {:?}", cfg);
            Ok(())
        }
        Some(cli::ConfigSubcommand::Paths) => {
            println!("Config search paths:");
            println!("  1. .apeguard.yaml (current dir)");
            println!("  2. .apeguard/config.yaml");
            println!("  3. apeguard.yaml");
            println!("  4. APEGUARD_* environment variables");
            Ok(())
        }
        None => {
            println!("Config: {:?}", cfg);
            Ok(())
        }
    }
}

/// Handle `apeguard cache` commands
async fn handle_cache(
    subcommand: &cli::CacheSubcommand,
    cfg: &config::Config,
    quiet: bool,
) -> anyhow::Result<()> {
    let cache = cache::ScanCache::open(&cfg.cache.path)?;
    let _ = cache.enforce_ttl(cfg.cache.ttl_hours);

    match subcommand {
        cli::CacheSubcommand::Stats => {
            let stats = cache.stats()?;
            quiet_println!(quiet, "═══ ApeGuard Cache Statistics ═══");
            quiet_println!(
                quiet,
                "  Enabled:          {}",
                if stats.enabled { "yes" } else { "no" }
            );
            quiet_println!(quiet, "  Scans recorded:   {}", stats.scan_count);
            quiet_println!(quiet, "  Total findings:    {}", stats.total_findings);
            quiet_println!(
                quiet,
                "  Database size:    {} bytes",
                stats.database_size_bytes
            );
            quiet_println!(quiet, "");
            if stats.enabled && stats.scan_count == 0 {
                quiet_println!(
                    quiet,
                    "  No data yet. Run `apeguard scan` to populate the cache."
                );
            }
        }
        cli::CacheSubcommand::Prune => {
            let removed = cache.prune(10)?;
            quiet_println!(quiet, "═══ ApeGuard Cache Prune ═══");
            quiet_println!(quiet, "  Removed scan records: {}", removed);
            let stats = cache.stats()?;
            quiet_println!(
                quiet,
                "  Database size now:   {} bytes",
                stats.database_size_bytes
            );
        }
    }

    Ok(())
}

/// Print version and dependency status
async fn print_version(quiet: bool) -> anyhow::Result<()> {
    quiet_println!(quiet, "ApeGuard v{}", env!("CARGO_PKG_VERSION"));
    quiet_println!(quiet, "License: {}", env!("CARGO_PKG_LICENSE"));
    quiet_println!(quiet, "");

    // Check if each scanner is available
    let scanners: [(&str, &str, &[&str]); 6] = [
        ("Gitleaks", "gitleaks", &["version"]),
        ("Semgrep", "semgrep", &["--version"]),
        ("Trivy", "trivy", &["--version"]),
        ("Nuclei", "nuclei", &["-version"]),
        ("Checkov", "checkov", &["--version"]),
        ("Syft", "syft", &["--version"]),
    ];

    for (name, binary, args) in &scanners {
        let status = tokio::process::Command::new(binary)
            .args(*args)
            .output()
            .await;

        match status {
            Ok(output) if output.status.success() => {
                let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                quiet_println!(
                    quiet,
                    "  ✅ {}: {}",
                    name,
                    ver.lines().next().unwrap_or("installed")
                );
            }
            _ => {
                quiet_println!(quiet, "  ❌ {}: not found", name);
            }
        }
    }

    Ok(())
}

// ApeGuard CLI — Security Posture Assessment
// One command. Three reports. Zero Trust mapped.
//
// Architecture: docs/03-Projects/ApeGuard/ApeGuard_Architecture.md
#![allow(dead_code)] // P3/P4 stubs and future features
pub(crate) mod arch;
pub(crate) mod cache;
pub(crate) mod chain;
mod cli;
mod config;
pub(crate) mod dedup;
pub(crate) mod find;
pub(crate) mod mcp;
pub(crate) mod normalize;
pub(crate) mod report;
pub(crate) mod scanner;

pub(crate) mod llm;

use sha2::Digest;
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
            ..
        } => {
            let target = path.clone().unwrap_or_else(|| ".".to_string());
            // CI mode: auto-upgrade fail_on to high (unless explicitly set)
            let effective_fail_on = if args.ci && matches!(fail_on, cli::FailOnThreshold::Never) {
                &cli::FailOnThreshold::High
            } else {
                fail_on
            };
            let scan_args = ScanArgs {
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
            };
            run_scan(scan_args, &cfg).await?;
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

/// Filter findings by minimum severity threshold
fn filter_by_severity(
    findings: Vec<find::CanonicalFinding>,
    filter: &cli::SeverityFilter,
) -> Vec<find::CanonicalFinding> {
    use find::Severity;

    let min_severity = match filter {
        cli::SeverityFilter::All => return findings, // No filtering
        cli::SeverityFilter::Info => Severity::Info,
        cli::SeverityFilter::Low => Severity::Low,
        cli::SeverityFilter::Medium => Severity::Medium,
        cli::SeverityFilter::High => Severity::High,
        cli::SeverityFilter::Critical => Severity::Critical,
    };

    let before = findings.len();
    let filtered: Vec<_> = findings
        .into_iter()
        .filter(|f| f.severity >= min_severity)
        .collect();
    let after = filtered.len();
    let removed = before - after;

    if removed > 0 {
        tracing::info!(
            "Severity filter ({:?}): removed {} findings below threshold (kept {})",
            filter,
            removed,
            after
        );
    }

    filtered
}

/// Arguments for a scan operation, grouped to avoid too-many-arguments lint
struct ScanArgs<'a> {
    target: &'a str,
    layers: &'a [u8],
    severity: &'a cli::SeverityFilter,
    fail_on: &'a cli::FailOnThreshold,
    output_dir: &'a str,
    no_cache: bool,
    quiet: bool,
    ci: bool,
    formats: Vec<cli::OutputFormat>,
    web_target: Option<String>,
    containers: Vec<String>,
    report_types: Vec<cli::ReportType>,
}

/// Format and print to stdout only if not in quiet mode
macro_rules! quiet_println {
    ($quiet:expr, $fmt:expr $(, $arg:expr)* $(,)? ) => {
        if !$quiet {
            println!($fmt $(, $arg)*);
        }
    };
}

/// Run a full security scan pipeline
async fn run_scan(args: ScanArgs<'_>, cfg: &config::Config) -> anyhow::Result<()> {
    let target = args.target;
    let layers = args.layers;
    let severity_filter = args.severity;
    let fail_on = args.fail_on;
    let output_dir = args.output_dir;
    let no_cache = args.no_cache;
    let quiet = args.quiet;
    let ci = args.ci;
    let formats = args.formats;
    let web_target = args.web_target;
    let containers = args.containers;
    let report_types = args.report_types;
    use crate::scanner::{
        checkov::Checkov, container::ContainerScanner, gitleaks::Gitleaks, semgrep::Semgrep,
        syft::Syft, trivy::Trivy, Scanner, ScannerResult,
    };
    use std::time::Instant;

    let start = Instant::now();
    let started_at = chrono::Utc::now().to_rfc3339();
    let scan_id = uuid::Uuid::new_v4().to_string();
    let target_path = PathBuf::from(target);
    let output_path = PathBuf::from(output_dir);

    tracing::info!("Starting scan: {}", target);

    // Initialize cache (disabled if --no-cache is set)
    let cache = if no_cache {
        cache::ScanCache::disabled()
    } else if cfg.cache.enabled {
        let cache = cache::ScanCache::open(&cfg.cache.path)?;
        let _ = cache.enforce_ttl(cfg.cache.ttl_hours);
        cache
    } else {
        cache::ScanCache::disabled()
    };

    // Collect scanners based on requested layers
    let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();

    for layer in layers {
        match layer {
            1 => scanners.push(Box::new(Gitleaks::with_binary(
                cfg.binaries.gitleaks.clone(),
            ))),
            2 => scanners.push(Box::new(Semgrep::with_binary(cfg.binaries.semgrep.clone()))),
            3 => {
                let bin = cfg.binaries.trivy.clone();
                scanners.push(Box::new(Trivy::with_mode_and_binary(
                    crate::scanner::trivy::TrivyMode::Vuln,
                    bin.clone(),
                )));
                scanners.push(Box::new(Trivy::with_mode_and_binary(
                    crate::scanner::trivy::TrivyMode::Secret,
                    bin.clone(),
                )));
                scanners.push(Box::new(Trivy::with_mode_and_binary(
                    crate::scanner::trivy::TrivyMode::Misconfig,
                    bin,
                )));
            }
            4 => {
                // Container image scanning
                for image in &containers {
                    scanners.push(Box::new(ContainerScanner::new(image)));
                }
            }
            5 => {
                // DAST scanning — requires a web target
                if let Some(url) = &web_target {
                    scanners.push(Box::new(crate::scanner::dast::DastScanner::new(url)));
                }
            }
            6 => {
                // IaC scanning via Checkov
                scanners.push(Box::new(Checkov::with_binary(cfg.binaries.checkov.clone())));
            }
            7 => {
                // SBOM inventory via Syft
                scanners.push(Box::new(Syft::with_binary(cfg.binaries.syft.clone())));
            }
            _ => tracing::warn!("Unknown layer: {}", layer),
        }
    }

    // Warn if --web provided but layer 5 not selected
    if let Some(url) = &web_target {
        if !layers.contains(&5) {
            tracing::warn!(
                "--web target '{}' provided but no DAST layer (5) selected. \
                 Use --layers 5 to enable DAST scanning.",
                url
            );
        }
    }

    // Run each scanner in parallel
    let mut all_findings: Vec<find::CanonicalFinding> = Vec::new();
    let mut scanners_used: Vec<String> = Vec::new();

    use futures::future::join_all;
    let scan_results = join_all(scanners.iter().map(|s| {
        let name = s.name();
        tracing::info!("Running scanner: {}", name);
        async {
            let result = s.scan(&target_path).await;
            (name.to_string(), result)
        }
    }))
    .await;

    for (name, result) in scan_results {
        scanners_used.push(name.clone());
        match result {
            Ok(ScannerResult::Complete { findings, .. }) => {
                tracing::info!("  {}: {} findings", name, findings.len());
                all_findings.extend(findings);
            }
            Ok(ScannerResult::NotInstalled { name, hint }) => {
                tracing::warn!("  {}: not installed ({})", name, hint);
            }
            Ok(ScannerResult::Error { name, error }) => {
                tracing::error!("  {}: error - {}", name, error);
            }
            Err(e) => {
                tracing::error!("  {}: failed - {}", name, e);
            }
        }
    }

    // Normalize and deduplicate
    normalize::normalize_findings(&mut all_findings);
    dedup::cross_reference(&mut all_findings);
    let mut final_findings = dedup::deduplicate(all_findings);

    // LLM remediation enhancement — gracefully skips if Ollama is not running
    let llm_cfg = llm::LlmConfig {
        endpoint: cfg.llm.endpoint.clone(),
        model: cfg.llm.model.clone(),
        enabled: cfg.llm.enabled,
    };
    match llm::enhance_remediations(&mut final_findings, &llm_cfg).await {
        Ok(n) if n > 0 => tracing::info!("LLM enhanced {} finding remediations via Ollama", n),
        Ok(_) => {}
        Err(e) => tracing::debug!("LLM enhancement skipped: {}", e),
    }

    // Apply severity filter
    let final_findings = filter_by_severity(final_findings, severity_filter);

    // Check fail-on threshold for CI exit codes
    let fail_threshold_reached = check_fail_on(&final_findings, fail_on);

    // Build attack chains
    let attack_chains = chain::build_attack_chains(&final_findings);

    // Compute Zero Trust scorecard
    let zt_scorecard = normalize::compute_zt_scorecard(&final_findings);

    // Build scan summary (before arch analysis — arch adds optional data)
    let duration = start.elapsed().as_secs_f64();
    let mut by_sev = find::FindingsBySeverity {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        info: 0,
    };
    for f in &final_findings {
        match f.severity {
            find::Severity::Critical => by_sev.critical += 1,
            find::Severity::High => by_sev.high += 1,
            find::Severity::Medium => by_sev.medium += 1,
            find::Severity::Low => by_sev.low += 1,
            find::Severity::Info => by_sev.info += 1,
        }
    }

    let total = final_findings.len();
    let (c_sev, h_sev, m_sev, l_sev, i_sev) = (
        by_sev.critical,
        by_sev.high,
        by_sev.medium,
        by_sev.low,
        by_sev.info,
    );

    let chain_count = attack_chains.len();

    // Discover architecture artifacts and assess component risks
    let arch_artifacts = crate::arch::discover_artifacts(&target_path);
    let component_risks = if !arch_artifacts.is_empty() {
        Some(crate::arch::assess_component_risks(
            &final_findings,
            &arch_artifacts,
        ))
    } else {
        None
    };
    let arch_diagram = component_risks.as_ref().and_then(|risks| {
        if !arch_artifacts.is_empty() {
            Some(crate::arch::generate_mermaid_diagram(
                &arch_artifacts,
                risks,
            ))
        } else {
            None
        }
    });

    let summary = find::ScanSummary {
        scan_id: scan_id.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        target: target.to_string(),
        target_hash: format!("{:x}", sha2::Sha256::digest(target.as_bytes())),
        duration_seconds: duration,
        total_findings: total as u32,
        findings_by_severity: by_sev,
        scanners_used: scanners_used.clone(),
        zt_scorecard: Some(zt_scorecard.clone()),
        attack_chains,
    };

    // Record scan in cache (with findings snapshot for report regeneration)
    let completed_at = chrono::Utc::now().to_rfc3339();
    cache.record_scan(cache::RecordScanInput {
        scan_id: &scan_id,
        target,
        started_at: &started_at,
        completed_at: &completed_at,
        total_findings: final_findings.len() as u32,
        scanners_used: &scanners_used,
        findings: &final_findings,
    })?;

    // Convert CLI report-type flags to report module enum
    let selected_report_types: Vec<report::ReportType> = report_types
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
        &final_findings,
        &zt_scorecard,
        &output_path,
        arch_diagram.as_deref(),
        &selected_report_types,
    )?;

    // Generate additional output formats
    for fmt in &formats {
        let path = match fmt {
            cli::OutputFormat::Md => continue, // Already generated
            cli::OutputFormat::Json => report::generate_json_report(
                &summary,
                &final_findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
            )?,
            cli::OutputFormat::Sarif => report::generate_sarif_report(
                &summary,
                &final_findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
            )?,
            cli::OutputFormat::Html => report::generate_html_report(
                &summary,
                &final_findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
            )?,
            cli::OutputFormat::Pdf => {
                // PDF not yet implemented, skip
                tracing::warn!("PDF output format not yet implemented");
                continue;
            }
        };
        report_paths.push(path);
    }

    // Print results summary
    quiet_println!(quiet, "");
    quiet_println!(quiet, "═══ ApeGuard Scan Complete ═══");
    quiet_println!(quiet, "  Target:  {}", target);
    quiet_println!(quiet, "  Duration: {:.1}s", duration);
    quiet_println!(
        quiet,
        "  Findings: {} (C:{}, H:{}, M:{}, L:{}, I:{})",
        total,
        c_sev,
        h_sev,
        m_sev,
        l_sev,
        i_sev
    );
    quiet_println!(quiet, "  Attack Chains: {}", chain_count);
    quiet_println!(quiet, "  Reports:");
    for p in &report_paths {
        quiet_println!(quiet, "    📋 {}", p.display());
    }
    quiet_println!(quiet, "");

    // Enforce --fail-on threshold for CI exit codes
    if fail_threshold_reached {
        if ci {
            // CI mode: clean exit code without error message
            if !quiet {
                eprintln!(
                    "FAILED: findings at or above '{}' threshold",
                    match fail_on {
                        cli::FailOnThreshold::Critical => "critical",
                        cli::FailOnThreshold::High => "high",
                        cli::FailOnThreshold::Never => unreachable!(),
                    }
                );
            }
            std::process::exit(1);
        } else {
            anyhow::bail!(
                "❌ Fail-on threshold '{}' reached — found findings at or above this severity",
                match fail_on {
                    cli::FailOnThreshold::Critical => "critical",
                    cli::FailOnThreshold::High => "high",
                    cli::FailOnThreshold::Never => unreachable!(),
                }
            );
        }
    }

    Ok(())
}

/// Check if findings exceed the fail-on threshold
fn check_fail_on(findings: &[find::CanonicalFinding], threshold: &cli::FailOnThreshold) -> bool {
    match threshold {
        cli::FailOnThreshold::Never => false,
        cli::FailOnThreshold::High => findings
            .iter()
            .any(|f| matches!(f.severity, find::Severity::High | find::Severity::Critical)),
        cli::FailOnThreshold::Critical => findings
            .iter()
            .any(|f| matches!(f.severity, find::Severity::Critical)),
    }
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
            )?,
            cli::OutputFormat::Sarif => report::generate_sarif_report(
                &summary,
                &findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
            )?,
            cli::OutputFormat::Html => report::generate_html_report(
                &summary,
                &findings,
                &zt_scorecard,
                &output_path,
                arch_diagram.as_deref(),
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

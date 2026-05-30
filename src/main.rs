// ApeGuard CLI — Security Posture Assessment
// One command. Three reports. Zero Trust mapped.
//
// Architecture: docs/03-Projects/ApeGuard/ApeGuard_Architecture.md
#![allow(dead_code)]

mod cli;
mod config;
mod scanner;
mod find;
mod normalize;
mod dedup;
mod cache;
mod chain;
mod arch;
mod report;

use std::path::PathBuf;
use sha2::Digest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Parse CLI args
    let args = cli::parse();

    // Load config (merge defaults + file + env + flags)
    let cfg = config::load(&args)?;

    match &args.command {
        cli::Command::Scan { path, layers, severity, output_dir, .. } => {
            let target = path.clone().unwrap_or_else(|| ".".to_string());
            let sev = severity.clone();
            run_scan(&target, layers, &sev, output_dir, &cfg).await?;
        }
        cli::Command::Report { path, snapshot, output_dir, .. } => {
            let target = path.clone().unwrap_or_else(|| ".".to_string());
            run_report(&target, snapshot.as_deref(), output_dir, &cfg).await?;
        }
        cli::Command::Compare { a, b, .. } => {
            run_compare(a, b).await?;
        }
        cli::Command::Init { path, template } => {
            let t = template.clone();
            config::generate_init(path.clone(), t)?;
        }
        cli::Command::Config { subcommand } => {
            handle_config(subcommand, &cfg)?;
        }
        cli::Command::Version => {
            print_version().await?;
        }
        cli::Command::Completions { shell } => {
            cli::generate_completions(*shell);
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
            filter, removed, after
        );
    }

    filtered
}

/// Run a full security scan pipeline
async fn run_scan(
    target: &str,
    layers: &[u8],
    _severity: &cli::SeverityFilter,
    output_dir: &str,
    cfg: &config::Config,
) -> anyhow::Result<()> {
    use crate::scanner::{Scanner, ScannerResult, gitleaks::Gitleaks, semgrep::Semgrep, trivy::Trivy};
    use std::time::Instant;

    let start = Instant::now();
    let scan_id = uuid::Uuid::new_v4().to_string();
    let target_path = PathBuf::from(target);
    let output_path = PathBuf::from(output_dir);

    tracing::info!("Starting scan: {}", target);

    // Initialize cache
    let cache = if cfg.cache.enabled {
        cache::ScanCache::open(&cfg.cache.path)?
    } else {
        cache::ScanCache::disabled()
    };

    // Collect scanners based on requested layers
    let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();

    for layer in layers {
        match layer {
            1 => scanners.push(Box::new(Gitleaks::new())),     // Secrets
            2 => scanners.push(Box::new(Semgrep::new())),       // SAST
            3 => {
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Vuln)));
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Secret)));
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Misconfig)));
            }
            _ => tracing::warn!("Unknown layer: {}", layer),
        }
    }

    // Run each scanner
    let mut all_findings: Vec<find::CanonicalFinding> = Vec::new();
    let mut scanners_used: Vec<String> = Vec::new();

    for s in &scanners {
        let name = s.name();
        tracing::info!("Running scanner: {}", name);
        scanners_used.push(name.to_string());

        match s.scan(&target_path).await {
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
    let final_findings = dedup::deduplicate(all_findings);

    // Apply severity filter
    let final_findings = filter_by_severity(final_findings, _severity);

    // Build attack chains
    let attack_chains = chain::build_attack_chains(&final_findings);

    // Compute Zero Trust scorecard
    let zt_scorecard = normalize::compute_zt_scorecard(&final_findings);

    // Build summary
    let duration = start.elapsed().as_secs_f64();
    let mut by_sev = find::FindingsBySeverity {
        critical: 0, high: 0, medium: 0, low: 0, info: 0,
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
        by_sev.critical, by_sev.high, by_sev.medium, by_sev.low, by_sev.info,
    );

    // Show chain count separately (before move)
    let chain_count = attack_chains.len();

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

    // Record scan in cache
    cache.record_scan(&scan_id, target, final_findings.len() as u32, &scanners_used)?;

    // Generate reports
    let report_paths = report::generate_all_reports(
        &summary,
        &final_findings,
        &zt_scorecard,
        &output_path,
    )?;

    // Print results summary
    println!();
    println!("═══ ApeGuard Scan Complete ═══");
    println!("  Target:  {}", target);
    println!("  Duration: {:.1}s", duration);
    println!("  Findings: {} (C:{}, H:{}, M:{}, L:{}, I:{})",
        total, c_sev, h_sev, m_sev, l_sev, i_sev);
    println!("  Attack Chains: {}", chain_count);
    println!("  Reports:");
    for p in &report_paths {
        println!("    📋 {}", p.display());
    }
    println!();

    Ok(())
}

/// Regenerate reports from cached scan
async fn run_report(
    _target: &str,
    snapshot: Option<&str>,
    output_dir: &str,
    _cfg: &config::Config,
) -> anyhow::Result<()> {
    // For now, just note that cached regeneration requires the cache module
    tracing::info!("Report regeneration from cache: snapshot={:?}", snapshot);

    let output_path = PathBuf::from(output_dir);
    std::fs::create_dir_all(&output_path)?;

    if let Some(snap) = snapshot {
        tracing::info!("Would load scan {} from cache", snap);
    }

    // TODO: implement full cache-based report regeneration in P2
    anyhow::bail!("Cache-based report regeneration is not yet implemented. Re-run with `apeguard scan` instead.");
}

/// Compare two scan snapshots
async fn run_compare(_a: &str, _b: &str) -> anyhow::Result<()> {
    // TODO: implement scan comparison in P2
    tracing::info!("Scan comparison: not yet implemented (P2)");
    anyhow::bail!("Scan comparison is not yet implemented (scheduled for P2).");
}

/// Handle config subcommand
fn handle_config(subcommand: &Option<cli::ConfigSubcommand>, cfg: &config::Config) -> anyhow::Result<()> {
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

/// Print version and dependency status
async fn print_version() -> anyhow::Result<()> {
    println!("ApeGuard v{}", env!("CARGO_PKG_VERSION"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!();

    // Check if each scanner is available
    let scanners: [(&str, &str, &[&str]); 3] = [
        ("Gitleaks", "gitleaks", &["version"]),
        ("Semgrep", "semgrep", &["--version"]),
        ("Trivy", "trivy", &["--version"]),
    ];

    for (name, binary, args) in &scanners {
        let status = tokio::process::Command::new(binary)
            .args(*args)
            .output()
            .await;

        match status {
            Ok(output) if output.status.success() => {
                let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  ✅ {}: {}", name, ver.lines().next().unwrap_or("installed"));
            }
            _ => {
                println!("  ❌ {}: not found", name);
            }
        }
    }

    Ok(())
}

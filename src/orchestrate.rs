// Scan orchestration — owns the top-level scan pipeline.
// All other modules (scanner, normalize, dedup, score, etc.) are called from here.

macro_rules! quiet_println {
    ($quiet:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        if !$quiet {
            println!($fmt $(, $arg)*);
        }
    };
}

use sha2::Digest as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use futures::future::join_all;

use crate::cache;
use crate::chain;
use crate::cli;
use crate::config;
use crate::dedup;
use crate::filter;
use crate::find::{self, AttackChain, FindingsBySeverity, ZeroTrustScorecard};
use crate::grade;
use crate::llm;
use crate::normalize;
use crate::policy;
use crate::reachability;
use crate::report;
use crate::scanner::{
    aws_s3::AwsS3Scanner, checkov::Checkov, container::ContainerScanner, context_drift,
    dast::DastScanner, gitleaks::Gitleaks, mcp_security::McpScanner, semgrep::Semgrep, syft::Syft,
    terraform::TerraformScanner, tls::TlsScanner, trivy::Trivy, trufflehog::Trufflehog,
    zap::ZapScanner, Scanner, ScannerResult,
};
use crate::score;
use crate::stride;

/// The output of a successful scan — all data needed by main.rs for reporting.
#[expect(dead_code)]
pub(crate) struct ScanOutput {
    pub scan_id: String,
    pub findings: Vec<find::CanonicalFinding>,
    pub findings_jsonl_path: PathBuf,
    pub started_at: String,
    pub duration_secs: f64,
    pub by_severity: FindingsBySeverity,
    pub scanners_used: Vec<String>,
    pub attack_chains: Vec<AttackChain>,
    pub zt_scorecard: ZeroTrustScorecard,
    pub reachability_result: reachability::ReachabilityResult,
    pub policy_result: policy::PolicyResult,
    pub stride_result: Option<stride::StrideResult>,
    pub fail_threshold_reached: bool,
    pub cache: cache::ScanCache,
}

/// Arguments for a scan operation, grouped to avoid too-many-arguments lint
pub(crate) struct ScanArgs<'a> {
    pub target: &'a str,
    pub layers: &'a [u8],
    pub severity: &'a cli::SeverityFilter,
    pub fail_on: &'a cli::FailOnThreshold,
    pub output_dir: &'a str,
    pub no_cache: bool,
    pub quiet: bool,
    #[expect(dead_code)] // Handled in main.rs before constructing ScanArgs
    pub ci: bool,
    pub resume: bool,
    pub grade: bool,
    #[expect(dead_code)] // Context drift now runs via the Scanner trait impl
    pub context_drift: bool,
    pub stride: bool,
    pub policy: bool,
    pub policy_dir: Option<String>,
    pub formats: Vec<cli::OutputFormat>,
    pub web_target: Option<String>,
    pub containers: Vec<String>,
    pub report_types: Vec<cli::ReportType>,
}

/// Filter findings by minimum severity threshold
pub(crate) fn filter_by_severity(
    findings: Vec<find::CanonicalFinding>,
    filter: &cli::SeverityFilter,
) -> Vec<find::CanonicalFinding> {
    use find::Severity;

    let min_severity = match filter {
        cli::SeverityFilter::All => return findings,
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

/// Read found_findings.jsonl and return the set of scanner names that completed successfully.
pub(crate) fn load_completed_scanners(
    path: &std::path::Path,
) -> anyhow::Result<std::collections::HashSet<String>> {
    if !path.exists() {
        return Ok(std::collections::HashSet::new());
    }

    let content = std::fs::read_to_string(path)?;
    let mut completed = std::collections::HashSet::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
            if entry["status"] == "complete" {
                if let Some(scanner) = entry["scanner"].as_str() {
                    completed.insert(scanner.to_string());
                }
            }
        }
    }

    Ok(completed)
}

/// Load persisted findings for scanners that completed in a previous run.
///
/// Complements `load_completed_scanners`: each completed scanner also wrote one
/// JSONL line per finding (`"status": "finding"`), which is reloaded here so a
/// resumed scan rebuilds the full finding set instead of an empty report.
pub(crate) fn load_previous_findings(
    path: &std::path::Path,
    completed: &std::collections::HashSet<String>,
) -> anyhow::Result<Vec<find::CanonicalFinding>> {
    if !path.exists() || completed.is_empty() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(path)?;
    let mut findings = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
            if entry["status"] == "finding" {
                if let Some(scanner) = entry["scanner"].as_str() {
                    if completed.contains(scanner) {
                        match serde_json::from_value::<find::CanonicalFinding>(
                            entry["finding"].clone(),
                        ) {
                            Ok(f) => findings.push(f),
                            Err(e) => {
                                tracing::warn!(
                                    "Resume mode: skipping malformed finding line for {}: {}",
                                    scanner,
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(findings)
}

/// Check if any findings exceed the fail-on threshold (for CI exit codes)
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

/// Generate reports from scan output
#[expect(clippy::too_many_arguments)]
fn generate_summary_report(
    findings: &[find::CanonicalFinding],
    target: &str,
    started_at: &str,
    duration_secs: f64,
    by_severity: &FindingsBySeverity,
    scanners_used: &[String],
    attack_chains: &[AttackChain],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    scan_id: &str,
    report_types: &[cli::ReportType],
    formats: &[cli::OutputFormat],
    stride_result: Option<&stride::StrideResult>,
    policy_result: &policy::PolicyResult,
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let summary = find::ScanSummary {
        scan_id: scan_id.to_string(),
        timestamp: started_at.to_string(),
        target: target.to_string(),
        target_hash: format!("{:x}", sha2::Sha256::digest(target.as_bytes())),
        duration_seconds: duration_secs,
        total_findings: findings.len() as u32,
        findings_by_severity: by_severity.clone(),
        scanners_used: scanners_used.to_vec(),
        zt_scorecard: Some(zt_scorecard.clone()),
        attack_chains: attack_chains.to_vec(),
    };

    let selected_report_types: Vec<report::ReportType> = if report_types.is_empty() {
        vec![
            report::ReportType::Technical,
            report::ReportType::Executive,
            report::ReportType::Roadmap,
        ]
    } else {
        report_types
            .iter()
            .map(|r| match r {
                cli::ReportType::Tech => report::ReportType::Technical,
                cli::ReportType::Exec => report::ReportType::Executive,
                cli::ReportType::Roadmap => report::ReportType::Roadmap,
            })
            .collect()
    };

    let mut report_paths = report::generate_all_reports(
        &summary,
        findings,
        zt_scorecard,
        output_dir,
        None,
        &selected_report_types,
        stride_result,
        Some(policy_result),
    )?;

    // Generate additional output formats
    for fmt in formats {
        let path = match fmt {
            cli::OutputFormat::Md => continue,
            cli::OutputFormat::Json => report::generate_json_report(
                &summary,
                findings,
                zt_scorecard,
                output_dir,
                None,
                stride_result,
                Some(policy_result),
            )?,
            cli::OutputFormat::Sarif => report::generate_sarif_report(
                &summary,
                findings,
                zt_scorecard,
                output_dir,
                None,
                stride_result,
                Some(policy_result),
            )?,
            cli::OutputFormat::Html => report::generate_html_report(
                &summary,
                findings,
                zt_scorecard,
                output_dir,
                None,
                stride_result,
                Some(policy_result),
            )?,
        };
        report_paths.push(path);
    }

    Ok(report_paths)
}

/// Run a full security scan pipeline
pub(crate) async fn run_scan(
    args: ScanArgs<'_>,
    cfg: &config::Config,
) -> anyhow::Result<ScanOutput> {
    let target = args.target;
    let layers = args.layers;
    let severity_filter = args.severity;
    let fail_on = args.fail_on;
    let output_dir = args.output_dir;
    let no_cache = args.no_cache;
    let resume = args.resume;
    let grade_flag = args.grade;
    let stride_flag = args.stride;
    let web_target = args.web_target;
    let containers = args.containers;
    let policy_flag = args.policy;
    let policy_dir = args.policy_dir.clone();

    let start = Instant::now();
    let started_at = chrono::Utc::now().to_rfc3339();
    // Single canonical scan ID for this run — used by reports AND the cache so
    // `report --snapshot <id>` / `compare <id>` work with the ID shown in reports.
    let scan_id = format!("scan_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S%3f"));
    let target_path = PathBuf::from(target);
    let output_path = PathBuf::from(output_dir);
    std::fs::create_dir_all(&output_path).ok();

    let findings_jsonl_path = output_path.join("found_findings.jsonl");

    // Start a fresh JSONL unless resuming — stale entries from an earlier run
    // would otherwise cause a later `--resume` to skip unrelated scanners.
    if !resume {
        let _ = std::fs::File::create(&findings_jsonl_path); // truncate
    }

    tracing::info!("Starting scan: {}", target);

    // Initialize cache
    let cache = if no_cache {
        cache::ScanCache::disabled()?
    } else if cfg.cache.enabled {
        let c = cache::ScanCache::open(&cfg.cache.path)?;
        let _ = c.enforce_ttl(cfg.cache.ttl_hours);
        c
    } else {
        cache::ScanCache::disabled()?
    };

    // Collect scanners by layer
    let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();

    for layer in layers {
        match layer {
            1 => {
                scanners.push(Box::new(Gitleaks::with_binary(
                    cfg.binaries.gitleaks.clone(),
                )));
                scanners.push(Box::new(Trufflehog::with_binary(
                    cfg.binaries.trufflehog.clone(),
                )));
            }
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
                for image in &containers {
                    scanners.push(Box::new(ContainerScanner::new(image)));
                }
            }
            5 => {
                if let Some(url) = &web_target {
                    scanners.push(Box::new(DastScanner::new(url)));
                    scanners.push(Box::new(ZapScanner::new(url)));
                }
            }
            6 => {
                scanners.push(Box::new(Checkov::with_binary(cfg.binaries.checkov.clone())));
            }
            7 => {
                scanners.push(Box::new(Syft::with_binary(cfg.binaries.syft.clone())));
            }
            8 => {
                // Layer 8: Context Drift Detection
                scanners.push(Box::new(context_drift::ContextDriftScanner::new(
                    Path::new("."),
                )));
            }
            9 => {
                // Layer 9: MCP Security Scanner
                scanners.push(Box::new(McpScanner::new(".apeguard/mcp-config.json")));
            }
            10 => {
                // Layer 10: Terraform IaC Scanner
                scanners.push(Box::new(TerraformScanner::new(".")));
            }
            11 => {
                // Layer 11: AWS S3 Scanner
                scanners.push(Box::new(AwsS3Scanner::new(".apeguard/aws-config.json")));
            }
            12 => {
                // Layer 12: TLS Certificate Scanner
                scanners.push(Box::new(TlsScanner::new(&[
                    "/etc/ssl/certs/ca-certificates.crt",
                ])));
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

    // Resume: skip completed scanners and remember which ones to reload
    let mut resume_completed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut resume_skipped_names: Vec<String> = Vec::new();
    if resume {
        match load_completed_scanners(&findings_jsonl_path) {
            Ok(completed) if !completed.is_empty() => {
                tracing::info!(
                    "Resume mode: {} scanners already completed: {:?}",
                    completed.len(),
                    completed
                );
                resume_skipped_names = scanners
                    .iter()
                    .filter(|s| completed.contains(s.name()))
                    .map(|s| s.name().to_string())
                    .collect();
                scanners.retain(|s| !completed.contains(s.name()));
                resume_completed = completed;
                if scanners.is_empty() {
                    tracing::info!(
                        "All requested layers already completed — regenerating report from existing data."
                    );
                }
            }
            Ok(_) => {
                tracing::info!(
                    "Resume mode: no previous scanner results found, running all layers."
                );
            }
            Err(e) => {
                tracing::warn!(
                    "Resume mode: could not read previous results ({}), running all layers.",
                    e
                );
            }
        }
    }

    // Run scanners in parallel
    let mut all_findings: Vec<find::CanonicalFinding> = Vec::new();
    let mut scanners_used: Vec<String> = Vec::new();

    // Cap concurrent scanner processes: a full 12-layer scan with many
    // containers would otherwise spawn dozens of simultaneous scanner binaries.
    const MAX_CONCURRENT_SCANNERS: usize = 8;
    let scanner_semaphore =
        std::sync::Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_SCANNERS));

    let scan_results = join_all(scanners.iter().map(|s| {
        let name = s.name();
        let semaphore = std::sync::Arc::clone(&scanner_semaphore);
        let target = std::path::PathBuf::clone(&target_path);
        tracing::info!("Running scanner: {}", name);
        async move {
            // Acquire inside the task: every task is spawned immediately, but
            // only MAX_CONCURRENT_SCANNERS run their scanner at a time.
            let _permit = semaphore.acquire_owned().await;
            let result = s.scan(&target).await;
            (name.to_string(), result)
        }
    }))
    .await;

    for (name, result) in scan_results {
        scanners_used.push(name.clone());
        let (status, finding_count) = match &result {
            Ok(ScannerResult::Complete { findings, .. }) => {
                tracing::info!("  {}: {} findings", name, findings.len());
                ("complete", findings.len())
            }
            Ok(ScannerResult::NotInstalled { .. }) => {
                tracing::warn!("  {}: not installed", name);
                ("skipped", 0)
            }
            Ok(ScannerResult::Error { error, .. }) => {
                tracing::error!("  {}: error - {}", name, error);
                ("error", 0)
            }
            Err(e) => {
                tracing::error!("  {}: failed - {}", name, e);
                ("error", 0)
            }
        };

        // Stream to JSONL for resume
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&findings_jsonl_path)
        {
            let entry = serde_json::json!({
                "scanner": name,
                "status": status,
                "finding_count": finding_count,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            let _ = writeln!(file, "{}", entry);

            // Persist each finding so --resume can reload results for skipped
            // scanners instead of regenerating empty reports.
            if let Ok(ScannerResult::Complete { findings, .. }) = &result {
                for f in findings {
                    let entry = serde_json::json!({
                        "scanner": name,
                        "status": "finding",
                        "finding": f,
                    });
                    let _ = writeln!(file, "{}", entry);
                }
            }
        }

        match result {
            Ok(ScannerResult::Complete { findings, .. }) => {
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

    // Reload findings for scanners skipped by --resume so the pipeline sees the
    // full result set — otherwise resumed scans produce empty reports and a
    // 0-finding cache entry.
    if !resume_completed.is_empty() {
        match load_previous_findings(&findings_jsonl_path, &resume_completed) {
            Ok(previous) if !previous.is_empty() => {
                tracing::info!(
                    "Resume mode: reloaded {} findings from {} skipped scanner(s)",
                    previous.len(),
                    resume_skipped_names.len()
                );
                for name in &resume_skipped_names {
                    if !scanners_used.contains(name) {
                        scanners_used.push(name.clone());
                    }
                }
                all_findings.extend(previous);
            }
            Ok(_) => {}
            Err(e) => tracing::warn!("Resume mode: could not reload previous findings: {}", e),
        }
    }

    // Normalize and deduplicate
    normalize::normalize_findings(&mut all_findings);
    dedup::cross_reference(&mut all_findings);
    let mut final_findings = dedup::deduplicate(all_findings);

    // Reachability analysis
    let reachability_result = reachability::analyze_reachability(
        &final_findings,
        &target_path,
        &reachability::ReachabilityConfig {
            enabled: cfg.reachability.enabled,
            entry_points: cfg.reachability.entry_points.clone(),
            include_extensions: cfg.reachability.include_extensions.clone(),
            exclude_dirs: cfg.reachability.exclude_dirs.clone(),
        },
    );
    if reachability_result.enabled {
        reachability::apply_reachability(&mut final_findings, &reachability_result, &target_path);
    }

    // False-positive suppression filters
    let (mut final_findings, filter_stats) = filter::apply_fp_filters(final_findings, &cfg.filters);
    if filter_stats.total_removed() > 0 {
        tracing::info!(
            "FP filter removed {} findings (path:{}, test:{}, cross:{}, grade:{}, conf:{}, sev:{})",
            filter_stats.total_removed(),
            filter_stats.path_excluded,
            filter_stats.test_suppressed,
            filter_stats.cross_scanner_filtered,
            filter_stats.grade_rejected,
            filter_stats.confidence_filtered,
            filter_stats.severity_filtered,
        );
    }

    // LLM remediation enhancement
    let llm_cfg = llm::LlmConfig {
        endpoint: cfg.llm.endpoint.clone(),
        model: cfg.llm.model.clone(),
        enabled: cfg.llm.enabled,
    };
    match llm::enhance_remediations(&mut final_findings, &llm_cfg).await {
        Ok(n) if n > 0 => {
            tracing::info!("LLM enhanced {} finding remediations via Ollama", n);
        }
        Ok(_) => {}
        Err(e) => tracing::debug!("LLM enhancement skipped: {}", e),
    }

    // Adversarial grading
    if grade_flag {
        let grade_count =
            grade::grade_findings(&mut final_findings, &cfg.llm.endpoint, &cfg.llm.model).await?;
        if grade_count > 0 {
            let counts = grade::count_verdicts(&final_findings);
            tracing::info!(
                "Graded {} findings via adversarial verification ({} confirmed, {} rejected, {} needs review)",
                grade_count,
                counts.confirmed,
                counts.rejected,
                counts.needs_review,
            );
        }
        // Drop findings the grader just rejected. The FP filter's grade stage
        // ran before grading, so this run's verdicts were not visible to it.
        let (post_grade_findings, rejected) = filter::apply_grade_rejection(final_findings);
        if rejected > 0 {
            tracing::info!(
                "Grade rejection removed {} grader-rejected findings",
                rejected
            );
        }
        final_findings = post_grade_findings;
    }

    // Zero Trust scorecard (computed before severity filter so it reflects full findings)
    let zt_scorecard = normalize::compute_zt_scorecard(&final_findings);

    // Apply severity filter
    final_findings = filter_by_severity(final_findings, severity_filter);

    // Policy-as-Code evaluation
    let policy_config = policy::PolicyConfig {
        enabled: policy_flag || cfg.policy.enabled,
        policy_dir: policy_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| cfg.policy.policy_dir.clone()),
    };
    let (mut final_findings, policy_result) =
        policy::evaluate_policies(final_findings, &policy_config);

    // Check fail-on threshold
    let fail_threshold_reached = check_fail_on(&final_findings, fail_on);

    // Attack chains
    let attack_chains = chain::build_attack_chains(&final_findings);

    // Risk scores
    score::score_all_findings(
        &mut final_findings,
        &attack_chains,
        &score::ScoreWeights::default(),
    );

    // STRIDE threat model coverage
    let stride_result = if stride_flag || cfg.stride.enabled {
        let threshold = cfg.stride.coverage_threshold.clamp(0.0, 1.0);
        let result = stride::analyze_stride_coverage(&final_findings, threshold);
        tracing::info!(
            "STRIDE coverage: {:.0}% ({}/6 categories covered, {} gap(s))",
            result.coverage_score * 100.0,
            result.covered_categories,
            result.gaps.len(),
        );
        for gap in &result.gaps {
            tracing::info!("  STRIDE gap: {} — {}", gap.label(), gap.description());
        }
        Some(result)
    } else {
        None
    };

    // Build severity counts
    let duration = start.elapsed().as_secs_f64();
    let mut by_sev = FindingsBySeverity {
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
    tracing::info!(
        "Scan complete: {} total findings (C:{}, H:{}, M:{}, L:{}, I:{}) in {:.1}s",
        total,
        by_sev.critical,
        by_sev.high,
        by_sev.medium,
        by_sev.low,
        by_sev.info,
        duration,
    );

    if fail_threshold_reached {
        tracing::warn!("Fail-on threshold reached — exit code will indicate CI failure");
    }

    // Generate reports and output formats
    let report_paths = generate_summary_report(
        &final_findings,
        target,
        &started_at,
        duration,
        &by_sev,
        &scanners_used,
        &attack_chains,
        &zt_scorecard,
        &output_path,
        &scan_id,
        &args.report_types,
        &args.formats,
        stride_result.as_ref(),
        &policy_result,
    )?;
    for p in &report_paths {
        quiet_println!(args.quiet, "    📋 {}", p.display());
    }

    // Final stdout summary (mirrors old main.rs output)
    quiet_println!(args.quiet, "");
    quiet_println!(args.quiet, "═══ ApeGuard Scan Complete ═══");
    quiet_println!(args.quiet, "  Target:  {}", target);
    quiet_println!(args.quiet, "  Duration: {:.1}s", duration);
    quiet_println!(
        args.quiet,
        "  Findings: {} (C:{}, H:{}, M:{}, L:{}, I:{})",
        total,
        by_sev.critical,
        by_sev.high,
        by_sev.medium,
        by_sev.low,
        by_sev.info
    );
    quiet_println!(args.quiet, "  Attack Chains: {}", attack_chains.len());
    quiet_println!(args.quiet, "");

    Ok(ScanOutput {
        scan_id,
        findings: final_findings,
        findings_jsonl_path,
        started_at,
        duration_secs: duration,
        by_severity: by_sev,
        scanners_used,
        attack_chains,
        zt_scorecard,
        reachability_result,
        policy_result,
        stride_result,
        fail_threshold_reached,
        cache,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::ScannerType;

    fn sample_finding(id: &str, scanner: ScannerType) -> find::CanonicalFinding {
        find::CanonicalFinding {
            id: id.to_string(),
            scanner,
            scanner_version: None,
            rule_id: "test-rule".to_string(),
            severity: find::Severity::High,
            confidence: find::Confidence::Firm,
            title: "Test finding".to_string(),
            description: "desc".to_string(),
            location: find::FindingLocation {
                file: std::path::PathBuf::from("src/main.rs"),
                line: Some(1),
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        }
    }

    /// Write a found_findings.jsonl with a completed status line and finding lines.
    fn write_jsonl(path: &std::path::Path, scanner: &str, findings: &[find::CanonicalFinding]) {
        use std::io::Write;
        let mut file = std::fs::File::create(path).expect("test: create jsonl");
        let status = serde_json::json!({
            "scanner": scanner,
            "status": "complete",
            "finding_count": findings.len(),
            "timestamp": "2026-01-01T00:00:00Z",
        });
        writeln!(file, "{}", status).expect("test: write status line");
        for f in findings {
            let entry = serde_json::json!({
                "scanner": scanner,
                "status": "finding",
                "finding": f,
            });
            writeln!(file, "{}", entry).expect("test: write finding line");
        }
    }

    #[test]
    fn test_load_completed_scanners_reads_status_lines() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");
        write_jsonl(
            &path,
            "gitleaks",
            &[sample_finding("F-1", ScannerType::Gitleaks)],
        );

        let completed = load_completed_scanners(&path).expect("load completed");
        assert!(completed.contains("gitleaks"));
        assert!(!completed.contains("semgrep"));
    }

    #[test]
    fn test_load_completed_scanners_missing_file() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("does-not-exist.jsonl");
        let completed = load_completed_scanners(&path).expect("load completed");
        assert!(completed.is_empty());
    }

    #[test]
    fn test_load_previous_findings_returns_completed_scanner_findings() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");
        let findings = vec![
            sample_finding("F-1", ScannerType::Gitleaks),
            sample_finding("F-2", ScannerType::Gitleaks),
        ];
        write_jsonl(&path, "gitleaks", &findings);

        let mut completed = std::collections::HashSet::new();
        completed.insert("gitleaks".to_string());

        let reloaded = load_previous_findings(&path, &completed).expect("reload findings");
        assert_eq!(reloaded.len(), 2);
        assert_eq!(reloaded[0].id, "F-1");
        assert_eq!(reloaded[1].id, "F-2");
        assert!(matches!(reloaded[0].scanner, ScannerType::Gitleaks));
    }

    #[test]
    fn test_load_previous_findings_skips_uncompleted_scanners() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");
        write_jsonl(
            &path,
            "gitleaks",
            &[sample_finding("F-1", ScannerType::Gitleaks)],
        );

        // semgrep completed, but the JSONL only has gitleaks findings
        let mut completed = std::collections::HashSet::new();
        completed.insert("semgrep".to_string());

        let reloaded = load_previous_findings(&path, &completed).expect("reload findings");
        assert!(reloaded.is_empty());
    }

    #[test]
    fn test_load_previous_findings_empty_completed_set() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");
        write_jsonl(
            &path,
            "gitleaks",
            &[sample_finding("F-1", ScannerType::Gitleaks)],
        );

        let reloaded =
            load_previous_findings(&path, &std::collections::HashSet::new()).expect("reload");
        assert!(reloaded.is_empty());
    }

    #[test]
    fn test_load_previous_findings_roundtrips_all_fields() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");

        let mut original = sample_finding("F-1", ScannerType::Semgrep);
        original.severity = find::Severity::Critical;
        original.cwe = Some("CWE-79".to_string());
        original.zt_pillars = vec!["endpoints".to_string()];
        original.reachable = Some(true);

        write_jsonl(&path, "semgrep", std::slice::from_ref(&original));

        let mut completed = std::collections::HashSet::new();
        completed.insert("semgrep".to_string());
        let reloaded = load_previous_findings(&path, &completed).expect("reload");
        assert_eq!(reloaded.len(), 1);
        assert_eq!(reloaded[0].severity, find::Severity::Critical);
        assert_eq!(reloaded[0].cwe.as_deref(), Some("CWE-79"));
        assert_eq!(reloaded[0].zt_pillars, vec!["endpoints"]);
        assert_eq!(reloaded[0].reachable, Some(true));
    }

    #[test]
    fn test_jsonl_roundtrip_survives_malformed_lines() {
        let tmp = tempfile::tempdir().expect("test: tempdir");
        let path = tmp.path().join("found_findings.jsonl");
        use std::io::Write;
        let mut file = std::fs::File::create(&path).expect("test: create jsonl");
        writeln!(file, "{{ not valid json").expect("test: write junk");
        writeln!(
            file,
            "{}",
            serde_json::json!({
                "scanner": "gitleaks",
                "status": "finding",
                "finding": sample_finding("F-1", ScannerType::Gitleaks),
            })
        )
        .expect("test: write finding");
        drop(file);

        let mut completed = std::collections::HashSet::new();
        completed.insert("gitleaks".to_string());
        let reloaded = load_previous_findings(&path, &completed).expect("reload");
        assert_eq!(reloaded.len(), 1);
    }
}

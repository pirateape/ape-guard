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
    checkov::Checkov, container::ContainerScanner, context_drift, dast::DastScanner,
    gitleaks::Gitleaks, semgrep::Semgrep, syft::Syft, trivy::Trivy, trufflehog::Trufflehog,
    Scanner, ScannerResult,
};
use crate::score;
use crate::stride;

/// The output of a successful scan — all data needed by main.rs for reporting.
#[allow(dead_code)]
pub(crate) struct ScanOutput {
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
#[allow(dead_code)]
pub(crate) struct ScanArgs<'a> {
    pub target: &'a str,
    pub layers: &'a [u8],
    pub severity: &'a cli::SeverityFilter,
    pub fail_on: &'a cli::FailOnThreshold,
    pub output_dir: &'a str,
    pub no_cache: bool,
    pub quiet: bool,
    pub ci: bool,
    pub resume: bool,
    pub grade: bool,
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
#[allow(clippy::too_many_arguments)]
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
    report_types: &[cli::ReportType],
    formats: &[cli::OutputFormat],
    stride_result: Option<&stride::StrideResult>,
    policy_result: &policy::PolicyResult,
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let scan_id = format!("{:x}", sha2::Sha256::digest(started_at.as_bytes()));
    let summary = find::ScanSummary {
        scan_id,
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
            cli::OutputFormat::Pdf => {
                tracing::warn!("PDF output format not yet implemented");
                continue;
            }
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
    let target_path = PathBuf::from(target);
    let output_path = PathBuf::from(output_dir);
    std::fs::create_dir_all(&output_path).ok();

    let findings_jsonl_path = output_path.join("found_findings.jsonl");

    tracing::info!("Starting scan: {}", target);

    // Initialize cache
    let cache = if no_cache {
        cache::ScanCache::disabled()
    } else if cfg.cache.enabled {
        let c = cache::ScanCache::open(&cfg.cache.path)?;
        let _ = c.enforce_ttl(cfg.cache.ttl_hours);
        c
    } else {
        cache::ScanCache::disabled()
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
                }
            }
            6 => {
                scanners.push(Box::new(Checkov::with_binary(cfg.binaries.checkov.clone())));
            }
            7 => {
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

    // Resume: skip completed scanners
    if resume {
        match load_completed_scanners(&findings_jsonl_path) {
            Ok(completed) if !completed.is_empty() => {
                tracing::info!(
                    "Resume mode: {} scanners already completed: {:?}",
                    completed.len(),
                    completed
                );
                scanners.retain(|s| !completed.contains(s.name()));
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

    // Context Drift Detection (Layer 8)
    if args.context_drift {
        let drift_scanner = context_drift::ContextDriftScanner::new(&target_path);
        let drift_result = drift_scanner.scan_drift();

        match drift_result {
            context_drift::DriftScanResult::Complete {
                context_file_count,
                total_claims,
                drift_findings,
                drift_counts,
            } => {
                let canonical = context_drift::drift_findings_to_canonical(&drift_findings);
                let drift_count = canonical.len();

                if drift_count > 0 {
                    tracing::info!(
                        "Context drift: {} files, {} claims, {} drifts (C:{}, H:{}, M:{}, L:{}, I:{})",
                        context_file_count,
                        total_claims,
                        drift_count,
                        drift_counts.critical,
                        drift_counts.high,
                        drift_counts.medium,
                        drift_counts.low,
                        drift_counts.info,
                    );
                } else if total_claims > 0 {
                    tracing::info!(
                        "Context drift: {} files, {} claims verified — no drift detected",
                        context_file_count,
                        total_claims,
                    );
                } else {
                    tracing::info!(
                        "Context drift: {} context files found but no extractable claims",
                        context_file_count,
                    );
                }

                all_findings.extend(canonical);
            }
            context_drift::DriftScanResult::NoContextFiles => {
                tracing::info!(
                    "Context drift: no AGENTS.md, CLAUDE.md, or .cursor/rules files found"
                );
            }
            context_drift::DriftScanResult::NoClaims => {
                tracing::info!(
                    "Context drift: context files found but no claims could be extracted"
                );
            }
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
        reachability::apply_reachability(&mut final_findings, &reachability_result);
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
    }

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

    // Zero Trust scorecard
    let zt_scorecard = normalize::compute_zt_scorecard(&final_findings);

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

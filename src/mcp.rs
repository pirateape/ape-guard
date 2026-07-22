// ApeGuard MCP Server (Model Context Protocol)
// Exposes ApeGuard as MCP tools for AI pentest agents.
// Implements JSON-RPC 2.0 over stdio transport per the MCP specification.

use crate::cache::ScanCache;
use crate::find::*;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

fn load_effective_config() -> anyhow::Result<crate::config::Config> {
    let args = crate::cli::Args {
        command: crate::cli::Command::Serve,
        config: None,
        log_level: "info".to_string(),
        no_color: false,
        quiet: false,
        ci: false,
    };

    crate::config::load(&args)
}

fn summarize_findings_by_severity(findings: &[CanonicalFinding]) -> FindingsBySeverity {
    let mut by = FindingsBySeverity {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        info: 0,
    };

    for f in findings {
        match f.severity {
            Severity::Critical => by.critical += 1,
            Severity::High => by.high += 1,
            Severity::Medium => by.medium += 1,
            Severity::Low => by.low += 1,
            Severity::Info => by.info += 1,
        }
    }

    by
}

/// Load findings from the cache (latest scan)
fn load_cached_findings() -> anyhow::Result<Option<(String, Vec<CanonicalFinding>)>> {
    let cfg = load_effective_config()?;
    if !cfg.cache.enabled {
        return Ok(None);
    }
    let cache = ScanCache::open(&cfg.cache.path)?;
    let _ = cache.enforce_ttl(cfg.cache.ttl_hours);
    cache.get_latest_scan_findings()
}

/// Run the MCP server — reads JSON-RPC requests from stdin and writes responses to stdout.
pub async fn serve() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Send server info on startup (log line, not JSON-RPC)
    tracing::info!("ApeGuard MCP server starting on stdio");

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let response = match handle_request(&line).await {
            Ok(resp) => resp,
            Err(e) => json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32603,
                    "message": format!("Internal error: {}", e)
                },
                "id": null
            }),
        };

        let output = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", output)?;
        stdout.flush()?;
    }

    Ok(())
}

/// Handle a single JSON-RPC request line.
async fn handle_request(line: &str) -> anyhow::Result<Value> {
    let msg: Value = serde_json::from_str(line)?;

    let method = msg["method"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing method"))?
        .to_string();

    let id = &msg["id"];
    let params = msg.get("params").cloned().unwrap_or(json!({}));

    match method.as_str() {
        "initialize" => Ok(handle_initialize(id, &params)),
        "listTools" => Ok(handle_list_tools(id)),
        "callTool" => handle_call_tool(id, &params).await,
        "resources/list" => Ok(handle_resource_list(id)),
        "resources/read" => Ok(handle_resource_read(id, &params)),
        "notifications/initialized" => {
            // No response for notifications
            Ok(Value::Null)
        }
        _ => Ok(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", method)
            },
            "id": id
        })),
    }
}

/// Handle initialize request.
fn handle_initialize(id: &Value, params: &Value) -> Value {
    let _protocol_version = params["protocolVersion"].as_str().unwrap_or("unknown");
    // Support the current MCP specification version (2025-11-25)
    let supported_version = "2025-11-25";

    tracing::debug!(protocol_version = %_protocol_version, "MCP initialize request");

    // Per MCP spec, respond with the version we support. Client decides compatibility.

    json!({
        "jsonrpc": "2.0",
        "result": {
            "protocolVersion": supported_version,
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "serverInfo": {
                "name": "apeguard",
                "version": env!("CARGO_PKG_VERSION")
            }
        },
        "id": id
    })
}

/// Handle listTools request.
fn handle_list_tools(id: &Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "result": {
            "tools": [
                {
                    "name": "scan",
                    "description": "Run a full security scan on a target directory",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "Path to scan"
                            },
                            "layers": {
                                "type": "array",
                                "items": { "type": "number" },
                                "description": "Scanner layers (1=secrets, 2=SAST, 3=SCA fs, 4=container image, 5=DAST, 6=IaC Checkov, 7=SBOM Syft)"
                            },
                            "container": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Container images for layer 4 (e.g. [\"nginx:latest\"])"
                            },
                            "web": {
                                "type": "string",
                                "description": "Web URL target for layer 5 DAST (e.g. https://example.com)"
                            },
                            "severity": {
                                "type": "string",
                                "enum": ["all", "info", "low", "medium", "high", "critical"],
                                "description": "Minimum severity"
                            }
                        },
                        "required": ["target"]
                    }
                },
                {
                    "name": "findings",
                    "description": "Get all findings from the last scan",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "severity": {
                                "type": "string",
                                "enum": ["all", "critical", "high", "medium", "low", "info"],
                                "description": "Filter by severity"
                            },
                            "limit": {
                                "type": "number",
                                "description": "Max findings to return (default 50)"
                            }
                        }
                    }
                },
                {
                    "name": "scorecard",
                    "description": "Get the Zero Trust scorecard",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "chains",
                    "description": "Get attack chain analysis",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "arch_analysis",
                    "description": "Analyze architectural components and their risks",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "Project root directory (default: current dir)"
                            }
                        }
                    }
                }
            ]
        },
        "id": id
    })
}

/// Handle callTool request.
async fn handle_call_tool(id: &Value, params: &Value) -> anyhow::Result<Value> {
    let tool_name = params["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?
        .to_string();

    let args = params.get("arguments").cloned().unwrap_or(json!({}));

    let result = match tool_name.as_str() {
        "scan" => handle_scan_tool(&args).await?,
        "findings" => handle_findings_tool(&args).await?,
        "scorecard" => handle_scorecard_tool().await?,
        "chains" => handle_chains_tool().await?,
        "arch_analysis" => handle_arch_tool(&args).await?,
        _ => json!({
            "error": format!("Unknown tool: {}", tool_name)
        }),
    };

    // Serialize the result to JSON text
    let text = match serde_json::to_string_pretty(&result) {
        Ok(t) => t,
        Err(e) => {
            return Ok(json!({
                "jsonrpc": "2.0",
                "error": { "code": -32603, "message": format!("Serialization error: {}", e) },
                "id": id
            }));
        }
    };

    Ok(json!({
        "jsonrpc": "2.0",
        "result": { "content": [{ "type": "text", "text": text }] },
        "id": id
    }))
}

/// Handle the scan tool.
async fn handle_scan_tool(args: &Value) -> anyhow::Result<Value> {
    let target = args["target"].as_str().unwrap_or(".");

    let layers: Vec<u8> = args["layers"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect()
        })
        .unwrap_or_else(|| vec![1, 2, 3, 6, 7]);

    let web_target = args
        .get("web")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            args.get("web_target")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

    let mut container_images: Vec<String> = args
        .get("container")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    if container_images.is_empty() {
        if let Some(single) = args.get("container").and_then(|v| v.as_str()) {
            container_images.push(single.to_string());
        }
    }

    if container_images.is_empty() {
        container_images = args
            .get("containers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
    }

    if container_images.is_empty() {
        container_images = args
            .get("container_images")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
    }

    // Build config and open cache for persistence
    let cfg = load_effective_config()?;
    let cache = if cfg.cache.enabled {
        if let Ok(cache) = crate::cache::ScanCache::open(&cfg.cache.path) {
            let _ = cache.enforce_ttl(cfg.cache.ttl_hours);
            Some(cache)
        } else {
            None
        }
    } else {
        None
    };

    // Run scanners
    use crate::scanner::{
        aws_s3::AwsS3Scanner, checkov::Checkov, container::ContainerScanner, context_drift,
        dast::DastScanner, gitleaks::Gitleaks, mcp_security::McpScanner, semgrep::Semgrep,
        syft::Syft, terraform::TerraformScanner, tls::TlsScanner, trivy::Trivy,
        trufflehog::Trufflehog, Scanner, ScannerResult,
    };
    let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    for layer in &layers {
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
                scanners.push(Box::new(Trivy::with_mode(
                    crate::scanner::trivy::TrivyMode::Vuln,
                )));
                scanners.push(Box::new(Trivy::with_mode(
                    crate::scanner::trivy::TrivyMode::Secret,
                )));
                scanners.push(Box::new(Trivy::with_mode(
                    crate::scanner::trivy::TrivyMode::Misconfig,
                )));
            }
            4 => {
                if container_images.is_empty() {
                    warnings.push("Layer 4 requested but no container images provided. Use arguments.container=[\"image:tag\"].".to_string());
                } else {
                    for image in &container_images {
                        scanners.push(Box::new(ContainerScanner::new(image)));
                    }
                }
            }
            5 => {
                if let Some(url) = web_target.as_deref() {
                    scanners.push(Box::new(DastScanner::new(url)));
                } else {
                    warnings.push("Layer 5 requested but no web target provided. Use arguments.web=\"https://example.com\".".to_string());
                }
            }
            6 => {
                scanners.push(Box::new(Checkov::with_binary(cfg.binaries.checkov.clone())));
            }
            7 => {
                scanners.push(Box::new(Syft::with_binary(cfg.binaries.syft.clone())));
            }
            8 => {
                scanners.push(Box::new(context_drift::ContextDriftScanner::new(
                    std::path::Path::new("."),
                )));
            }
            9 => {
                scanners.push(Box::new(McpScanner::new(".apeguard/mcp-config.json")));
            }
            10 => {
                scanners.push(Box::new(TerraformScanner::new(".")));
            }
            11 => {
                scanners.push(Box::new(AwsS3Scanner::new(".apeguard/aws-config.json")));
            }
            12 => {
                scanners.push(Box::new(TlsScanner::new(&[
                    "/etc/ssl/certs/ca-certificates.crt",
                ])));
            }
            _ => {
                warnings.push(format!("Unknown layer: {}. Supported layers: 1-12.", layer));
            }
        }
    }

    let target_path = PathBuf::from(target);
    let mut all_findings: Vec<CanonicalFinding> = Vec::new();
    let mut scanners_used: Vec<String> = Vec::new();

    for s in &scanners {
        let name = s.name();
        scanners_used.push(name.to_string());
        match s.scan(&target_path).await {
            Ok(ScannerResult::Complete { findings, .. }) => {
                all_findings.extend(findings);
            }
            Ok(ScannerResult::NotInstalled { name, hint }) => {
                tracing::warn!("{} not installed: {}", name, hint);
            }
            Ok(ScannerResult::Error { name, error }) => {
                tracing::warn!("{} error: {}", name, error);
            }
            Err(e) => {
                tracing::warn!("{} failed: {}", name, e);
            }
        }
    }

    // Process findings
    crate::normalize::normalize_findings(&mut all_findings);
    crate::dedup::cross_reference(&mut all_findings);
    let deduped = crate::dedup::deduplicate(all_findings);

    // Build attack chains
    let chains = crate::chain::build_attack_chains(&deduped);

    // Build scorecard
    let scorecard = crate::normalize::compute_zt_scorecard(&deduped);

    // Persist to cache if available (makes MCP scan results available for report/compare)
    if let Some(ref cache) = cache {
        let scan_id = uuid::Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now().to_rfc3339();
        let completed_at = chrono::Utc::now().to_rfc3339();
        let _ = cache.record_scan(crate::cache::RecordScanInput {
            scan_id: &scan_id,
            target,
            started_at: &started_at,
            completed_at: &completed_at,
            total_findings: deduped.len() as u32,
            scanners_used: &scanners_used,
            findings: &deduped,
        });
    }

    Ok(json!({
        "target": target,
        "requested_layers": layers,
        "container_images": container_images,
        "web_target": web_target,
        "warnings": warnings,
        "total_findings": deduped.len(),
        "scanners_used": scanners_used,
        "attack_chains": chains.len(),
        "zt_score": scorecard.overall_score,
        "zt_max_score": scorecard.max_score,
        "findings": deduped.iter().map(|f| json!({
            "id": f.id,
            "scanner": format!("{:?}", f.scanner),
            "rule": f.rule_id,
            "severity": format!("{:?}", f.severity),
            "title": f.title,
            "file": f.location.file.to_string_lossy(),
            "line": f.location.line,
        })).collect::<Vec<_>>(),
    }))
}

/// Handle the findings tool — returns cached findings.
async fn handle_findings_tool(args: &Value) -> anyhow::Result<Value> {
    let cached = load_cached_findings()?;
    let (_, findings) = match cached {
        Some(s) => s,
        None => {
            return Ok(json!({
                "message": "No cached scan found. Run a scan first using the 'scan' tool.",
                "hint": "Use: {\"name\": \"scan\", \"arguments\": {\"target\": \"/path/to/project\"}}"
            }));
        }
    };

    // Apply severity filter
    let severity_filter = args["severity"].as_str().unwrap_or("all");
    let min_severity = match severity_filter {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        "info" => Severity::Info,
        _ => Severity::Info, // "all" — include everything
    };

    // Save total before potential move
    let unfiltered_total = &findings.len();

    let filtered: Vec<_> = if severity_filter == "all" {
        findings
    } else {
        findings
            .into_iter()
            .filter(|f| f.severity >= min_severity)
            .collect()
    };

    let limit = args["limit"].as_u64().unwrap_or(50) as usize;
    let truncated: Vec<_> = filtered.into_iter().take(limit).collect();

    Ok(json!({
        "total": truncated.len(),
        "unfiltered_total": unfiltered_total,
        "findings": truncated.iter().map(|f| json!({
            "id": f.id,
            "scanner": format!("{:?}", f.scanner),
            "rule": f.rule_id,
            "severity": format!("{:?}", f.severity),
            "title": f.title,
            "file": f.location.file.to_string_lossy(),
            "line": f.location.line,
            "cwe": f.cwe,
            "cvss": f.cvss,
            "zt_pillars": f.zt_pillars,
        })).collect::<Vec<_>>(),
    }))
}

/// Handle the scorecard tool.
async fn handle_scorecard_tool() -> anyhow::Result<Value> {
    let cached = load_cached_findings()?;
    let (_, findings) = match cached {
        Some(s) => s,
        None => {
            return Ok(json!({
                "message": "No cached scorecard. Run a scan first.",
                "hint": "Use the 'scan' tool first, then call 'scorecard' again."
            }));
        }
    };

    let scorecard = crate::normalize::compute_zt_scorecard(&findings);

    Ok(json!({
        "overall_score": scorecard.overall_score,
        "max_score": scorecard.max_score,
        "pillars_at_advanced_or_higher": scorecard.pillars_at_advanced_or_higher,
        "target_maturity": format!("{:?}", scorecard.target_maturity),
        "pillars": scorecard.pillars.iter().map(|p| json!({
            "name": p.name,
            "maturity": format!("{:?}", p.maturity),
            "gap_count": p.gap_count,
            "score": p.score,
        })).collect::<Vec<_>>(),
        "gap_analysis": scorecard.gap_analysis.iter().map(|g| json!({
            "pillar": g.pillar,
            "current_maturity": format!("{:?}", g.current_maturity),
            "target_maturity": format!("{:?}", g.target_maturity),
            "gap": format!("{:?}", g.gap),
            "blocking_findings": g.blocking_findings,
            "recommendations": g.recommendations,
        })).collect::<Vec<_>>(),
    }))
}

/// Handle the chains tool.
async fn handle_chains_tool() -> anyhow::Result<Value> {
    let cached = load_cached_findings()?;
    let (_, findings) = match cached {
        Some(s) => s,
        None => {
            return Ok(json!({
                "message": "No cached attack chains. Run a scan first.",
                "hint": "Use the 'scan' tool first, then call 'chains' again."
            }));
        }
    };

    let chains = crate::chain::build_attack_chains(&findings);

    Ok(json!({
        "total": chains.len(),
        "chains": chains.iter().map(|c| json!({
            "id": c.id,
            "risk_score": c.risk_score,
            "description": c.description,
            "steps": c.steps,
            "finding_ids": c.finding_ids,
            "recommendation": c.recommendation,
        })).collect::<Vec<_>>(),
    }))
}

/// Handle the architecture analysis tool.
async fn handle_arch_tool(args: &Value) -> anyhow::Result<Value> {
    let target = args["target"].as_str().unwrap_or(".");
    let target_path = PathBuf::from(target);

    let artifacts = crate::arch::discover_artifacts(&target_path);

    let cached_findings = load_cached_findings()?
        .map(|(_, findings)| findings)
        .unwrap_or_default();

    let component_risks = if cached_findings.is_empty() {
        Vec::new()
    } else {
        crate::arch::assess_component_risks(&cached_findings, &artifacts)
    };

    let arch_diagram = if !artifacts.is_empty() && !component_risks.is_empty() {
        Some(crate::arch::generate_mermaid_diagram(
            &artifacts,
            &component_risks,
        ))
    } else {
        None
    };

    Ok(json!({
        "target": target,
        "artifacts_found": artifacts.len(),
        "component_risks_found": component_risks.len(),
        "arch_diagram": arch_diagram,
        "artifacts": artifacts.iter().map(|a| json!({
            "path": a.path.to_string_lossy(),
            "type": format!("{:?}", a.artifact_type),
            "summary": a.content_summary,
            "components": a.components,
            "dependencies": a.dependencies,
            "decisions": a.decisions.len(),
        })).collect::<Vec<_>>(),
        "component_risks": component_risks.iter().map(|r| json!({
            "component_name": r.component_name,
            "finding_count": r.finding_count,
            "critical_count": r.critical_count,
            "high_count": r.high_count,
            "risk_level": format!("{:?}", r.risk_level),
            "recommendations": r.recommendations,
        })).collect::<Vec<_>>(),
    }))
}

/// Handle resources/list request.
fn handle_resource_list(id: &Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "result": {
            "resources": [
                {
                    "uri": "apeguard://reports/latest",
                    "name": "Latest scan report",
                    "description": "The most recent security scan report in markdown",
                    "mimeType": "text/markdown"
                },
                {
                    "uri": "apeguard://scorecard/latest",
                    "name": "Latest Zero Trust scorecard",
                    "description": "Zero Trust maturity scorecard from the latest scan",
                    "mimeType": "application/json"
                }
            ]
        },
        "id": id
    })
}

/// Handle resources/read request.
fn handle_resource_read(id: &Value, params: &Value) -> Value {
    let uri = params.get("uri").and_then(|v| v.as_str()).or_else(|| {
        params
            .get("arguments")
            .and_then(|a| a.get("uri"))
            .and_then(|v| v.as_str())
    });

    let Some(uri) = uri else {
        return json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32602,
                "message": "Missing required parameter: uri"
            },
            "id": id
        });
    };

    let cached = match load_cached_findings() {
        Ok(c) => c,
        Err(e) => {
            return json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32603,
                    "message": format!("Failed to load cached findings: {}", e)
                },
                "id": id
            });
        }
    };

    match uri {
        "apeguard://reports/latest" => {
            let text = if let Some((scan_id, findings)) = cached {
                let by_sev = summarize_findings_by_severity(&findings);
                let top: Vec<String> = findings
                    .iter()
                    .take(20)
                    .map(|f| {
                        format!(
                            "- **{:?}** {} (`{}`) at `{}`{}",
                            f.severity,
                            f.title,
                            f.rule_id,
                            f.location.file.to_string_lossy(),
                            f.location
                                .line
                                .map(|l| format!(":{}", l))
                                .unwrap_or_default()
                        )
                    })
                    .collect();

                format!(
                    "# ApeGuard Latest Scan Report\n\n- Scan ID: {}\n- Total findings: {}\n\n## Findings by severity\n\n- Critical: {}\n- High: {}\n- Medium: {}\n- Low: {}\n- Info: {}\n\n## Top findings\n\n{}\n",
                    scan_id,
                    findings.len(),
                    by_sev.critical,
                    by_sev.high,
                    by_sev.medium,
                    by_sev.low,
                    by_sev.info,
                    if top.is_empty() {
                        "- No findings".to_string()
                    } else {
                        top.join("\n")
                    }
                )
            } else {
                "No cached scan found. Run scan tool first.".to_string()
            };

            json!({
                "jsonrpc": "2.0",
                "result": {
                    "contents": [{
                        "uri": uri,
                        "mimeType": "text/markdown",
                        "text": text
                    }]
                },
                "id": id
            })
        }
        "apeguard://scorecard/latest" => {
            let text = if let Some((scan_id, findings)) = cached {
                let scorecard = crate::normalize::compute_zt_scorecard(&findings);
                serde_json::to_string_pretty(&json!({
                    "scan_id": scan_id,
                    "scorecard": scorecard,
                }))
                .unwrap_or_else(|_| "{\"error\":\"failed to serialize scorecard\"}".to_string())
            } else {
                serde_json::to_string_pretty(&json!({
                    "message": "No cached scan found. Run scan tool first."
                }))
                .unwrap_or_else(|_| {
                    "{\"message\":\"No cached scan found. Run scan tool first.\"}".to_string()
                })
            };

            json!({
                "jsonrpc": "2.0",
                "result": {
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": text
                    }]
                },
                "id": id
            })
        }
        _ => json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32602,
                "message": format!("Unknown resource URI: {}", uri)
            },
            "id": id
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_response() {
        let resp = handle_initialize(&json!(1), &json!({"protocolVersion": "2025-11-25"}));
        assert_eq!(resp["result"]["serverInfo"]["name"], "apeguard");
        assert!(resp["result"]["capabilities"]["tools"].is_object());
    }

    #[test]
    fn test_list_tools_response() {
        let resp = handle_list_tools(&json!(1));
        let tools = resp["result"]["tools"].as_array().unwrap();
        let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
        assert!(tool_names.contains(&"scan"));
        assert!(tool_names.contains(&"findings"));
        assert!(tool_names.contains(&"scorecard"));
        assert!(tool_names.contains(&"chains"));
        assert!(tool_names.contains(&"arch_analysis"));
    }

    #[test]
    fn test_resource_list() {
        let resp = handle_resource_list(&json!(1));
        let resources = resp["result"]["resources"].as_array().unwrap();
        assert!(resources
            .iter()
            .any(|r| r["uri"].as_str().unwrap_or("").contains("reports")));
    }

    #[test]
    fn test_resource_read_missing_uri() {
        let resp = handle_resource_read(&json!(1), &json!({}));
        assert!(resp.get("error").is_some());
        assert_eq!(resp["error"]["code"], -32602);
    }

    #[test]
    fn test_resource_read_unknown_uri() {
        let resp = handle_resource_read(&json!(1), &json!({"uri":"apeguard://unknown"}));
        assert!(resp.get("error").is_some());
        assert_eq!(resp["error"]["code"], -32602);
    }

    #[test]
    fn test_handle_resources_read_valid() {
        let line = r#"{"jsonrpc":"2.0","method":"resources/read","params":{"uri":"apeguard://scorecard/latest"},"id":1}"#;
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(handle_request(line));
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp["result"]["contents"].is_array());
    }

    #[test]
    fn test_handle_initialize_valid() {
        let line = r#"{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2025-11-25"},"id":1}"#;
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(handle_request(line));
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp["result"]["serverInfo"]["name"], "apeguard");
    }

    #[test]
    fn test_handle_list_tools_valid() {
        let line = r#"{"jsonrpc":"2.0","method":"listTools","id":1}"#;
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(handle_request(line));
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp["result"]["tools"].is_array());
    }

    #[test]
    fn test_handle_unknown_method() {
        let line = r#"{"jsonrpc":"2.0","method":"unknownMethod","id":1}"#;
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(handle_request(line));
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.get("error").is_some());
        assert_eq!(resp["error"]["code"], -32601);
    }

    #[test]
    fn test_handle_invalid_json() {
        let line = "not json";
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(handle_request(line));
        assert!(result.is_err());
    }
}

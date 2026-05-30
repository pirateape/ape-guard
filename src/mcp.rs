// ApeGuard MCP Server (Model Context Protocol)
// Exposes ApeGuard as MCP tools for AI pentest agents.
// Implements JSON-RPC 2.0 over stdio transport per the MCP specification.

use crate::find::*;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

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
        "initialize" => Ok(handle_initialize(id)),
        "listTools" => Ok(handle_list_tools(id)),
        "callTool" => handle_call_tool(id, &params).await,
        "resources/list" => Ok(handle_resource_list(id)),
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
fn handle_initialize(id: &Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "result": {
            "protocolVersion": "2025-03-26",
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
                                "description": "Scanner layers (1=secrets, 2=SAST, 3=SCA)"
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
    let target = args["target"]
        .as_str()
        .unwrap_or(".");
    let layers: Vec<u8> = args["layers"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
        .unwrap_or_else(|| vec![1, 2, 3]);

    // Build config
    let cfg = crate::config::Config::default();

    // Run scanners
    use crate::scanner::{Scanner, ScannerResult, gitleaks::Gitleaks, semgrep::Semgrep, trivy::Trivy};
    let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();
    for layer in &layers {
        match layer {
            1 => scanners.push(Box::new(Gitleaks::new())),
            2 => scanners.push(Box::new(Semgrep::new())),
            3 => {
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Vuln)));
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Secret)));
                scanners.push(Box::new(Trivy::with_mode(crate::scanner::trivy::TrivyMode::Misconfig)));
            }
            _ => {}
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

    Ok(json!({
        "target": target,
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
    // In a full implementation, this would query the cache.
    // For now, return a message indicating no cached scan.
    Ok(json!({
        "message": "No cached scan found. Run a scan first using the 'scan' tool.",
        "hint": "Use: {\"name\": \"scan\", \"arguments\": {\"target\": \"/path/to/project\"}}"
    }))
}

/// Handle the scorecard tool.
async fn handle_scorecard_tool() -> anyhow::Result<Value> {
    Ok(json!({
        "message": "No cached scorecard. Run a scan first.",
        "hint": "Use the 'scan' tool first, then call 'scorecard' again."
    }))
}

/// Handle the chains tool.
async fn handle_chains_tool() -> anyhow::Result<Value> {
    Ok(json!({
        "message": "No cached attack chains. Run a scan first.",
        "hint": "Use the 'scan' tool first, then call 'chains' again."
    }))
}

/// Handle the architecture analysis tool.
async fn handle_arch_tool(args: &Value) -> anyhow::Result<Value> {
    let target = args["target"]
        .as_str()
        .unwrap_or(".");
    let target_path = PathBuf::from(target);

    let artifacts = crate::arch::discover_artifacts(&target_path);

    Ok(json!({
        "target": target,
        "artifacts_found": artifacts.len(),
        "artifacts": artifacts.iter().map(|a| json!({
            "path": a.path.to_string_lossy(),
            "type": format!("{:?}", a.artifact_type),
            "components": a.components,
            "decisions": a.decisions.len(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_response() {
        let resp = handle_initialize(&json!(1));
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
        assert!(resources.iter().any(|r| r["uri"].as_str().unwrap_or("").contains("reports")));
    }

    #[test]
    fn test_handle_initialize_valid() {
        let line = r#"{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}"#;
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

// ApeGuard LLM Enhancement Module
// Feeds findings into a local LLM (via Ollam) to generate rich, context-aware remediation.
// Optional — skips silently if Ollama is not running.
use crate::find::CanonicalFinding;
use serde::Deserialize;

/// Configuration for LLM enhancement
pub struct LlmConfig {
    /// Ollama endpoint (default: http://localhost:11434)
    pub endpoint: String,
    /// Model name (default: codellama or deepseek-coder)
    pub model: String,
    /// Whether LLM enhancement is enabled
    pub enabled: bool,
}

impl Default for LlmConfig {
    fn default() -> Self {
        LlmConfig {
            endpoint: "http://localhost:11434".to_string(),
            model: "codellama".to_string(),
            enabled: true,
        }
    }
}

/// Generate AI-powered remediation for a batch of findings
pub async fn enhance_remediations(
    findings: &mut [CanonicalFinding],
    config: &LlmConfig,
) -> anyhow::Result<u32> {
    if !config.enabled {
        return Ok(0);
    }

    // Check if Ollama is available
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let ping = client
        .head(format!("{}/api/tags", config.endpoint))
        .send()
        .await;

    if ping.is_err() {
        tracing::warn!(
            "Ollama not available at {} — skipping LLM enhancement",
            config.endpoint
        );
        return Ok(0);
    }

    let mut enhanced = 0u32;

    for finding in findings.iter_mut() {
        // Skip findings that already have detailed remediation
        if let Some(ref remediation) = finding.remediation {
            if remediation.len() > 40 {
                continue;
            }
        }

        // Build prompt for the LLM
        let prompt = build_remediation_prompt(finding);

        // Call Ollama API
        match call_ollama(&config.endpoint, &config.model, &prompt).await {
            Ok(response) => {
                finding.remediation = Some(response.trim().to_string());
                enhanced += 1;
            }
            Err(e) => {
                tracing::debug!("LLM remediation failed for {}: {}", finding.id, e);
            }
        }
    }

    Ok(enhanced)
}

fn build_remediation_prompt(finding: &CanonicalFinding) -> String {
    format!(
        "You are a security engineer. Generate a concise, actionable remediation fix for the following security finding.\n\
         Keep it under 200 characters. Focus on the specific code change needed.\n\n\
         Rule: {}\n\
         Severity: {:?}\n\
         Description: {}\n\
         File: {}\n",
        finding.rule_id,
        finding.severity,
        finding.description,
        finding.location.file.display(),
    )
}

pub(crate) async fn call_ollama(
    endpoint: &str,
    model: &str,
    prompt: &str,
) -> anyhow::Result<String> {
    #[derive(serde::Serialize)]
    struct OllamaRequest<'a> {
        model: &'a str,
        prompt: &'a str,
        stream: bool,
    }

    #[derive(Deserialize)]
    struct OllamaResponse {
        response: String,
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let resp = client
        .post(format!("{}/api/generate", endpoint))
        .json(&OllamaRequest {
            model,
            prompt,
            stream: false,
        })
        .send()
        .await?;

    let body: OllamaResponse = resp.json().await?;
    Ok(body.response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity};
    use std::path::PathBuf;

    #[test]
    fn test_llm_config_default() {
        let cfg = LlmConfig::default();
        assert_eq!(cfg.endpoint, "http://localhost:11434");
        assert_eq!(cfg.model, "codellama");
        assert!(cfg.enabled);
    }

    #[tokio::test]
    async fn test_enhance_remediations_disabled() {
        let mut findings = vec![CanonicalFinding {
            id: "F-1".to_string(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "rule-1".to_string(),
            severity: Severity::High,
            confidence: Confidence::Certain,
            title: "Title".to_string(),
            description: "Desc".to_string(),
            location: FindingLocation {
                file: PathBuf::from("test.txt"),
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
        }];
        let cfg = LlmConfig {
            enabled: false,
            ..LlmConfig::default()
        };
        let count = enhance_remediations(&mut findings, &cfg).await.unwrap();
        assert_eq!(count, 0);
        assert!(findings[0].remediation.is_none());
    }

    #[test]
    fn test_build_remediation_prompt() {
        let finding = CanonicalFinding {
            id: "F-1".to_string(),
            scanner: ScannerType::Gitleaks,
            scanner_version: None,
            rule_id: "rule-1".to_string(),
            severity: Severity::High,
            confidence: Confidence::Certain,
            title: "Title".to_string(),
            description: "Description text".to_string(),
            location: FindingLocation {
                file: PathBuf::from("test.txt"),
                line: Some(10),
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
        };
        let prompt = build_remediation_prompt(&finding);
        assert!(prompt.contains("rule-1"));
        assert!(prompt.contains("High"));
        assert!(prompt.contains("Description text"));
        assert!(prompt.contains("test.txt"));
    }
}

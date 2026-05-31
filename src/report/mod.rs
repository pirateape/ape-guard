// ApeGuard Report Generator
// Produces three types of reports from scanning findings:
//   - Technical: detailed findings for engineers
//   - Executive: risk posture summary for leadership
//   - Roadmap: prioritized remediation plan for engineering managers
// Also supports JSON and SARIF output formats.
use crate::find::{CanonicalFinding, ScanSummary, ZeroTrustScorecard};
use serde_json::json;
use std::path::Path;
use tera::{Context, Tera};

/// Report type enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportType {
    Technical,
    Executive,
    Roadmap,
}

impl ReportType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReportType::Technical => "technical",
            ReportType::Executive => "executive",
            ReportType::Roadmap => "roadmap",
        }
    }
}

/// Generate all three reports from scan results.
/// `report_types`: which report types to generate; pass an empty slice to generate all.
pub fn generate_all_reports(
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    arch_diagram: Option<&str>,
    report_types: &[ReportType],
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let mut generated = Vec::new();

    let all_types = [
        ReportType::Technical,
        ReportType::Executive,
        ReportType::Roadmap,
    ];
    let types_to_run: &[ReportType] = if report_types.is_empty() {
        &all_types
    } else {
        report_types
    };

    for rtype in types_to_run {
        let path = generate_report(
            rtype,
            summary,
            findings,
            zt_scorecard,
            output_dir,
            arch_diagram,
        )?;
        generated.push(path);
    }

    Ok(generated)
}

/// Generate a single report
pub fn generate_report(
    report_type: &ReportType,
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    arch_diagram: Option<&str>,
) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    let mut tera = Tera::default();

    // Register the template for this report type
    let template_name = match report_type {
        ReportType::Technical => "technical.md",
        ReportType::Executive => "executive.md",
        ReportType::Roadmap => "roadmap.md",
    };

    let template_content = get_template(report_type);
    tera.add_raw_template(template_name, template_content)?;

    // Build context
    let mut context = Context::new();
    context.insert("scan_id", &summary.scan_id);
    context.insert("timestamp", &summary.timestamp);
    context.insert("target", &summary.target);
    context.insert("duration_seconds", &summary.duration_seconds);
    context.insert("total_findings", &summary.total_findings);
    context.insert("findings_by_severity", &summary.findings_by_severity);
    context.insert("scanners_used", &summary.scanners_used);
    context.insert("zt_scorecard", zt_scorecard);
    context.insert("arch_diagram", &arch_diagram.unwrap_or(""));

    // Enrich findings with MITRE mapping
    let enriched_findings: Vec<_> = findings
        .iter()
        .map(|f| {
            let mitre = crate::normalize::mitre_mapping(f);
            EnrichedFinding {
                id: f.id.clone(),
                scanner: format!("{:?}", f.scanner),
                rule_id: f.rule_id.clone(),
                severity: format!("{:?}", f.severity),
                title: f.title.clone(),
                description: f.description.clone(),
                file: f.location.file.to_string_lossy().to_string(),
                line: f.location.line,
                cwe: f.cwe.clone(),
                cvss: f.cvss,
                remediation: f.remediation.clone(),
                zt_pillars: f.zt_pillars.clone(),
                mitre_tactics: mitre,
                cross_refs: f
                    .cross_refs
                    .iter()
                    .map(|cr| format!("{:?}/{}", cr.scanner, cr.rule_id))
                    .collect(),
            }
        })
        .collect();

    context.insert("findings", &enriched_findings);

    let rendered = tera.render(template_name, &context)?;

    let filename = format!("{}-report.md", report_type.as_str());
    let output_path = output_dir.join(&filename);
    std::fs::write(&output_path, rendered)?;

    tracing::info!(
        "Generated {} report: {}",
        report_type.as_str(),
        output_path.display()
    );

    Ok(output_path)
}

#[derive(Debug, serde::Serialize)]
struct EnrichedFinding {
    id: String,
    scanner: String,
    rule_id: String,
    severity: String,
    title: String,
    description: String,
    file: String,
    line: Option<u32>,
    cwe: Option<String>,
    cvss: Option<f32>,
    remediation: Option<String>,
    zt_pillars: Vec<String>,
    mitre_tactics: Vec<String>,
    cross_refs: Vec<String>,
}

fn get_template(rtype: &ReportType) -> &'static str {
    match rtype {
        ReportType::Technical => TECHNICAL_TEMPLATE,
        ReportType::Executive => EXECUTIVE_TEMPLATE,
        ReportType::Roadmap => ROADMAP_TEMPLATE,
    }
}

const TECHNICAL_TEMPLATE: &str = r#"---
title: ApeGuard Technical Report
scan_id: {{ scan_id }}
date: {{ timestamp }}
target: {{ target }}
---

# ApeGuard Technical Report

## Summary

| Metric | Value |
|--------|-------|
| **Target** | {{ target }} |
| **Scan ID** | {{ scan_id }} |
| **Duration** | {{ duration_seconds }}s |
| **Scanners** | {{ scanners_used | join(sep=", ") }} |
| **Total Findings** | {{ total_findings }} |

### Findings by Severity

| Severity | Count |
|----------|:-----:|
| Critical | {{ findings_by_severity.critical }} |
| High | {{ findings_by_severity.high }} |
| Medium | {{ findings_by_severity.medium }} |
| Low | {{ findings_by_severity.low }} |
| Info | {{ findings_by_severity.info }} |

{% if zt_scorecard %}
## Zero Trust Scorecard

**Overall Score:** {{ zt_scorecard.overall_score }} / {{ zt_scorecard.max_score }}
**Pillars at Advanced+:** {{ zt_scorecard.pillars_at_advanced_or_higher }}
**Target Maturity:** {{ zt_scorecard.target_maturity }}

| Pillar | Maturity | Gaps | Score |
|--------|----------|:----:|:----:|
{% for pillar in zt_scorecard.pillars %}
| {{ pillar.name }} | {{ pillar.maturity }} | {{ pillar.gap_count }} | {{ pillar.score }} |
{% endfor %}
{% endif %}

## Detailed Findings

{% for finding in findings %}
### {{ finding.severity }}: {{ finding.title }}
- **Scanner:** {{ finding.scanner }}
- **Rule:** `{{ finding.rule_id }}`
- **File:** `{{ finding.file }}`{% if finding.line %}:{{ finding.line }}{% endif %}
{% if finding.cvss %}**CVSS:** {{ finding.cvss }}{% endif %}
{% if finding.cwe %}**CWE:** {{ finding.cwe }}{% endif %}
- **Description:** {{ finding.description }}
{% if finding.remediation %}**Remediation:** {{ finding.remediation }}{% endif %}
{% if finding.zt_pillars %}**ZT Pillars:** {{ finding.zt_pillars | join(sep=", ") }}{% endif %}
{% if finding.mitre_tactics %}**MITRE:** {{ finding.mitre_tactics | join(sep=", ") }}{% endif %}

{% endfor %}

{% if arch_diagram %}
## Architecture Risk Diagram

{{ arch_diagram }}
{% endif %}
"#;

const EXECUTIVE_TEMPLATE: &str = r#"---
title: ApeGuard Executive Report
scan_id: {{ scan_id }}
date: {{ timestamp }}
target: {{ target }}
---

# ApeGuard Executive Security Report

**Prepared for:** {{ target }}
**Date:** {{ timestamp }}
**Scan ID:** {{ scan_id }}

## Risk Overview

{% if zt_scorecard %}
Your **Zero Trust maturity score** is **{{ zt_scorecard.overall_score }} / {{ zt_scorecard.max_score }}**.
{{ zt_scorecard.pillars_at_advanced_or_higher }} of {{ zt_scorecard.pillars | length }} pillars are at Advanced maturity or higher.

### Maturity by Pillar

| Pillar | Maturity | Score |
|--------|----------|:----:|
{% for pillar in zt_scorecard.pillars %}
| {{ pillar.name }} | {{ pillar.maturity }} | {{ pillar.score }}/100 |
{% endfor %}
{% endif %}

## Finding Summary

| Severity | Count | Action |
|----------|:-----:|--------|
| Critical | {{ findings_by_severity.critical }} | Immediate remediation required |
| High | {{ findings_by_severity.high }} | Address within current sprint |
| Medium | {{ findings_by_severity.medium }} | Schedule for next iteration |
| Low | {{ findings_by_severity.low }} | Monitor or accept risk |

**Total:** {{ total_findings }} findings across {{ scanners_used | length }} scanners.

## Top Findings

{% for finding in findings | slice(end=10) %}
- **{{ finding.severity }}** — {{ finding.title }} ({{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %})
{% endfor %}
"#;

const ROADMAP_TEMPLATE: &str = r#"---
title: ApeGuard Remediation Roadmap
scan_id: {{ scan_id }}
date: {{ timestamp }}
target: {{ target }}
---

# ApeGuard Remediation Roadmap

## Immediate (Critical — 24-48h)

{% for finding in findings | filter(attribute="severity", value="Critical") %}
- [ ] **{{ finding.title }}** — {{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %}
  - **Scanner:** {{ finding.scanner }}
  - {% if finding.remediation %}{{ finding.remediation }}{% endif %}
{% else %}
*No critical findings.*
{% endfor %}

## Short-term (High — This Sprint)

{% for finding in findings | filter(attribute="severity", value="High") %}
- [ ] **{{ finding.title }}** — {{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %}
  - {% if finding.remediation %}{{ finding.remediation }}{% endif %}
{% else %}
*No high-severity findings.*
{% endfor %}

{% if arch_diagram %}
## Architecture Risk Diagram

{{ arch_diagram }}
{% endif %}
"#;

/// Generate JSON report output
pub fn generate_json_report(
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    arch_diagram: Option<&str>,
) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    #[derive(serde::Serialize)]
    struct JsonReport<'a> {
        summary: &'a ScanSummary,
        scorecard: &'a ZeroTrustScorecard,
        arch_diagram: Option<&'a str>,
        findings: Vec<JsonFinding<'a>>,
    }

    #[derive(serde::Serialize)]
    struct JsonFinding<'a> {
        id: &'a str,
        scanner: String,
        rule_id: &'a str,
        severity: String,
        title: &'a str,
        description: &'a str,
        file: String,
        line: Option<u32>,
        cwe: Option<&'a str>,
        cvss: Option<f32>,
        remediation: Option<&'a str>,
        zt_pillars: &'a [String],
    }

    let json_findings: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            id: &f.id,
            scanner: format!("{:?}", f.scanner),
            rule_id: &f.rule_id,
            severity: format!("{:?}", f.severity),
            title: &f.title,
            description: &f.description,
            file: f.location.file.to_string_lossy().to_string(),
            line: f.location.line,
            cwe: f.cwe.as_deref(),
            cvss: f.cvss,
            remediation: f.remediation.as_deref(),
            zt_pillars: &f.zt_pillars,
        })
        .collect();

    let report = JsonReport {
        summary,
        scorecard: zt_scorecard,
        arch_diagram,
        findings: json_findings,
    };

    let output_path = output_dir.join("apeguard-report.json");
    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write(&output_path, json)?;
    tracing::info!("Generated JSON report: {}", output_path.display());

    Ok(output_path)
}

/// Generate SARIF 2.1.0 report output (Static Analysis Results Interchange Format)
pub fn generate_sarif_report(
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    arch_diagram: Option<&str>,
) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    // Build SARIF 2.1.0 compliant structure
    // Reference: https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html
    #[derive(serde::Serialize)]
    struct SarifReport {
        #[serde(rename = "$schema")]
        schema: String,
        version: String,
        runs: Vec<SarifRun>,
    }

    #[derive(serde::Serialize)]
    struct SarifRun {
        tool: SarifTool,
        results: Vec<SarifResult>,
        properties: serde_json::Value,
    }

    #[derive(serde::Serialize)]
    struct SarifTool {
        driver: SarifDriver,
    }

    #[derive(serde::Serialize)]
    struct SarifDriver {
        name: String,
        version: String,
        information_uri: String,
        rules: Vec<SarifRule>,
    }

    #[derive(serde::Serialize)]
    struct SarifRule {
        id: String,
        name: Option<String>,
        short_description: Option<SarifMessage>,
        full_description: Option<SarifMessage>,
        default_configuration: SarifDefaultConfig,
        properties: Option<serde_json::Value>,
    }

    #[derive(serde::Serialize)]
    struct SarifDefaultConfig {
        level: String,
    }

    #[derive(serde::Serialize)]
    struct SarifMessage {
        text: String,
    }

    #[derive(serde::Serialize)]
    struct SarifResult {
        rule_id: String,
        rule_index: usize,
        level: String,
        message: SarifMessage,
        locations: Vec<SarifLocation>,
        properties: Option<serde_json::Value>,
    }

    #[derive(serde::Serialize)]
    struct SarifLocation {
        physical_location: SarifPhysicalLocation,
    }

    #[derive(serde::Serialize)]
    struct SarifPhysicalLocation {
        artifact_location: SarifArtifactLocation,
        region: Option<SarifRegion>,
    }

    #[derive(serde::Serialize)]
    struct SarifArtifactLocation {
        uri: String,
    }

    #[derive(serde::Serialize)]
    struct SarifRegion {
        start_line: usize,
    }

    // Collect unique rules
    let mut rule_ids: Vec<&str> = findings.iter().map(|f| f.rule_id.as_str()).collect();
    rule_ids.sort();
    rule_ids.dedup();

    let rules: Vec<SarifRule> = rule_ids
        .iter()
        .map(|id| {
            let f = findings.iter().find(|f| f.rule_id == *id).unwrap();
            let severity_str = format!("{:?}", f.severity);
            let level = match f.severity {
                crate::find::Severity::Critical | crate::find::Severity::High => "error",
                crate::find::Severity::Medium => "warning",
                crate::find::Severity::Low | crate::find::Severity::Info => "note",
            };
            SarifRule {
                id: format!(
                    "apeguard/{}/{}",
                    format!("{:?}", f.scanner).to_lowercase(),
                    id
                ),
                name: Some(f.title.clone()),
                short_description: Some(SarifMessage {
                    text: f.description.clone(),
                }),
                full_description: None,
                default_configuration: SarifDefaultConfig {
                    level: level.to_string(),
                },
                properties: Some(json!({
                    "severity": severity_str,
                    "scanner": format!("{:?}", f.scanner),
                    "zt_pillars": f.zt_pillars,
                })),
            }
        })
        .collect();

    let results: Vec<SarifResult> = findings
        .iter()
        .map(|f| {
            let rule_index = rule_ids
                .iter()
                .position(|r| *r == f.rule_id.as_str())
                .unwrap_or(0);
            let level = match f.severity {
                crate::find::Severity::Critical | crate::find::Severity::High => "error",
                crate::find::Severity::Medium => "warning",
                crate::find::Severity::Low | crate::find::Severity::Info => "note",
            };
            SarifResult {
                rule_id: f.rule_id.clone(),
                rule_index,
                level: level.to_string(),
                message: SarifMessage {
                    text: format!("{:?}: {}", f.severity, f.title),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: f.location.file.to_string_lossy().to_string(),
                        },
                        region: f.location.line.map(|l| SarifRegion {
                            start_line: l as usize,
                        }),
                    },
                }],
                properties: Some(json!({
                    "scanner": format!("{:?}", f.scanner),
                    "zt_pillars": f.zt_pillars,
                    "cwe": f.cwe,
                    "cvss": f.cvss,
                })),
            }
        })
        .collect();

    let report = SarifReport {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "ApeGuard".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    information_uri: "https://github.com/pirateape/apeguard".to_string(),
                    rules,
                },
            },
            results,
            properties: json!({
                "apeguard": {
                    "zt_scorecard": zt_scorecard,
                    "scan_target": summary.target,
                    "scan_id": summary.scan_id,
                    "arch_diagram": arch_diagram,
                }
            }),
        }],
    };

    let output_path = output_dir.join("apeguard-report.sarif");
    let json = serde_json::to_string_pretty(&report)?;
    std::fs::write(&output_path, json)?;
    tracing::info!("Generated SARIF report: {}", output_path.display());

    Ok(output_path)
}

/// Generate a self-contained HTML report with inline CSS
pub fn generate_html_report(
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
    arch_diagram: Option<&str>,
) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    let mut tera = Tera::default();
    tera.add_raw_template("report.html", HTML_TEMPLATE)?;

    // Enrich findings for template
    let enriched_findings: Vec<_> = findings
        .iter()
        .map(|f| {
            let mitre = crate::normalize::mitre_mapping(f);
            EnrichedFinding {
                id: f.id.clone(),
                scanner: format!("{:?}", f.scanner),
                rule_id: f.rule_id.clone(),
                severity: format!("{:?}", f.severity),
                title: f.title.clone(),
                description: f.description.clone(),
                file: f.location.file.to_string_lossy().to_string(),
                line: f.location.line,
                cwe: f.cwe.clone(),
                cvss: f.cvss,
                remediation: f.remediation.clone(),
                zt_pillars: f.zt_pillars.clone(),
                mitre_tactics: mitre,
                cross_refs: f
                    .cross_refs
                    .iter()
                    .map(|cr| format!("{:?}/{}", cr.scanner, cr.rule_id))
                    .collect(),
            }
        })
        .collect();

    let mut context = Context::new();
    context.insert("scan_id", &summary.scan_id);
    context.insert("timestamp", &summary.timestamp);
    context.insert("target", &summary.target);
    context.insert("duration_seconds", &summary.duration_seconds);
    context.insert("total_findings", &summary.total_findings);
    context.insert("findings_by_severity", &summary.findings_by_severity);
    context.insert("scanners_used", &summary.scanners_used);
    context.insert("zt_scorecard", zt_scorecard);
    context.insert("findings", &enriched_findings);
    context.insert("arch_diagram", &arch_diagram.unwrap_or(""));

    let rendered = tera.render("report.html", &context)?;

    let output_path = output_dir.join("apeguard-report.html");
    std::fs::write(&output_path, rendered)?;
    tracing::info!("Generated HTML report: {}", output_path.display());

    Ok(output_path)
}

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>ApeGuard Report — {{ target }}</title>
<style>
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
  :root {
    --bg: #0d1117; --surface: #161b22; --border: #30363d;
    --text: #e6edf3; --text-muted: #8b949e; --accent: #58a6ff;
    --critical: #f85149; --high: #d29922; --medium: #a371f7;
    --low: #58a6ff; --info: #8b949e;
    --success: #3fb950; --card-radius: 8px;
  }
  body { font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif; background: var(--bg); color: var(--text); line-height: 1.6; padding: 0; }
  .container { max-width: 1100px; margin: 0 auto; padding: 32px 20px; }
  header { border-bottom: 1px solid var(--border); padding-bottom: 24px; margin-bottom: 32px; }
  header h1 { font-size: 1.8rem; color: var(--accent); margin-bottom: 8px; display: flex; align-items: center; gap: 10px; }
  header .shield { width: 32px; height: 32px; background: var(--accent); border-radius: 6px; display: inline-flex; align-items: center; justify-content: center; font-size: 18px; color: #fff; }
  .meta { color: var(--text-muted); font-size: 0.9rem; display: flex; flex-wrap: wrap; gap: 20px; }
  .meta span { display: inline-flex; align-items: center; gap: 4px; }
  .cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 16px; margin-bottom: 32px; }
  .card { background: var(--surface); border: 1px solid var(--border); border-radius: var(--card-radius); padding: 20px; text-align: center; }
  .card .value { font-size: 2rem; font-weight: 600; color: var(--accent); }
  .card .label { font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; margin-top: 4px; }
  .card.critical .value { color: var(--critical); }
  .card.high .value { color: var(--high); }
  .card.medium .value { color: var(--medium); }
  .card.low .value { color: var(--low); }
  .card.info .value { color: var(--info); }
  h2 { font-size: 1.3rem; margin: 24px 0 16px; color: var(--accent); border-bottom: 1px solid var(--border); padding-bottom: 8px; }
  table { width: 100%; border-collapse: collapse; margin-bottom: 24px; background: var(--surface); border: 1px solid var(--border); border-radius: var(--card-radius); overflow: hidden; }
  th, td { padding: 10px 14px; text-align: left; border-bottom: 1px solid var(--border); font-size: 0.9rem; }
  th { background: #1c2128; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.4px; font-size: 0.78rem; font-weight: 600; }
  tr:last-child td { border-bottom: none; }
  .severity-badge { display: inline-block; padding: 2px 8px; border-radius: 12px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.3px; }
  .severity-Critical { background: rgba(248,81,73,0.15); color: var(--critical); }
  .severity-High { background: rgba(210,153,34,0.15); color: var(--high); }
  .severity-Medium { background: rgba(163,113,247,0.15); color: var(--medium); }
  .severity-Low { background: rgba(88,166,255,0.15); color: var(--low); }
  .severity-Info { background: rgba(139,148,158,0.15); color: var(--info); }
  code { font-family: "SFMono-Regular",Consolas,"Liberation Mono",Menlo,monospace; background: #1c2128; padding: 2px 6px; border-radius: 4px; font-size: 0.85rem; }
  .finding-title { font-weight: 600; }
  .scorecard-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 16px; margin-bottom: 24px; }
  .pillar-card { background: var(--surface); border: 1px solid var(--border); border-radius: var(--card-radius); padding: 16px; }
  .pillar-card .pillar-name { font-weight: 600; margin-bottom: 4px; }
  .pillar-card .pillar-score { font-size: 1.5rem; font-weight: 600; color: var(--accent); }
  .pillar-card .pillar-gaps { font-size: 0.8rem; color: var(--text-muted); }
  .maturity-Baseline { color: var(--critical); }
  .maturity-Advanced { color: var(--high); }
  .maturity-Adaptive { color: var(--success); }
  .arch-diagram { background: var(--surface); border: 1px solid var(--border); border-radius: var(--card-radius); padding: 20px; margin-bottom: 24px; overflow-x: auto; }
  .arch-diagram pre { font-family: "SFMono-Regular",Consolas,monospace; font-size: 0.85rem; color: var(--text-muted); white-space: pre-wrap; }
  .pillar-tag { display: inline-block; padding: 1px 6px; border-radius: 4px; font-size: 0.7rem; background: rgba(88,166,255,0.1); color: var(--accent); margin: 1px; }
  @media (max-width: 600px) { .cards { grid-template-columns: repeat(2, 1fr); } .scorecard-grid { grid-template-columns: 1fr; } }
  footer { text-align: center; padding: 24px; color: var(--text-muted); font-size: 0.8rem; border-top: 1px solid var(--border); margin-top: 32px; }
</style>
</head>
<body>
<div class="container">
  <header>
    <h1><span class="shield">&#x1F6E1;</span>ApeGuard Security Report</h1>
    <div class="meta">
      <span>&#x1F4CB; Scan: <code>{{ scan_id }}</code></span>
      <span>&#x1F4C5; {{ timestamp }}</span>
      <span>&#x1F4E6; Target: <code>{{ target }}</code></span>
      <span>&#x23F1; {{ duration_seconds }}s</span>
    </div>
  </header>

  <div class="cards">
    <div class="card critical"><div class="value">{{ findings_by_severity.critical }}</div><div class="label">Critical</div></div>
    <div class="card high"><div class="value">{{ findings_by_severity.high }}</div><div class="label">High</div></div>
    <div class="card medium"><div class="value">{{ findings_by_severity.medium }}</div><div class="label">Medium</div></div>
    <div class="card low"><div class="value">{{ findings_by_severity.low }}</div><div class="label">Low</div></div>
    <div class="card info"><div class="value">{{ findings_by_severity.info }}</div><div class="label">Info</div></div>
  </div>

  {% if zt_scorecard %}
  <h2>Zero Trust Scorecard</h2>
  <div class="meta" style="margin-bottom:12px;">
    <span>Score: <strong>{{ zt_scorecard.overall_score }} / {{ zt_scorecard.max_score }}</strong></span>
    <span>Pillars at Advanced+: <strong>{{ zt_scorecard.pillars_at_advanced_or_higher }} / {{ zt_scorecard.pillars | length }}</strong></span>
  </div>
  <div class="scorecard-grid">
    {% for pillar in zt_scorecard.pillars %}
    <div class="pillar-card">
      <div class="pillar-name">{{ pillar.name }}</div>
      <div class="pillar-score maturity-{{ pillar.maturity }}">{{ pillar.score }}/100</div>
      <div class="pillar-gaps">Maturity: <span class="maturity-{{ pillar.maturity }}">{{ pillar.maturity }}</span> &middot; {{ pillar.gap_count }} gaps</div>
    </div>
    {% endfor %}
  </div>
  {% endif %}

  {% if arch_diagram %}
  <h2>Architecture Risk Diagram</h2>
  <div class="arch-diagram"><pre>{{ arch_diagram | safe }}</pre></div>
  {% endif %}

  <h2>Findings ({{ total_findings }})</h2>
  <table>
    <thead>
      <tr><th>Severity</th><th>Finding</th><th>File</th><th>Scanner</th><th>Remediation</th></tr>
    </thead>
    <tbody>
    {% for finding in findings %}
      <tr>
        <td><span class="severity-badge severity-{{ finding.severity }}">{{ finding.severity }}</span></td>
        <td>
          <div class="finding-title">{{ finding.title }}</div>
          <div style="font-size:0.8rem;color:var(--text-muted);">{{ finding.description }}</div>
          {% if finding.zt_pillars %}<div style="margin-top:4px;">{% for p in finding.zt_pillars %}<span class="pillar-tag">{{ p }}</span>{% endfor %}</div>{% endif %}
          {% if finding.cwe %}<div style="font-size:0.75rem;color:var(--text-muted);margin-top:2px;">{{ finding.cwe }}</div>{% endif %}
        </td>
        <td><code>{{ finding.file }}</code>{% if finding.line %}:{{ finding.line }}{% endif %}</td>
        <td><code>{{ finding.scanner }}</code></td>
        <td style="font-size:0.85rem;">{{ finding.remediation | default(value="—") }}</td>
      </tr>
    {% endfor %}
    </tbody>
  </table>

  <footer>Generated by ApeGuard v{{ scanners_used | first | default(value="") }} &mdash; {{ timestamp }}</footer>
</div>
</body>
</html>"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find::{
        CanonicalFinding, Confidence, FindingLocation, FindingsBySeverity, GapAnalysis, GapLevel,
        MaturityTier, PillarScore, ScanSummary, ScannerType, Severity, ZeroTrustScorecard,
    };
    use std::path::PathBuf;

    fn empty_scorecard() -> ZeroTrustScorecard {
        ZeroTrustScorecard {
            overall_score: 0,
            max_score: 100,
            pillars: vec![],
            pillars_at_advanced_or_higher: 0,
            target_maturity: MaturityTier::Baseline,
            gap_analysis: vec![],
        }
    }

    fn create_test_context() -> (ScanSummary, ZeroTrustScorecard, Vec<CanonicalFinding>) {
        let findings = vec![
            CanonicalFinding {
                id: "F-1".to_string(),
                scanner: ScannerType::Gitleaks,
                scanner_version: None,
                rule_id: "secret-rule".to_string(),
                severity: Severity::Critical,
                confidence: Confidence::Certain,
                title: "Hardcoded Secret".to_string(),
                description: "Found a secret in .env".to_string(),
                location: FindingLocation {
                    file: PathBuf::from(".env"),
                    line: Some(1),
                    column: None,
                    commit: None,
                    author: None,
                    snippet: Some("KEY=123".to_string()),
                },
                cwe: Some("CWE-798".to_string()),
                cvss: Some(9.8),
                remediation: Some("Rotate the secret".to_string()),
                fix_effort: None,
                evidence: None,
                tags: vec!["env".to_string()],
                zt_pillars: vec!["Identity".to_string()],
                cross_refs: vec![],
            },
            CanonicalFinding {
                id: "F-2".to_string(),
                scanner: ScannerType::Semgrep,
                scanner_version: None,
                rule_id: "sql-inj".to_string(),
                severity: Severity::High,
                confidence: Confidence::Firm,
                title: "SQL Injection".to_string(),
                description: "Possible SQL injection".to_string(),
                location: FindingLocation {
                    file: PathBuf::from("app.py"),
                    line: Some(42),
                    column: None,
                    commit: None,
                    author: None,
                    snippet: None,
                },
                cwe: Some("CWE-89".to_string()),
                cvss: Some(8.0),
                remediation: None,
                fix_effort: None,
                evidence: None,
                tags: vec![],
                zt_pillars: vec![],
                cross_refs: vec![],
            },
        ];

        let scorecard = ZeroTrustScorecard {
            overall_score: 45,
            max_score: 100,
            pillars: vec![PillarScore {
                name: "Identity".to_string(),
                maturity: MaturityTier::Baseline,
                gap_count: 2,
                score: 20,
            }],
            pillars_at_advanced_or_higher: 0,
            target_maturity: MaturityTier::Advanced,
            gap_analysis: vec![GapAnalysis {
                pillar: "Identity".to_string(),
                current_maturity: MaturityTier::Baseline,
                target_maturity: MaturityTier::Advanced,
                gap: GapLevel::Large,
                blocking_findings: 1,
                recommendations: vec!["Enable MFA".to_string()],
            }],
        };

        let summary = ScanSummary {
            scan_id: "test-scan".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            target: "test-repo".to_string(),
            target_hash: "abc123hash".to_string(),
            duration_seconds: 120.0,
            total_findings: 2,
            findings_by_severity: FindingsBySeverity {
                critical: 1,
                high: 1,
                medium: 0,
                low: 0,
                info: 0,
            },
            scanners_used: vec!["Gitleaks".to_string(), "Semgrep".to_string()],
            zt_scorecard: Some(scorecard.clone()),
            attack_chains: vec![],
        };

        (summary, scorecard, findings)
    }

    #[test]
    fn test_generate_all_reports_creates_files() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let paths = generate_all_reports(&summary, &findings, &scorecard, output_dir, None, &[])
            .expect("Failed to generate all reports");

        assert_eq!(paths.len(), 3);
        assert!(paths[0].to_str().unwrap().contains("technical-report.md"));
        assert!(paths[1].to_str().unwrap().contains("executive-report.md"));
        assert!(paths[2].to_str().unwrap().contains("roadmap-report.md"));

        for path in &paths {
            assert!(path.exists(), "Report file does not exist: {:?}", path);
            let content = std::fs::read_to_string(path).unwrap();
            assert!(!content.is_empty());
        }
    }

    #[test]
    fn test_generate_selected_report_types() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let paths = generate_all_reports(
            &summary,
            &findings,
            &scorecard,
            output_dir,
            None,
            &[ReportType::Executive],
        )
        .expect("Failed to generate reports");

        assert_eq!(paths.len(), 1);
        assert!(paths[0].to_str().unwrap().contains("executive-report.md"));
        assert!(paths[0].exists());
    }

    #[test]
    fn test_generate_json_report_format() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_json_report(&summary, &findings, &scorecard, output_dir, None)
            .expect("Failed to generate JSON report");

        assert!(path.exists());
        let content = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(json["summary"]["scan_id"], "test-scan");
        assert_eq!(json["findings"].as_array().unwrap().len(), 2);
        assert_eq!(json["findings"][0]["severity"], "Critical");
    }

    #[test]
    fn test_generate_json_report_with_arch_diagram() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_json_report(
            &summary,
            &findings,
            &scorecard,
            output_dir,
            Some("graph TD; A-->B;"),
        )
        .expect("Failed to generate JSON report");

        let content = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["arch_diagram"], "graph TD; A-->B;");
    }

    #[test]
    fn test_generate_sarif_report_format() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_sarif_report(&summary, &findings, &scorecard, output_dir, None)
            .expect("Failed to generate SARIF report");

        assert!(path.exists());
        let content = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(json["version"], "2.1.0");
        assert_eq!(json["runs"][0]["tool"]["driver"]["name"], "ApeGuard");
        assert!(json["$schema"]
            .as_str()
            .unwrap()
            .contains("sarif-schema-2.1.0"));
    }

    #[test]
    fn test_generate_sarif_report_with_arch_diagram() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_sarif_report(
            &summary,
            &findings,
            &scorecard,
            output_dir,
            Some("graph LR; A-->C;"),
        )
        .expect("Failed to generate SARIF report");

        let content = std::fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(
            json["runs"][0]["properties"]["apeguard"]["arch_diagram"],
            "graph LR; A-->C;"
        );
    }

    #[test]
    fn test_generate_report_technical_contains_findings() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_report(
            &ReportType::Technical,
            &summary,
            &findings,
            &scorecard,
            output_dir,
            None,
        )
        .expect("Failed to generate technical report");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("Hardcoded Secret"));
        assert!(content.contains("SQL Injection"));
        assert!(content.contains("CWE-798"));
        assert!(content.contains("Identity"));
    }

    #[test]
    fn test_generate_report_executive_contains_summary() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_report(
            &ReportType::Executive,
            &summary,
            &findings,
            &scorecard,
            output_dir,
            None,
        )
        .expect("Failed to generate executive report");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("Executive Security Report"));
        assert!(content.contains("45 / 100"));
        assert!(content.contains("2 findings across 2 scanners"));
    }

    #[test]
    fn test_generate_report_roadmap_contains_remediation() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_report(
            &ReportType::Roadmap,
            &summary,
            &findings,
            &scorecard,
            output_dir,
            None,
        )
        .expect("Failed to generate roadmap report");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("Remediation Roadmap"));
        assert!(content.contains("Rotate the secret"));
    }

    #[test]
    fn test_generate_report_with_arch_diagram_appears() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_report(
            &ReportType::Technical,
            &summary,
            &findings,
            &scorecard,
            output_dir,
            Some("graph TD; A-->B;"),
        )
        .expect("Failed to generate technical report with arch diagram");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("graph TD; A-->B;"));
    }

    #[test]
    fn test_report_type_default_technical_as_str() {
        assert_eq!(ReportType::Technical.as_str(), "technical");
        assert_eq!(ReportType::Executive.as_str(), "executive");
        assert_eq!(ReportType::Roadmap.as_str(), "roadmap");
    }

    #[test]
    fn test_technical_template_has_expected_sections() {
        assert!(TECHNICAL_TEMPLATE.contains("Summary"));
        assert!(TECHNICAL_TEMPLATE.contains("Detailed Findings"));
        assert!(TECHNICAL_TEMPLATE.contains("zt_scorecard"));
    }

    #[test]
    fn test_executive_template_has_expected_sections() {
        assert!(EXECUTIVE_TEMPLATE.contains("Risk Overview"));
        assert!(EXECUTIVE_TEMPLATE.contains("Finding Summary"));
        assert!(EXECUTIVE_TEMPLATE.contains("Top Findings"));
    }

    #[test]
    fn test_roadmap_template_has_expected_sections() {
        assert!(ROADMAP_TEMPLATE.contains("Immediate"));
        assert!(ROADMAP_TEMPLATE.contains("Short-term"));
        assert!(ROADMAP_TEMPLATE.contains("Remediation"));
    }

    #[test]
    fn test_generate_html_report_format() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_html_report(&summary, &findings, &scorecard, output_dir, None)
            .expect("Failed to generate HTML report");

        assert!(path.exists());
        assert!(path.to_str().unwrap().ends_with("apeguard-report.html"));

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("ApeGuard Security Report"));
        assert!(content.contains("test-scan"));
        assert!(content.contains("Hardcoded Secret"));
        assert!(content.contains("SQL Injection"));
    }

    #[test]
    fn test_generate_html_report_with_arch_diagram() {
        let (summary, scorecard, findings) = create_test_context();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();

        let path = generate_html_report(
            &summary,
            &findings,
            &scorecard,
            output_dir,
            Some("graph TD; A-->B;"),
        )
        .expect("Failed to generate HTML report with arch diagram");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("graph TD; A-->B;"));
        assert!(content.contains("Architecture Risk Diagram"));
    }

    #[test]
    fn test_generate_html_report_empty_findings() {
        let empty_summary = ScanSummary {
            scan_id: "empty-scan".to_string(),
            timestamp: "2026-01-01".to_string(),
            target: "test".to_string(),
            target_hash: "abc123".to_string(),
            duration_seconds: 0.0,
            total_findings: 0,
            findings_by_severity: FindingsBySeverity {
                critical: 0,
                high: 0,
                medium: 0,
                low: 0,
                info: 0,
            },
            scanners_used: vec!["ApeGuard".to_string()],
            zt_scorecard: Some(empty_scorecard()),
            attack_chains: vec![],
        };
        let empty_scorecard = empty_scorecard();
        let tmp = tempfile::tempdir().unwrap();
        let output_dir = tmp.path();
        let empty_findings = vec![];

        let path = generate_html_report(
            &empty_summary,
            &empty_findings,
            &empty_scorecard,
            output_dir,
            None,
        )
        .expect("Failed to generate HTML report with empty findings");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("ApeGuard Security Report"));
        assert!(content.contains("Findings (0)"));
    }

    #[test]
    fn test_html_template_has_expected_sections() {
        assert!(HTML_TEMPLATE.contains("ApeGuard Security Report"));
        assert!(HTML_TEMPLATE.contains("Zero Trust Scorecard"));
        assert!(HTML_TEMPLATE.contains("Architecture Risk Diagram"));
        assert!(HTML_TEMPLATE.contains("Findings ({{ total_findings }})"));
        assert!(HTML_TEMPLATE.contains("severity-badge"));
    }
}

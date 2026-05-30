// ApeGuard Report Generator
// Produces three types of reports from scan findings:
//   - Technical: detailed findings for engineers
//   - Executive: risk posture summary for leadership
//   - Roadmap: prioritized remediation plan for engineering managers
use crate::find::{CanonicalFinding, ScanSummary, ZeroTrustScorecard};
use crate::normalize;
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

/// Generate all three reports from scan results
pub fn generate_all_reports(
    summary: &ScanSummary,
    findings: &[CanonicalFinding],
    zt_scorecard: &ZeroTrustScorecard,
    output_dir: &Path,
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let mut generated = Vec::new();

    let report_types = [ReportType::Technical, ReportType::Executive, ReportType::Roadmap];
    for rtype in &report_types {
        let path = generate_report(rtype, summary, findings, zt_scorecard, output_dir)?;
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

    // Enrich findings with MITRE mapping
    let enriched_findings: Vec<_> = findings
        .iter()
        .map(|f| {
            let mitre = normalize::mitre_mapping(f);
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
                cross_refs: f.cross_refs.iter().map(|cr| format!("{:?}/{}", cr.scanner, cr.rule_id)).collect(),
            }
        })
        .collect();

    context.insert("findings", &enriched_findings);

    let rendered = tera.render(template_name, &context)?;

    let filename = format!("{}-report.md", report_type.as_str());
    let output_path = output_dir.join(&filename);
    std::fs::write(&output_path, rendered)?;

    tracing::info!("Generated {} report: {}", report_type.as_str(), output_path.display());

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

/// Built-in templates as Rust strings (no external template files needed)
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

## Medium-term (Medium — Next Sprint)

{% for finding in findings | filter(attribute="severity", value="Medium") %}
- [ ] **{{ finding.title }}** — {{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %}
{% else %}
*No medium-severity findings.*
{% endfor %}

## Low / Info (Track)

{% for finding in findings | filter(attribute="severity", value="Low") %}
- [ ] **{{ finding.title }}** — {{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %}
{% endfor %}
{% for finding in findings | filter(attribute="severity", value="Info") %}
- [ ] {{ finding.title }} — {{ finding.file }}{% if finding.line %}:{{ finding.line }}{% endif %}
{% endfor %}

## Zero Trust Improvement Plan

{% if zt_scorecard %}
| Pillar | Maturity | Target | Action Needed |
|--------|----------|--------|---------------|
{% for pillar in zt_scorecard.pillars %}
{% if pillar.maturity != "Adaptive" %}
| {{ pillar.name }} | {{ pillar.maturity }} | Advanced | Address {{ pillar.gap_count }} gap(s) |
{% endif %}
{% endfor %}
{% endif %}
"#;

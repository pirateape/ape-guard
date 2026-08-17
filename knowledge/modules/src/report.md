---
type: Rust Module
title: report
resource: src/report/mod.rs#L1-L1609
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding-scansummary-zerotrustscorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tera-context-tera
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-findingsbyseverity-gapanalysis-gaplevel-maturitytier-pillarscore-scansummary-scannertype-severity-zerotrustscorecard
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ReportType](../../classes/src/report/ReportType.md)
- [as_str](../../functions/src/report/ReportType/as_str.md)
- [generate_all_reports](../../functions/src/report/generate_all_reports.md)
- [generate_report](../../functions/src/report/generate_report.md)
- [EnrichedFinding](../../classes/src/report/EnrichedFinding.md)
- [get_template](../../functions/src/report/get_template.md)
- [generate_json_report](../../functions/src/report/generate_json_report.md)
- [JsonReport](../../classes/src/report/JsonReport.md)
- [JsonFinding](../../classes/src/report/JsonFinding.md)
- [generate_sarif_report](../../functions/src/report/generate_sarif_report.md)
- [SarifReport](../../classes/src/report/SarifReport.md)
- [SarifRun](../../classes/src/report/SarifRun.md)
- [SarifTool](../../classes/src/report/SarifTool.md)
- [SarifDriver](../../classes/src/report/SarifDriver.md)
- [SarifRule](../../classes/src/report/SarifRule.md)
- [SarifDefaultConfig](../../classes/src/report/SarifDefaultConfig.md)
- [SarifMessage](../../classes/src/report/SarifMessage.md)
- [SarifResult](../../classes/src/report/SarifResult.md)
- [SarifLocation](../../classes/src/report/SarifLocation.md)
- [SarifPhysicalLocation](../../classes/src/report/SarifPhysicalLocation.md)
- [SarifArtifactLocation](../../classes/src/report/SarifArtifactLocation.md)
- [SarifRegion](../../classes/src/report/SarifRegion.md)
- [generate_html_report](../../functions/src/report/generate_html_report.md)
- [empty_scorecard](../../functions/src/report/empty_scorecard.md)
- [create_test_context](../../functions/src/report/create_test_context.md)
- [test_generate_all_reports_creates_files](../../functions/src/report/test_generate_all_reports_creates_files.md)
- [test_generate_selected_report_types](../../functions/src/report/test_generate_selected_report_types.md)
- [test_generate_json_report_format](../../functions/src/report/test_generate_json_report_format.md)
- [test_generate_json_report_with_arch_diagram](../../functions/src/report/test_generate_json_report_with_arch_diagram.md)
- [test_generate_sarif_report_format](../../functions/src/report/test_generate_sarif_report_format.md)
- [test_generate_sarif_report_with_arch_diagram](../../functions/src/report/test_generate_sarif_report_with_arch_diagram.md)
- [test_generate_report_technical_contains_findings](../../functions/src/report/test_generate_report_technical_contains_findings.md)
- [test_generate_report_executive_contains_summary](../../functions/src/report/test_generate_report_executive_contains_summary.md)
- [test_generate_report_roadmap_contains_remediation](../../functions/src/report/test_generate_report_roadmap_contains_remediation.md)
- [test_generate_report_with_arch_diagram_appears](../../functions/src/report/test_generate_report_with_arch_diagram_appears.md)
- [test_report_type_default_technical_as_str](../../functions/src/report/test_report_type_default_technical_as_str.md)
- [test_technical_template_has_expected_sections](../../functions/src/report/test_technical_template_has_expected_sections.md)
- [test_executive_template_has_expected_sections](../../functions/src/report/test_executive_template_has_expected_sections.md)
- [test_roadmap_template_has_expected_sections](../../functions/src/report/test_roadmap_template_has_expected_sections.md)
- [test_generate_html_report_format](../../functions/src/report/test_generate_html_report_format.md)
- [test_generate_html_report_with_arch_diagram](../../functions/src/report/test_generate_html_report_with_arch_diagram.md)
- [test_generate_html_report_empty_findings](../../functions/src/report/test_generate_html_report_empty_findings.md)
- [test_html_template_has_expected_sections](../../functions/src/report/test_html_template_has_expected_sections.md)

# Imports

- `crate::find::{CanonicalFinding, ScanSummary, ZeroTrustScorecard}`
- `serde_json::json`
- `std::path::Path`
- `tera::{Context, Tera}`
- `super::*`
- `crate::find::{
        CanonicalFinding, Confidence, FindingLocation, FindingsBySeverity, GapAnalysis, GapLevel,
        MaturityTier, PillarScore, ScanSummary, ScannerType, Severity, ZeroTrustScorecard,
    }`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
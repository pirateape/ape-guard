---
type: Rust Function
title: mitre_mapping
resource: src/normalize.rs#L415-L442
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/normalize/test_mitre_mapping_secret
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/report/generate_html_report
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn mitre_mapping(finding: &CanonicalFinding) -> Vec<String>`

# Called by

- [test_mitre_mapping_secret](../../../functions/src/normalize/test_mitre_mapping_secret.md)
- [generate_report](../../../functions/src/report/generate_report.md)
- [generate_html_report](../../../functions/src/report/generate_html_report.md)
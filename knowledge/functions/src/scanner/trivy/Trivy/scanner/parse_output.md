---
type: Rust Method
title: parse_output
resource: src/scanner/trivy.rs#L114-L120
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/trivy/Trivy/parse_vuln
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/parse_secret
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/trivy/Trivy/parse_misconfig
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_output(&self, raw: &[u8]) -> Result<Vec<CanonicalFinding>, ScannerError>`

# Calls

- [parse_vuln](../../../../../../functions/src/scanner/trivy/Trivy/parse_vuln.md)
- [parse_secret](../../../../../../functions/src/scanner/trivy/Trivy/parse_secret.md)
- [parse_misconfig](../../../../../../functions/src/scanner/trivy/Trivy/parse_misconfig.md)
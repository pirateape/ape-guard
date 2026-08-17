---
type: Rust Function
title: generate_pillar_recommendations
resource: src/normalize.rs#L314-L412
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/normalize/test_pillar_recommendations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/normalize/test_recommendations_missing_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_pillar_recommendations( pillar: &str, maturity: MaturityTier, finding_count: u32, ) -> Vec<String>`

# Called by

- [test_pillar_recommendations](../../../functions/src/normalize/test_pillar_recommendations.md)
- [test_recommendations_missing_findings](../../../functions/src/normalize/test_recommendations_missing_findings.md)
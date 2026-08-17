---
type: Rust Function
title: compute_gap_analysis
resource: src/normalize.rs#L246-L310
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/normalize/compute_zt_scorecard
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_gap_analysis( all_pillars: &[&str], pillar_severity: &std::collections::HashMap<&str, u32>, pillar_finding_refs: &std::collections::HashMap<&str, Vec<&CanonicalFinding>>, ) -> Vec<GapAnalysis>`

# Called by

- [compute_zt_scorecard](../../../functions/src/normalize/compute_zt_scorecard.md)
---
type: Rust Function
title: apply_reachability
resource: src/reachability.rs#L791-L829
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_apply_reachability_markings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn apply_reachability(findings: &mut [CanonicalFinding], result: &ReachabilityResult)`

# Calls

- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
- [test_apply_reachability_markings](../../../functions/src/reachability/test_apply_reachability_markings.md)
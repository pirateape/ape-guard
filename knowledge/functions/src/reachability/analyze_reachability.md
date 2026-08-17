---
type: Rust Function
title: analyze_reachability
resource: src/reachability.rs#L709-L787
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/discover_entry_points
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/collect_source_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/build_import_graph
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/bfs_reachable_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/canonicalize_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_analyze_reachability_disabled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn analyze_reachability( findings: &[CanonicalFinding], target: &Path, config: &ReachabilityConfig, ) -> ReachabilityResult`

# Calls

- [discover_entry_points](../../../functions/src/reachability/discover_entry_points.md)
- [collect_source_files](../../../functions/src/reachability/collect_source_files.md)
- [build_import_graph](../../../functions/src/reachability/build_import_graph.md)
- [bfs_reachable_files](../../../functions/src/reachability/bfs_reachable_files.md)
- [canonicalize_path](../../../functions/src/reachability/canonicalize_path.md)

# Called by

- [run_scan](../../../functions/src/orchestrate/run_scan.md)
- [test_analyze_reachability_disabled](../../../functions/src/reachability/test_analyze_reachability_disabled.md)
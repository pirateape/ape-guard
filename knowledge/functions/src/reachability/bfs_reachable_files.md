---
type: Rust Function
title: bfs_reachable_files
resource: src/reachability.rs#L668-L696
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_bfs_simple
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_bfs_with_cycle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_bfs_unreachable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/reachability/test_bfs_multiple_entry_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bfs_reachable_files(graph: &ImportGraph, entry_points: &[PathBuf]) -> HashSet<PathBuf>`

# Called by

- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
- [test_bfs_simple](../../../functions/src/reachability/test_bfs_simple.md)
- [test_bfs_with_cycle](../../../functions/src/reachability/test_bfs_with_cycle.md)
- [test_bfs_unreachable](../../../functions/src/reachability/test_bfs_unreachable.md)
- [test_bfs_multiple_entry_points](../../../functions/src/reachability/test_bfs_multiple_entry_points.md)
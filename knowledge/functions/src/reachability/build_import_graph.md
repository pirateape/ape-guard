---
type: Rust Function
title: build_import_graph
resource: src/reachability.rs#L625-L664
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/reachability/extract_imports
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/reachability/analyze_reachability
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_import_graph( all_files: &[PathBuf], target_root: &Path, ) -> (ImportGraph, ImportGraph, usize)`

# Calls

- [extract_imports](../../../functions/src/reachability/extract_imports.md)

# Called by

- [analyze_reachability](../../../functions/src/reachability/analyze_reachability.md)
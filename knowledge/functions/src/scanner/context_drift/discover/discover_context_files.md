---
type: Rust Function
title: discover_context_files
resource: src/scanner/context_drift/discover.rs#L41-L60
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/scanner/context_drift/types/ContextFileType/file_names
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/scanner/context_drift/ContextDriftScanner/scan_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/test_discover_context_files_agents_md
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/scanner/context_drift/parse/parse_all_context_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn discover_context_files(root: &Path) -> Vec<PathBuf>`

# Calls

- [file_names](../../../../../functions/src/scanner/context_drift/types/ContextFileType/file_names.md)

# Called by

- [scan_drift](../../../../../functions/src/scanner/context_drift/ContextDriftScanner/scan_drift.md)
- [test_discover_context_files_agents_md](../../../../../functions/src/scanner/context_drift/test_discover_context_files_agents_md.md)
- [parse_all_context_files](../../../../../functions/src/scanner/context_drift/parse/parse_all_context_files.md)
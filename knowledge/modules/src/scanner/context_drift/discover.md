---
type: Rust Module
title: discover
resource: src/scanner/context_drift/discover.rs#L1-L81
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-types-contextfiletype
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-oncelock
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [dep_pattern](../../../../functions/src/scanner/context_drift/discover/dep_pattern.md)
- [version_pattern](../../../../functions/src/scanner/context_drift/discover/version_pattern.md)
- [path_pattern](../../../../functions/src/scanner/context_drift/discover/path_pattern.md)
- [technology_keyword_pattern](../../../../functions/src/scanner/context_drift/discover/technology_keyword_pattern.md)
- [discover_context_files](../../../../functions/src/scanner/context_drift/discover/discover_context_files.md)
- [detect_file_type](../../../../functions/src/scanner/context_drift/discover/detect_file_type.md)
- [read_file](../../../../functions/src/scanner/context_drift/discover/read_file.md)

# Imports

- `super::types::ContextFileType`
- `regex::Regex`
- `std::path::{Path, PathBuf}`
- `std::sync::OnceLock`

# Member of

- [apeguard](../../../../packages/apeguard.md)
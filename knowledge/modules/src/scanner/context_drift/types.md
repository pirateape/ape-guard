---
type: Rust Module
title: types
resource: src/scanner/context_drift/types.rs#L1-L136
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-confidence-severity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ContextFileType](../../../../classes/src/scanner/context_drift/types/ContextFileType.md)
- [file_names](../../../../functions/src/scanner/context_drift/types/ContextFileType/file_names.md)
- [ClaimCategory](../../../../classes/src/scanner/context_drift/types/ClaimCategory.md)
- [as_str](../../../../functions/src/scanner/context_drift/types/ClaimCategory/as_str.md)
- [default_severity](../../../../functions/src/scanner/context_drift/types/ClaimCategory/default_severity.md)
- [ContextClaim](../../../../classes/src/scanner/context_drift/types/ContextClaim.md)
- [ContextFileRef](../../../../classes/src/scanner/context_drift/types/ContextFileRef.md)
- [VerificationResult](../../../../classes/src/scanner/context_drift/types/VerificationResult.md)
- [DriftFinding](../../../../classes/src/scanner/context_drift/types/DriftFinding.md)
- [ContextFileConfig](../../../../classes/src/scanner/context_drift/types/ContextFileConfig.md)
- [default](../../../../functions/src/scanner/context_drift/types/ContextFileConfig/default/default.md)

# Imports

- `crate::find::{Confidence, Severity}`
- `serde::{Deserialize, Serialize}`
- `std::path::PathBuf`

# Member of

- [apeguard](../../../../packages/apeguard.md)
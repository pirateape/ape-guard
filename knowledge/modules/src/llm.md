---
type: Rust Module
title: llm
resource: src/llm.rs#L1-L257
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-find-canonicalfinding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-find-canonicalfinding-confidence-findinglocation-scannertype-severity
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

- [LlmConfig](../../classes/src/llm/LlmConfig.md)
- [default](../../functions/src/llm/LlmConfig/default/default.md)
- [enhance_remediations](../../functions/src/llm/enhance_remediations.md)
- [build_remediation_prompt](../../functions/src/llm/build_remediation_prompt.md)
- [with_retry](../../functions/src/llm/with_retry.md)
- [call_ollama](../../functions/src/llm/call_ollama.md)
- [OllamaRequest](../../classes/src/llm/OllamaRequest.md)
- [OllamaResponse](../../classes/src/llm/OllamaResponse.md)
- [test_llm_config_default](../../functions/src/llm/test_llm_config_default.md)
- [test_enhance_remediations_disabled](../../functions/src/llm/test_enhance_remediations_disabled.md)
- [test_build_remediation_prompt](../../functions/src/llm/test_build_remediation_prompt.md)

# Imports

- `crate::find::CanonicalFinding`
- `serde::Deserialize`
- `super::*`
- `crate::find::{CanonicalFinding, Confidence, FindingLocation, ScannerType, Severity}`
- `std::path::PathBuf`

# Member of

- [apeguard](../../packages/apeguard.md)
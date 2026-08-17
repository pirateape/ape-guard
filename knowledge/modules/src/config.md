---
type: Rust Module
title: config
resource: src/config.rs#L1-L551
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-cli
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Config](../../classes/src/config/Config.md)
- [FilterConfig](../../classes/src/config/FilterConfig.md)
- [default](../../functions/src/config/FilterConfig/default/default.md)
- [ReachabilityConfig](../../classes/src/config/ReachabilityConfig.md)
- [default](../../functions/src/config/ReachabilityConfig/default/default.md)
- [StrideConfig](../../classes/src/config/StrideConfig.md)
- [default](../../functions/src/config/StrideConfig/default/default.md)
- [ContextDriftConfig](../../classes/src/config/ContextDriftConfig.md)
- [default](../../functions/src/config/ContextDriftConfig/default/default.md)
- [ScannerBinaries](../../classes/src/config/ScannerBinaries.md)
- [CacheConfig](../../classes/src/config/CacheConfig.md)
- [ReportConfig](../../classes/src/config/ReportConfig.md)
- [LlmConfig](../../classes/src/config/LlmConfig.md)
- [default](../../functions/src/config/LlmConfig/default/default.md)
- [default](../../functions/src/config/Config/default/default.md)
- [load](../../functions/src/config/load.md)
- [merge](../../functions/src/config/merge.md)
- [generate_init](../../functions/src/config/generate_init.md)
- [test_config_defaults](../../functions/src/config/test_config_defaults.md)
- [test_merge_overlay](../../functions/src/config/test_merge_overlay.md)
- [test_merge_empty_layers_does_not_override](../../functions/src/config/test_merge_empty_layers_does_not_override.md)
- [test_merge_report_formats](../../functions/src/config/test_merge_report_formats.md)
- [test_config_yaml_roundtrip](../../functions/src/config/test_config_yaml_roundtrip.md)
- [test_config_yaml_custom_values](../../functions/src/config/test_config_yaml_custom_values.md)
- [test_generate_init_fails_if_exists](../../functions/src/config/test_generate_init_fails_if_exists.md)
- [test_scanner_binaries_defaults](../../functions/src/config/test_scanner_binaries_defaults.md)

# Imports

- `crate::cli`
- `serde::{Deserialize, Serialize}`
- `std::path::PathBuf`
- `super::*`

# Member of

- [apeguard](../../packages/apeguard.md)
---
type: Rust Method
title: default
resource: src/config.rs#L228-L261
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/ContextDriftConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/FilterConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/ReachabilityConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/StrideConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/policy/PolicyConfig/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/config/load
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/generate_init
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/test_config_defaults
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/test_merge_overlay
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/test_merge_empty_layers_does_not_override
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/test_merge_report_formats
    resolved_by: rust-analyzer
    confidence: semantic
  - target: functions/src/config/test_config_yaml_roundtrip
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default](../../../../../functions/src/config/ContextDriftConfig/default/default.md)
- [default](../../../../../functions/src/config/FilterConfig/default/default.md)
- [default](../../../../../functions/src/config/ReachabilityConfig/default/default.md)
- [default](../../../../../functions/src/config/StrideConfig/default/default.md)
- [default](../../../../../functions/src/policy/PolicyConfig/default/default.md)

# Called by

- [load](../../../../../functions/src/config/load.md)
- [generate_init](../../../../../functions/src/config/generate_init.md)
- [test_config_defaults](../../../../../functions/src/config/test_config_defaults.md)
- [test_merge_overlay](../../../../../functions/src/config/test_merge_overlay.md)
- [test_merge_empty_layers_does_not_override](../../../../../functions/src/config/test_merge_empty_layers_does_not_override.md)
- [test_merge_report_formats](../../../../../functions/src/config/test_merge_report_formats.md)
- [test_config_yaml_roundtrip](../../../../../functions/src/config/test_config_yaml_roundtrip.md)
---
type: Rust Function
title: generate_init
resource: src/config.rs#L381-L414
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: rust-analyzer
    confidence: semantic
  called_by:
  - target: functions/src/config/test_generate_init_fails_if_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_init(path: Option<String>, _template: cli::InitTemplate) -> anyhow::Result<()>`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)

# Called by

- [test_generate_init_fails_if_exists](../../../functions/src/config/test_generate_init_fails_if_exists.md)
- [main](../../../functions/src/main.md)
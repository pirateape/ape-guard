---
type: Rust Module
title: integration
resource: tests/integration.rs#L1-L253
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/assert-cmd-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [test_version](../../functions/tests/integration/test_version.md)
- [test_init_creates_config](../../functions/tests/integration/test_init_creates_config.md)
- [test_init_fails_if_exists](../../functions/tests/integration/test_init_fails_if_exists.md)
- [test_completions_bash](../../functions/tests/integration/test_completions_bash.md)
- [test_completions_zsh](../../functions/tests/integration/test_completions_zsh.md)
- [test_config_validate](../../functions/tests/integration/test_config_validate.md)
- [test_config_paths](../../functions/tests/integration/test_config_paths.md)
- [test_scan_on_empty_dir](../../functions/tests/integration/test_scan_on_empty_dir.md)
- [test_scan_with_config](../../functions/tests/integration/test_scan_with_config.md)
- [test_help_output](../../functions/tests/integration/test_help_output.md)
- [test_scan_help](../../functions/tests/integration/test_scan_help.md)
- [test_full_scan_with_findings_and_formats](../../functions/tests/integration/test_full_scan_with_findings_and_formats.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`

# Member of

- [apeguard](../../packages/apeguard.md)
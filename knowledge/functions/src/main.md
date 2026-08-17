---
type: Rust Function
title: main
resource: src/main.rs#L80-L208
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/cli/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/find/ScannerType/std-fmt-display/fmt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/install_signal_handler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/orchestrate/run_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cache/ScanCache/record_scan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/run_compare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/generate_init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/handle_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/print_version
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/generate_completions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/serve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/handle_cache
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn main() -> anyhow::Result<()>`

# Calls

- [parse](../../functions/src/cli/parse.md)
- [fmt](../../functions/src/find/ScannerType/std-fmt-display/fmt.md)
- [install_signal_handler](../../functions/src/install_signal_handler.md)
- [load](../../functions/src/config/load.md)
- [run_scan](../../functions/src/orchestrate/run_scan.md)
- [record_scan](../../functions/src/cache/ScanCache/record_scan.md)
- [run_report](../../functions/src/run_report.md)
- [run_compare](../../functions/src/run_compare.md)
- [generate_init](../../functions/src/config/generate_init.md)
- [handle_config](../../functions/src/handle_config.md)
- [print_version](../../functions/src/print_version.md)
- [generate_completions](../../functions/src/cli/generate_completions.md)
- [serve](../../functions/src/mcp/serve.md)
- [handle_cache](../../functions/src/handle_cache.md)
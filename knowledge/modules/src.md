---
type: Rust Module
title: src
resource: src/main.rs#L1-L727
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/chrono-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-signal-unix-signal-signalkind
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [install_signal_handler](../functions/src/install_signal_handler.md)
- [cleanup_child_processes](../functions/src/cleanup_child_processes.md)
- [cleanup_child_processes](../functions/src/cleanup_child_processes-2.md)
- [main](../functions/src/main.md)
- [run_report](../functions/src/run_report.md)
- [run_compare](../functions/src/run_compare.md)
- [count_by_sev](../functions/src/count_by_sev.md)
- [CompareResult](../classes/src/CompareResult.md)
- [handle_config](../functions/src/handle_config.md)
- [handle_cache](../functions/src/handle_cache.md)
- [print_version](../functions/src/print_version.md)

# Imports

- `chrono::Utc`
- `sha2::Digest`
- `tokio::signal::unix::{signal, SignalKind}`
- `std::path::PathBuf`
- `std::collections::HashMap`

# Member of

- [apeguard](../packages/apeguard.md)
---
type: Rust Function
title: cleanup_child_processes
resource: src/main.rs#L66-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stride/StrideCategory/id
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/install_signal_handler
    resolved_by: rust-analyzer
    confidence: semantic
---

# Signature

`fn cleanup_child_processes()`

# Calls

- [id](../../functions/src/stride/StrideCategory/id.md)

# Called by

- [install_signal_handler](../../functions/src/install_signal_handler.md)
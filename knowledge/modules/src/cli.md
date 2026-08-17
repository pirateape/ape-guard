---
type: Rust Module
title: cli
resource: src/cli.rs#L1-L256
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/clap-parser-subcommand-valueenum
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-commandfactory
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/apeguard
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Args](../../classes/src/cli/Args.md)
- [Command](../../classes/src/cli/Command.md)
- [CacheSubcommand](../../classes/src/cli/CacheSubcommand.md)
- [is_scan](../../functions/src/cli/Command/is_scan.md)
- [SeverityFilter](../../classes/src/cli/SeverityFilter.md)
- [OutputFormat](../../classes/src/cli/OutputFormat.md)
- [ReportType](../../classes/src/cli/ReportType.md)
- [FailOnThreshold](../../classes/src/cli/FailOnThreshold.md)
- [CompareFormat](../../classes/src/cli/CompareFormat.md)
- [InitTemplate](../../classes/src/cli/InitTemplate.md)
- [ConfigSubcommand](../../classes/src/cli/ConfigSubcommand.md)
- [parse](../../functions/src/cli/parse.md)
- [generate_completions](../../functions/src/cli/generate_completions.md)

# Imports

- `clap::{Parser, Subcommand, ValueEnum}`
- `clap::CommandFactory`

# Member of

- [apeguard](../../packages/apeguard.md)
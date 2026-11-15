---
id: apeguard-llm
type: module-design
title: llm.rs — Ollama remediation enhancement
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# llm.rs — Ollama remediation enhancement

## Responsibility

`src/llm.rs` feeds findings into a local Ollama model to generate concise remediation (<200
chars). `LlmConfig { endpoint (default http://localhost:11434), model (default codellama),
enabled }`. `enhance_remediations` skips findings with existing remediation > 40 chars,
pings Ollama, and applies `guardrail_check` to each response before storing. `call_ollama`
retries 3× with exponential backoff. `guardrail_check` rejects instruction-injection
markers, dangerous commands, and pipe-to-shell patterns (shared pattern with `grade.rs`).

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `reqwest`, `tokio`, `serde`, `anyhow`,
  `tracing`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Design note

The LLM is fed untrusted scanner output (description, file from a scanned repo), so the
deterministic guardrail is a cheap output-layer defense before a poisoned remediation is
stored or shown.

## Tests

Inline unit tests cover config default, disabled enhancement, prompt construction, and
guardrail acceptance/rejection (all in `src/llm.rs`).

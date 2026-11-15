---
id: apeguard-grade
type: module-design
title: grade.rs — adversarial AI grader
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find, apeguard-llm]
references: []
implements: []
covers: []
tags: []
---

# grade.rs — adversarial AI grader

## Responsibility

`src/grade.rs` verifies scanner findings via an adversarial Ollama prompt ("prove this is
a false positive"). `grade_findings` skips already-graded findings, pings Ollama
(`/api/tags`), rate-limits to 8 parallel calls (Semaphore), and assigns verdicts by original
index. Returns the count of findings graded; gracefully degrades (returns 0) if Ollama is
unavailable. Each finding gets `GradeVerdict`: Confirmed { confidence, reasoning },
Rejected { reasoning, reason_category: RejectReason }, or NeedsReview { reasoning,
open_questions }.

`build_grade_prompt` wraps untrusted scanner data (rule_id, title, file, snippet) in
`<<<BEGIN UNTRUSTED DATA>>>` fences. `parse_grade_response` handles clean JSON, markdown-fenced
JSON, and keyword fallback. `guardrail_check` rejects instruction-injection markers and
dangerous-command / pipe-to-shell patterns before a verdict is stored.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, GradeVerdict, RejectReason}`, `llm::{call_ollama}`,
  `reqwest`, `tokio`, `futures`, `serde`, `std::sync::Arc`, `tokio::sync::Semaphore`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Design note

Per docs/grade-engine.md: per-finding grading (independent, parallelizable) over batch;
"guilty until proven innocent" adversarial frame; integrated via `--grade` CLI flag.

## Tests

Inline unit tests cover prompt construction, JSON/markdown-fenced/keyword parsing, reject
reason mapping, verdict counting, guardrail acceptance/rejection, and serialize roundtrip
(all in `src/grade.rs`).

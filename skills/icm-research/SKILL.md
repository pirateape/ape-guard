---
name: icm-research
description: >-
  Project-local research workflow — spikes, benchmarks, experiments,
  comparisons, investigations. Use when investigating a question,
  benchmarking performance, running an experiment, or deciding where
  findings belong: docs/research/ here, docs/adr/, or the APE-Brain
  knowledge hub (the graduation rule).
---

# Project-Local Research

## Where findings belong
| Question | Home |
|----------|------|
| Spike / benchmark / experiment supporting a change here | `docs/research/` (this repo) |
| A decision with lasting consequences | `docs/adr/` (see icm-adr) |
| Knowledge useful beyond this project | APE-Brain `02-Notes/` (graduation) |

## The graduation rule
Project-local research stays versioned in this repo. When a finding matters
beyond the project: write a **dated** note in APE-Brain `02-Notes/` and add
it to the hub's `MEMORY.md` index, referencing back here. Never maintain a
duplicate knowledge tree; never copy the hub's content back into the project
(the hub rule: don't save there what this repo already records).

## Method (evidence loop)
See the icm-problem-solving skill for the full loop: local evidence first
(okf-rs / mempalace / codegraph), then external sources, then triangulate.
Record raw numbers and method, not just conclusions — a benchmark without
its method is not evidence.

Details: docs/research/README.md (the convention, scaffolded with ADR 0001)

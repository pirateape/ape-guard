# Research Notes (project-local)

Working home for research produced *by this project*: spikes, benchmarks,
experiment results, comparisons, and investigations that support code changes
here.

## What belongs here
- Benchmarks and performance measurements with their raw numbers and method.
- Spike summaries (what was tried, what worked, links to branches/commits).
- Experiments and their outcomes (hypothesis → evidence → conclusion).
- Investigations that informed a change (link the resulting ADR in `docs/adr/`).

## What does NOT belong here
- Durable cross-project knowledge → graduates to the APE-Brain knowledge hub
  (`~/Documents/src/APE-Brain/02-Notes/`, dated file + `MEMORY.md` index entry).
- Architecture decisions → `docs/adr/`.
- Feature discovery evidence → the feature-discovery workflow state
  (`docs/workflows/`, when the workflow module is enabled for this project).

## Graduation rule
Project-local research stays versioned in this repo. When a finding matters
beyond this project, write a dated note in APE-Brain `02-Notes/` and reference
it here — never maintain a duplicate knowledge tree in parallel. The APE-Brain
memory protocol rule applies: don't save in the hub what this repo already
records.

---
name: icm-adr
description: >-
  Record architecture decisions as ADRs. Use when a change alters project
  conventions, scripts, or scaffold output; changes a behavior contract;
  chooses between alternatives with lasting consequences; or when asked to
  document a decision. Covers the template, numbering, and docs/adr/ index.
---

# Architecture Decision Records

## When a change needs an ADR
- It alters conventions, scripts, or scaffold output (what future scaffolds
  emit).
- It changes a behavior contract other code or agents rely on.
- It chooses between real alternatives and is costly to reverse.
Skip ADRs for trivial or easily reversible choices.

## How
```bash
cp <icm-checkout>/templates/adr-template.md docs/adr/NNNN-short-title.md
```
1. Number sequentially (`ls docs/adr/`), no duplicates.
2. Fill Status / Context / Decision / Consequences / Alternatives.
3. Update the `docs/adr/README.md` index table (ID, title, status, date, authors).
4. Cross-link: related ADRs under "Related ADRs"; research that informed the
   decision lives in `docs/research/` and links here and vice versa.
5. ADRs are immutable history — supersede, never edit accepted decisions.

## Lighter alternative
Small reversible tweaks don't need the full template — a commit message or a
research note may suffice. When in doubt, write the ADR; they are cheap.

Details: docs/adr/0001 (the convention itself) · templates/adr-template.md

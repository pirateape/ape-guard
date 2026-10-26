# ADR 0001: Record architecture decisions

## Status
Accepted

## Context
ApeGuard's architecture evolves through decisions made in conversations
and pull requests. Without a durable record, the reasoning behind a choice is
lost — future contributors re-litigate settled questions or unknowingly
violate them.

## Decision
Record every significant architecture decision as a numbered ADR in `docs/adr/`:

- One Markdown file per decision, named `NNNN-short-title.md`, numbered sequentially.
- Write it with the ADR template shipped by ICM (`templates/adr-template.md` in
  the ICM checkout): Status, Context, Decision, Consequences, Alternatives.
- Keep `docs/adr/README.md`'s index table up to date when an ADR is added.
- ADRs are immutable history: supersede rather than edit accepted decisions.

## Consequences
- Decisions are reviewable in PRs and greppable from any agent context.
- Small overhead per decision; skip ADRs for trivial or easily reversible choices.

## References
- Template: `templates/adr-template.md` (ICM checkout)
- Michael Nygard, "Documenting Architecture Decisions" (the ADR format)

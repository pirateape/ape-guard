---
id: apeguard-dedup
type: module-design
title: dedup.rs — deduplication & cross-referencing
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# dedup.rs — deduplication & cross-referencing

## Responsibility

`src/dedup.rs` provides two functions:
- `deduplicate(findings)` — drops exact duplicates keyed by `(file, line, rule_id)`;
  findings with `line == None` are always kept (cannot confirm a duplicate location).
  Priority: first-seen finding at a key wins (higher-confidence scanner preference is
  documented but not enforced).
- `cross_reference(findings)` — links findings at the same `(file, line)` regardless of
  `rule_id`, so different scanners/rules at one location cross-reference each other;
  avoids duplicate `(scanner, rule_id)` refs and self-references.

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `std::collections::HashSet`/`HashMap`.
- **Forbidden reaches:** must not import any other apeguard module.

## Invariant

- Dedup key is `(file, line, rule_id)`; same file+line with different rule_ids are both kept
  (two different CVEs can appear at the same location).

## Tests

Inline unit tests cover duplicate removal, different files/lines, no-line handling, empty
input, cross-reference linking (2 and 3 scanners), no self-ref, and different-location
non-linking (all in `src/dedup.rs`).

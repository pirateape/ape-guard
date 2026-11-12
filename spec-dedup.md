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
- `deduplicate(findings)` — drops exact duplicates keyed by `(file, line, rule_id)`.
  When `line` is `None` (e.g. Trivy vulnerability findings), the same file + rule_id
  IS confirmably the same finding, so dedup applies there too. Different rule_ids
  at the same location are always both kept (two different CVEs can appear at the
  same location).
- `cross_reference(findings)` — links findings at the same `(file, line)` regardless of
  `rule_id`, so different scanners/rules at one location cross-reference each other;
  avoids duplicate `(scanner, rule_id)` refs and self-references.

## Boundary

- **Allowed deps:** `find::CanonicalFinding`, `std::collections::HashSet`/`HashMap`.
- **Forbidden reaches:** must not import any other apeguard module.

## Invariant

- Dedup key is `(file, line, rule_id)`; same file+line with different rule_ids are both kept
  (two different CVEs can appear at the same location). When `line` is `None`, the key
  degrades to `(file, None, rule_id)` — same file + same rule is the same finding.

## Tests

Inline unit tests cover duplicate removal, different files/lines, no-line handling, empty
input, cross-reference linking (2 and 3 scanners), no self-ref, and different-location
non-linking (all in `src/dedup.rs`).

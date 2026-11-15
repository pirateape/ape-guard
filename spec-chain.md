---
id: apeguard-chain
type: module-design
title: chain.rs — attack chain builder
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# chain.rs — attack chain builder

## Responsibility

`src/chain.rs` connects individual findings into composite attack chains using structured
attribute matching (CWE codes, scanner types, tags, severity, ZT pillars) rather than
fragile keyword search. Four `ChainRule`s (min 2 findings each):
- "Credential Compromise Chain" (credential + injection, multiplier 2.0)
- "Supply Chain Attack" (vulnerability + misconfig, multiplier 1.5)
- "Infrastructure Escalation" (misconfig + vulnerability, multiplier 1.8)
- "Data Exfiltration Path" (injection + credential, multiplier 2.5)

Plus three strategies: same-file grouping, same-directory grouping (fresh findings only),
multi-scanner confirmation chains (cross_refs ≥ 2), and ZT cross-pillar chains. Risk
scores capped at 10. Each chain gets an id `AC-NNN`, description, steps, finding_ids, and
recommendation.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, ...}`, `std::collections::{HashMap, HashSet}`.
- **Forbidden reaches:** must not import any other apeguard module.

## Invariants

- Chain rule predicates require structured attributes (CWE/tags/scanner); keyword fallback
  uses word-boundary matching (a bare `"key"` does not match `"monkey"`).
- Already-chained finding IDs are excluded from later grouping strategies to avoid
  duplicate chains.
- Rules with identical predicate sets (Credential Compromise and Data Exfiltration;
  Supply Chain Attack and Infrastructure Escalation) fire on the same finding set.
  `evaluate_rules_on_group` sorts matching rules by `risk_multiplier` descending and
  emits only the highest-multiplier rule per distinct finding set — no duplicate chains.

## Tests

Inline unit tests cover all four chain rules, same-directory chaining, single-finding/no-chain,
empty findings, supply chain, multi-scanner confirmation, infrastructure escalation,
scoring caps, tag/CWE matching, and ZT pillar chains (all in `src/chain.rs`).

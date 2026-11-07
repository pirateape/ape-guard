---
id: apeguard-policy
type: module-design
title: policy.rs — Rego/OPA policy-as-code engine
status: draft
parent: apeguard-architecture
depends-on: [apeguard-find]
references: []
implements: []
covers: []
tags: []
---

# policy.rs — Rego/OPA policy-as-code engine

## Responsibility

`src/policy.rs` evaluates Rego/OPA policies (via the `regorus` crate) against scan
findings. Policies are `.rego` files in a configurable directory (default
`.apeguard/policies/`); each defines an `apeguard.policy_actions` rule returning an array of
action objects `{ finding_id, action, severity?, tags?, message?, reason? }`.
`PolicyAction`: Block / Escalate / Downgrade / Flag / Tag. `PolicyResult` tallies policies
loaded, actions applied, and counts blocked/escalated/downgraded/flagged/tagged.
Pipeline placement: after severity filter, before report generation.

## Boundary

- **Allowed deps:** `find::{CanonicalFinding, Severity}`, `regorus`, `serde`,
  `std::path::{Path, PathBuf}`, `std::collections::HashMap`.
- **Forbidden reaches:** must not import `orchestrate`, `scanner`, `report`, `cli`, `main`.

## Notes

- Opt-in via `--policy` flag or `config.policy.enabled`; default `policy_dir:
  .apeguard/policies`.
- ~946 lines; partially stubbed (P3/P4 markers) — confirm which policy actions are fully
  wired vs planned.

# ApeGuard Governance

## Overview

ApeGuard is a source-available security scanning tool maintained under the Elastic License 2.0 (ELv2).
This document defines the project's governance structure, decision-making process,
and maintainer responsibilities.

## Maintainers

| Role | Person | Responsibilities |
|------|--------|------------------|
| Lead Maintainer | pirateape | Final decision authority, releases, security response |

## Decision Making

### Consensus Preferred
- All maintainers should agree on significant changes
- Disagreements resolved through discussion in GitHub Issues/PRs

### Lead Maintainer Authority
- If consensus cannot be reached, the Lead Maintainer makes the final decision
- Lead Maintainer may delegate decisions to other maintainers

### What Requires Consensus
- Breaking changes to CLI or MCP interfaces
- Adding new scanner layers
- Changes to scoring/grading algorithms
- License changes
- Governance changes

### What Can Be Decided by Single Maintainer
- Bug fixes and patches
- Documentation updates
- Dependency updates
- CI/CD improvements
- Test additions

## Contribution Path

1. **Contributor** — anyone who opens issues or submits PRs
2. **Trusted Contributor** — consistently high-quality contributions, invited by Lead Maintainer
3. **Maintainer** — can review/merge PRs, invited by Lead Maintainer

## Security Response

1. Report received via GitHub Security Advisories
2. Lead Maintainer acknowledges within 48 hours
3. Fix developed in private branch
4. CVE requested if applicable
5. Coordinated disclosure after fix is released

## Code of Conduct

All participants must follow the [Code of Conduct](CODE_OF_CONDUCT.md).
Violations may result in temporary or permanent exclusion from the project.

## Changes to This Document

This document can be modified by the Lead Maintainer with notice in the
project's GitHub Discussions or Issues.

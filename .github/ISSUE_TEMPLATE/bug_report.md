---
name: Bug Report
about: Report a problem to help us improve ApeGuard
title: ""
labels: bug
assignees: ""
---

## Describe the bug

A clear and concise description of what the bug is.

## Reproduction

**Command ran:**

```shell
apeguard scan --layers 1,2,3 --format md,json
```

**Config file** (if non-default):

```yaml
# paste your .apeguard.yaml here
```

**Target:** <!-- e.g., local directory, GitHub repo, specific project -->

## Expected behavior

What did you expect to happen?

## Actual behavior

What happened instead? Include error output, panics, or unexpected results.

## Environment

- **OS:** <!-- e.g., macOS 14.5, Ubuntu 22.04, Windows 11 -->
- **ApeGuard version:** <!-- `apeguard version` output -->
- **Scanner versions** (if relevant):
  - Gitleaks:
  - Semgrep:
  - Trivy:
  - Nuclei:
  - Checkov:
  - Syft:

## Additional context

- Are you using the binary release, Homebrew, or building from source?
- Does the issue reproduce with `--no-cache`?
- Any relevant raw scanner output?

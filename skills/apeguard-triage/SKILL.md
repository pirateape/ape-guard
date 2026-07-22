---
name: apeguard-triage
description: >
  ApeGuard-augmented triage skill. Use AFTER the generic loop-triage skill when
  operating on the ApeGuard repository itself. Pulls in ApeGuard's own scan
  output as the authoritative security/CI signal, plus GitHub Actions status,
  recent commits, and Cargo-generated test/clippy signals. Produces a
  prioritized findings report suitable for both STATE.md update and a human
  reviewer. Writes nothing to source files; updates only STATE.md and
  loop-run-log.md.
domain: security-ops
subdomain: vulnerability-management
tags:
  [
    apeguard,
    triage,
    security-scanning,
    ci,
    vulnerability-management,
    rust,
    reporting,
  ]
version: "1.0"
author: APE-Brain
license: MIT
user_invocable: true
---

# ApeGuard Triage Skill

You are an expert engineering triage agent specialized for the ApeGuard
repository. Your job is to produce a clean, prioritized list of things that the
ApeGuard loop should consider acting on — _and you use ApeGuard's own scan
output as your authoritative security/CI signal source wherever possible_.

## Inputs (gather these in order)

### 1. ApeGuard self-scan (preferred, authoritative)

If `target/release/apeguard` exists (or build it once with `cargo build
--release` if missing and the run budget allows), run:

```bash
./target/release/apeguard scan . --format sarif --quiet 2>&1 | tail -200
```

Treat this as the authoritative signal for:

- Secrets in the codebase (Gitleaks findings)
- SAST issues (Semgrep findings)
- SCA vulns (Trivy fs findings)
- IaC misconfigs (Checkov findings — relevant for `.github/workflows/*.yml`)
- SBOM drift (Syft findings)
- Context drift vs AGENTS.md/CLAUDE.md/.cursor rules (Layer 8)

If the binary is missing or the self-scan fails, fall through to GitHub
Actions logs (see step 2) and note the fallback in the run log.

### 2. CI/build signal (fallback when self-scan unavailable)

- Latest `ci.yml` workflow_run result for `main` branch
- Local `cargo test --no-run 2>&1 | tail -50` — capture compilation errors fast
- Local `cargo clippy -- -D warnings 2>&1 | tail -100` — current warning surface
- Local `cargo fmt --check 2>&1 | tail -50` — formatting drift

### 3. Prior state

Read `STATE.md` **first**, before any scanner runs. Identify:

- Items in "High Priority (waiting on human)" — are any resolved?
- Items in "Watch List" — any progress signals?
- `loop-pause-all: true` → exit immediately with no action (kill switch)

### 4. Recent commits (last 24–48h)

```bash
git log --oneline --since="2 days ago" -- main 2>&1 | head -30
```

Identify scope regressions: anything touching `src/scanner/*.rs`,
`src/find/mod.rs`, `src/pipeline.rs`, `src/report/mod.rs` is high-risk and
should be in Watch.

### 5. Dependency / supply chain sweep (weekly only)

On Mondays (or if CI checks the dependency tree):

- `cargo outdated 2>&1 | head -50` (if installed)
- `cargo audit 2>&1 | head -50` (if installed)
- `cargo deny check advisories 2>&1 | tail -30` (ApeGuard already has `deny.toml`)

### 6. MCP server audit (every run)

Scan all MCP configs for tool poisoning, SSRF, and unauthenticated exposure:

```bash
# Check standard MCP config locations
for conf in ~/.cursor/mcp.json ~/.vscode/mcp.json ~/.claude/mcp.json; do
  if [ -f "$conf" ]; then
    echo "=== MCP audit: $conf ==="
    python3 -c "
import json, sys
with open('$conf') as f:
    cfg = json.load(f)
    tools = cfg.get('mcpServers', {}).get('servers', {}) if isinstance(cfg.get('mcpServers'), dict) else {}
    for name, tool in tools.items():
        desc = tool.get('description', '') or ''
        if len(desc) > 500 or '<important>' in desc.lower() or 'do not tell the user' in desc.lower():
            print(f'POISONING RISK: {name} description={desc[:200]}')
        else:
            print(f'OK: {name} ({len(desc)} chars)')
" 2>&1
  fi
done

# If snyk-agent-scan is installed, run full audit
if command -v uvx &> /dev/null; then
  uvx snyk-agent-scan@latest --help 2>/dev/null && echo "snyk-agent-scan available (requires SNYK_TOKEN)"
fi
```

Flag as **High Priority** if tool poisoning, SSRF, or unauth exposure detected.

### 7. Terraform IaC audit (weekly only)

Check infrastructure-as-code for security misconfigurations:

```bash
# If checkov is installed
if command -v checkov &> /dev/null; then
  for tf_dir in .terraform ../infrastructure ../iac; do
    if [ -d "$tf_dir" ]; then
      echo "=== Terraform audit: $tf_dir ==="
      checkov -d "$tf_dir" 2>&1 | tail -50
    fi
  done
else
  echo "Terraform audit: checkov not installed (install via: brew install checkov)"
fi
```

Flag as **High Priority** if public S3 buckets, missing encryption, or overly permissive IAM detected.

### 8. AWS S3 bucket audit (weekly only)

Scan for publicly accessible S3 buckets and misconfigured permissions:

```bash
# If AWS CLI is configured
if command -v aws &> /dev/null && aws configure list 2>/dev/null | grep -q 'access_key'; then
  echo "=== AWS S3 audit ==="
  aws s3api list-buckets --query 'Buckets[].Name' --output json 2>/dev/null | \
    jq -r '.[]' | while read bucket; do
      echo "Bucket: $bucket"
      aws s3api get-bucket-acl --bucket "$bucket" --query 'Grants[].Permission' --output json 2>/dev/null
      aws s3api get-bucket-policy --bucket "$bucket" --query 'Policy.Statement' --output json 2>/dev/null | \
        grep -q '"Principal":"*"' && echo "⚠️ PUBLIC BUCKET: $bucket"
    done
else
  echo "AWS S3 audit: AWS CLI not configured (install via: brew install awscli)"
fi
```

Flag as **High Priority** if publicly accessible buckets or overly permissive ACLs detected.

### 9. TLS certificate audit (weekly only)

Scan for expired certs, shadow SSL, and CT log anomalies:

```bash
# If openssl is available
if command -v openssl &> /dev/null; then
  echo "=== TLS certificate audit ==="
  # Check local certs if available
  for cert in /etc/ssl/certs/*.pem /usr/local/etc/openssl/certs/*.pem; do
    if [ -f "$cert" ]; then
      openssl x509 -in "$cert" -noout -dates -subject 2>/dev/null | \
        grep -q 'expire' && echo "⚠️ EXPIRING/EXPIRED: $cert"
    fi
  done
else
  echo "TLS audit: openssl not installed (install via: brew install openssl)"
fi
```

Flag as **High Priority** if expired certs or shadow SSL detected.

## Output Format

Produce a markdown report with these exact sections (the opencode loop-driver
appends this to STATE.md):

### 1. High-Priority Items (act on these)

For each item:

- One-line description
- Why it matters (impact, risk, customer pain) — specific to ApeGuard's contract
- Suggested next loop action (one of: report-only / open worktree / draft PR / escalate-human)
- Effort estimate (small <1h / medium 1–4h / large >4h / xl multi-day)
- Source signal (which scanner found it: gitleaks|semgrep|trivy|checkov|syft|nuclei|context-drift|ci|cargo|mcp|terraform|aws-s3|tls)

### 2. Watch Items (monitor, do not act yet)

Same format, lower urgency. Default home for P5-C/D items and items the loop has surfaced <3 times.

### 3. Noise / Ignore

Brief list — closed items, dependabot-style noise, items triaged as low-confidence FP.

### 4. State Updates

Facts the loop should remember for next run:

- Resolved items (move to Noise next run)
- New items (append to High Priority or Watch)
- Timestamp + attempt-count updates
- Items that have been in "High Priority" >3 days → escalate to human review

## Rules

- **Brutally concise.** No narrative. The STATE.md consumer is a fast-skimming human.
- **No source edits.** You are triage, not implementer. Source edits break L1 contract.
- **ApeGuard self-scan is authoritative** when available — prefer its SARIF output over parsing GitHub logs.
- **Respect the pipeline invariants** in `AGENTS.md`: layers are orthogonal, CanonicalFinding is sacred, pipeline order is fixed.
- **Never propose architectural overhauls during triage.** This skill is for signal, not invention. Anything touching >5 files or the pipeline order goes to "High Priority (escalate-human)".
- **Escalate by default** on: Cargo.toml major bumps, deny.toml changes, release.yml changes, pre-commit config changes, secrets/auth/migrations paths (none exist yet but reserve).
- **Respect `loop-constraints.md`** rules — they are binding.
- **Kill switch check is mandatory** before any other action. If `loop-pause-all: true` in STATE.md, exit immediately with a one-line log entry and no further work.

## Post-Run Critique (mandatory)

At the end of every run, append to `STATE.md` under "Post-Run Critique":

- High-noise items (false positives in this run)
- Items that should be deprioritized
- Re-prioritizations applied
- One change to improve the next cycle

## State Updates (mandatory)

Always update `STATE.md`:

- `Last run:` timestamp → current ISO timestamp
- Resolved items moved to Noise
- New High-Priority items appended with attempt-count: 0
- Watch items touched with last-review timestamp
- append a run-log entry to `loop-run-log.md`:
  `<ISO> | daily-triage | <duration>s | <findings> | <actions> | <escalations> | <tokens_est> | <outcome>`

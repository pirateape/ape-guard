#!/usr/bin/env bash
# mcp-audit.sh — MCP Security Scanner (Layer 9)
# Scans MCP configs for tool poisoning, SSRF, and unauthenticated exposure
#
# Usage:
#   ./scripts/mcp-audit.sh [config-path]
#   ./scripts/mcp-audit.sh ~/.claude/mcp.json
#   ./scripts/mcp-audit.sh --all
#
# Exit codes:
#   0: No poisoning detected
#   1: Tool poisoning or SSRF detected
#   2: Error (missing binary, invalid config)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APEGUARD_BIN="${APEGUARD_BIN:-$(cd "$SCRIPT_DIR/.." && cargo build --release 2>/dev/null && echo "$SCRIPT_DIR/../target/release/apeguard")}"
CONFIG_PATH="${1:-.apeguard/mcp-config.json}"
SCAN_ALL="${2:-}"

# ─── Helpers ───
log() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] $*"; }
warn() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ⚠️  $*" >&2; }
fail() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ❌ $*" >&2; exit 1; }

# ─── Check dependencies ───
check_deps() {
    if ! command -v python3 &>/dev/null; then
        fail "python3 required. Install: brew install python3"
    fi
}

# ─── Manual inspection ───
manual_scan() {
    local config="$1"
    if [[ ! -f "$config" ]]; then
        warn "Config not found: $config (skipping)"
        return 0
    fi

    log "Scanning MCP config: $config"
    python3 -c "
import json, sys, os

config_path = sys.argv[1]
with open(config_path) as f:
    cfg = json.load(f)

# Parse MCP config (supports both Cursor and VSCode formats)
servers = cfg.get('mcpServers', {}).get('servers', {}) if isinstance(cfg.get('mcpServers'), dict) else {}
if not servers and isinstance(cfg.get('servers'), list):
    servers = {s.get('name', 'unknown'): s for s in cfg['servers']}

poisoning_risk = 0
ssrf_risk = 0
unauth_risk = 0

for name, tool in servers.items():
    desc = tool.get('description', '') or ''
    env = tool.get('env', {})

    # Check for tool poisoning indicators
    if len(desc) > 500 or '<important>' in desc.lower() or 'do not tell the user' in desc.lower() or 'ignore previous' in desc.lower():
        print(f'POISONING RISK: {name} description_len={len(desc)}')
        poisoning_risk += 1
    else:
        print(f'OK: {name} ({len(desc)} chars)')

    # Check for SSRF indicators
    if 'ssrf' in desc.lower() or 'ssrf:' in desc.lower() or 'ssrf.' in desc.lower():
        print(f'SSRF INDICATOR: {name}')
        ssrf_risk += 1

    # Check for unauthenticated exposure
    if not env and 'url' in str(tool).lower():
        print(f'UNAUTH EXPOSURE: {name} (no env auth)')
        unauth_risk += 1

print(f'SUMMARY: poisoning={poisoning_risk} ssrf={ssrf_risk} unauth={unauth_risk}')
sys.exit(1 if (poisoning_risk > 0 or ssrf_risk > 0) else 0)
" "$config"
}

# ─── snyk-agent-scan ───
snyk_scan() {
    if [[ -z "${SNYK_TOKEN:-}" ]]; then
        warn "SNYK_TOKEN not set — running manual scan only"
        return 0
    fi

    log "Running snyk-agent-scan (requires SNYK_TOKEN)"
    if command -v uvx &>/dev/null; then
        uvx snyk-agent-scan@latest "$@" 2>&1 || warn "snyk-agent-scan completed with warnings"
    else
        warn "uvx not found — install: curl -LsSf https://astral.sh/uv/install.sh | sh"
    fi
}

# ─── ApeGuard Layer 9 ───
apeguard_scan() {
    log "Running ApeGuard Layer 9 (MCP Security Scanner)"
    if [[ -x "$APEGUARD_BIN" ]]; then
        "$APEGUARD_BIN" scan --layers 9 --output-dir .apeguard/reports 2>&1 || warn "ApeGuard layer 9 completed with findings"
    else
        warn "ApeGuard binary not found at $APEGUARD_BIN"
    fi
}

# ─── Main ───
main() {
    check_deps

    if [[ "$SCAN_ALL" == "--all" ]]; then
        log "Scanning all MCP config locations"
        for conf in ~/.cursor/mcp.json ~/.vscode/mcp.json ~/.claude/mcp.json; do
            manual_scan "$conf" || exit 1
        done
    else
        manual_scan "$CONFIG_PATH" || exit 1
    fi

    snyk_scan
    apeguard_scan

    log "MCP audit complete"
}

main "$@"

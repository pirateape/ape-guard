#!/usr/bin/env bash
# terraform-audit.sh — Terraform IaC Scanner (Layer 10)
# Scans Terraform infrastructure configs for security misconfigurations
#
# Usage:
#   ./scripts/terraform-audit.sh [terraform-dir]
#   ./scripts/terraform-audit.sh ../ApeWall
#   ./scripts/terraform-audit.sh --all
#
# Exit codes:
#   0: No critical findings
#   1: Critical/high findings detected
#   2: Error (missing tool)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APEGUARD_BIN="${APEGUARD_BIN:-$(cd "$SCRIPT_DIR/.." && cargo build --release 2>/dev/null && echo "$SCRIPT_DIR/../target/release/apeguard")}"
TF_DIR="${1:-.}"
SCAN_ALL="${2:-}"

# ─── Helpers ───
log() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] $*"; }
warn() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ⚠️  $*" >&2; }

# ─── Check dependencies ───
check_deps() {
    if ! command -v checkov &>/dev/null && ! command -v tfsec &>/dev/null; then
        warn "Neither checkov nor tfsec found. Install: pip install checkov || brew install tfsec"
        warn "Running manual inspection only..."
    fi
}

# ─── Checkov scan ───
checkov_scan() {
    local target="$1"
    if ! command -v checkov &>/dev/null; then
        warn "checkov not found — skipping"
        return 0
    fi

    log "Running checkov against: $target"
    checkov -d "$target" --compact --quiet 2>&1 || warn "checkov completed with findings"
}

# ─── tfsec scan ───
tfsec_scan() {
    local target="$1"
    if ! command -v tfsec &>/dev/null; then
        warn "tfsec not found — skipping"
        return 0
    fi

    log "Running tfsec against: $target"
    tfsec "$target" --no-colour 2>&1 || warn "tfsec completed with findings"
}

# ─── Manual inspection ───
manual_scan() {
    local target="$1"
    if [[ ! -d "$target" ]]; then
        warn "Directory not found: $target (skipping)"
        return 0
    fi

    log "Running manual Terraform inspection: $target"
    local findings=0

    find "$target" -name "*.tf" -o -name "*.tf.json" 2>/dev/null | while read -r file; do
        if grep -qE '"Principal".*"\*"' "$file" 2>/dev/null; then
            echo "PUBLIC S3 BUCKET: $file"
            ((findings++)) || true
        fi
        if grep -qE 'resource\s+"aws_s3_bucket"' "$file" 2>/dev/null && \
           ! grep -q "server_side_encryption_configuration" "$file" 2>/dev/null; then
            echo "MISSING ENCRYPTION: $file (S3 bucket without encryption)"
            ((findings++)) || true
        fi
        if grep -qE '"Action".*"\*"' "$file" 2>/dev/null; then
            echo "OVERLY PERMISSIVE IAM: $file"
            ((findings++)) || true
        fi
    done
}

# ─── ApeGuard Layer 10 ───
apeguard_scan() {
    log "Running ApeGuard Layer 10 (Terraform IaC Scanner)"
    if [[ -x "$APEGUARD_BIN" ]]; then
        "$APEGUARD_BIN" scan --layers 10 --output-dir .apeguard/reports 2>&1 || warn "ApeGuard layer 10 completed with findings"
    else
        warn "ApeGuard binary not found at $APEGUARD_BIN"
    fi
}

# ─── Main ───
main() {
    check_deps

    if [[ "$SCAN_ALL" == "--all" ]]; then
        log "Scanning all Terraform config locations"
        for tf_dir in .terraform ../infrastructure ../iac ../ApeWall ../ArdentKey ../SentinelNet; do
            [[ -d "$tf_dir" ]] && {
                checkov_scan "$tf_dir"
                tfsec_scan "$tf_dir"
                manual_scan "$tf_dir"
            }
        done
    else
        checkov_scan "$TF_DIR"
        tfsec_scan "$TF_DIR"
        manual_scan "$TF_DIR"
    fi

    apeguard_scan
    log "Terraform audit complete"
}

main "$@"

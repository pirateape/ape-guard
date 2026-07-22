#!/usr/bin/env bash
# aws-s3-audit.sh — AWS S3 Bucket Scanner (Layer 11)
# Scans for publicly accessible S3 buckets and misconfigured permissions
#
# Usage:
#   ./scripts/aws-s3-audit.sh [config-path]
#   ./scripts/aws-s3-audit.sh --all
#   # Requires AWS credentials (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
#
# Exit codes:
#   0: No critical findings
#   1: Critical/high findings detected
#   2: Error (missing AWS CLI, no credentials)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APEGUARD_BIN="${APEGUARD_BIN:-$(cd "$SCRIPT_DIR/.." && cargo build --release 2>/dev/null && echo "$SCRIPT_DIR/../target/release/apeguard")}"
CONFIG_PATH="${1:-.apeguard/aws-config.json}"
SCAN_ALL="${2:-}"

# ─── Helpers ───
log() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] $*"; }
warn() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ⚠️  $*" >&2; }
fail() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ❌ $*" >&2; exit 1; }

# ─── Check dependencies ───
check_deps() {
    if ! command -v aws &>/dev/null; then
        warn "aws CLI not found. Install: brew install awscli"
        fail "AWS CLI required"
    fi
    if [[ -z "${AWS_ACCESS_KEY_ID:-}" ]]; then
        warn "AWS_ACCESS_KEY_ID not set — running manual scan only"
    fi
}

# ─── AWS CLI scan ───
aws_scan() {
    log "Listing S3 buckets"
    aws s3api list-buckets --query "Buckets[].{Name:Name,LastModified:LastModified}" --output json 2>&1

    # Check each bucket for public access
    local public_buckets=0
    local unencrypted_buckets=0

    for bucket in $(aws s3api list-buckets --query "Buckets[].Name" --output text 2>/dev/null); do
        log "Checking bucket: $bucket"

        # Check ACL
        local acl
        acl=$(aws s3api get-bucket-acl --bucket "$bucket" --output json 2>/dev/null || echo "{}")
        if echo "$acl" | grep -q '"Grantee.*URI.*AllUsers"'; then
            echo "PUBLIC ACL: $bucket"
            ((public_buckets++)) || true
        fi

        # Check public access block
        local pab
        pab=$(aws s3api get-public-access-block --bucket "$bucket" 2>&1 || echo "")
        if echo "$pab" | grep -q "AccessBlockNotFound"; then
            echo "NO PUBLIC ACCESS BLOCK: $bucket"
            ((public_buckets++)) || true
        fi

        # Check encryption
        local encryption
        encryption=$(aws s3api get-bucket-encryption --bucket "$bucket" 2>&1 || echo "")
        if echo "$encryption" | grep -q "ServerSideEncryptionConfigurationNotFoundError"; then
            echo "NO ENCRYPTION: $bucket"
            ((unencrypted_buckets++)) || true
        fi
    done

    echo "SUMMARY: public=$public_buckets unencrypted=$unencrypted_buckets"
    ((public_buckets > 0 || unencrypted_buckets > 0)) && return 1 || return 0
}

# ─── Prowler scan ───
prowler_scan() {
    if ! command -v prowler &>/dev/null; then
        warn "prowler not found — skipping"
        return 0
    fi

    log "Running prowler"
    prowler aws 2>&1 || warn "prowler completed with findings"
}

# ─── Manual inspection ───
manual_scan() {
    local config="$1"
    if [[ ! -f "$config" ]]; then
        warn "Config not found: $config (skipping)"
        return 0
    fi

    log "Scanning AWS config: $config"
    local content
    content=$(cat "$config")

    if echo "$content" | grep -qE '"Principal".*"\*"' || echo "$content" | grep -qE '"Principal":"\*"'; then
        echo "PUBLIC S3 BUCKET: Bucket with public access policy"
    fi
    if echo "$content" | grep -q "BucketEncryption" && echo "$content" | grep -q "Status: Disabled"; then
        echo "MISSING ENCRYPTION: S3 bucket encryption disabled"
    fi
}

# ─── ApeGuard Layer 11 ───
apeguard_scan() {
    log "Running ApeGuard Layer 11 (AWS S3 Scanner)"
    if [[ -x "$APEGUARD_BIN" ]]; then
        "$APEGUARD_BIN" scan --layers 11 --output-dir .apeguard/reports 2>&1 || warn "ApeGuard layer 11 completed with findings"
    else
        warn "ApeGuard binary not found at $APEGUARD_BIN"
    fi
}

# ─── Main ───
main() {
    check_deps

    if [[ "$SCAN_ALL" == "--all" ]]; then
        log "Scanning all AWS config locations"
        for conf in .apeguard/aws-config.json ~/.aws/config; do
            manual_scan "$conf"
        done
    elif [[ -n "${AWS_ACCESS_KEY_ID:-}" ]]; then
        aws_scan || exit 1
        prowler_scan
    else
        manual_scan "$CONFIG_PATH"
    fi

    apeguard_scan
    log "AWS S3 audit complete"
}

main "$@"

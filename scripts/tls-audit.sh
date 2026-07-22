#!/usr/bin/env bash
# tls-audit.sh — TLS Certificate Scanner (Layer 12)
# Scans for expired certs, shadow SSL, and CT log anomalies
#
# Usage:
#   ./scripts/tls-audit.sh [cert-path]
#   ./scripts/tls-audit.sh /etc/ssl/certs/ca-certificates.crt
#   ./scripts/tls-audit.sh --all
#
# Exit codes:
#   0: All certs valid
#   1: Expired/expiring cert detected
#   2: Error (missing openssl)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APEGUARD_BIN="${APEGUARD_BIN:-$(cd "$SCRIPT_DIR/.." && cargo build --release 2>/dev/null && echo "$SCRIPT_DIR/../target/release/apeguard")}"
CERT_PATH="${1:-}"
SCAN_ALL="${2:-}"

# ─── Helpers ───
log() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] $*"; }
warn() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ⚠️  $*" >&2; }
fail() { echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ❌ $*" >&2; exit 1; }

# ─── Check dependencies ───
check_deps() {
    if ! command -v openssl &>/dev/null; then
        fail "openssl required. Install: brew install openssl (macOS) or apt install openssl (Linux)"
    fi
}

# ─── Single cert check ───
check_cert() {
    local cert="$1"
    if [[ ! -f "$cert" ]]; then
        warn "Cert not found: $cert (skipping)"
        return 0
    fi

    log "Checking cert: $cert"
    local dates
    dates=$(openssl x509 -in "$cert" -noout -dates -subject 2>/dev/null) || {
        warn "Invalid cert: $cert"
        return 0
    }

    echo "$dates"

    # Check expiry (30 days)
    if ! openssl x509 -checkend 2592000 -in "$cert" -noout 2>/dev/null; then
        warn "CERT EXPIRING/EXPIRED (within 30 days): $cert"
        return 1
    fi

    # Check expiry (7 days) for critical
    if ! openssl x509 -checkend 604800 -in "$cert" -noout 2>/dev/null; then
        log "CERT EXPIRING (within 7 days): $cert"
    fi

    log "OK: $cert"
    return 0
}

# ─── Directory scan ───
scan_directory() {
    local dir="$1"
    local has_expiring=0

    if [[ ! -d "$dir" ]]; then
        warn "Directory not found: $dir (skipping)"
        return 0
    fi

    log "Scanning certs in: $dir"
    for cert in "$dir"/*.pem "$dir"/*.crt; do
        [[ -f "$cert" ]] && {
            if ! check_cert "$cert"; then
                has_expiring=1
            fi
        }
    done

    return $has_expiring
}

# ─── ApeGuard Layer 12 ───
apeguard_scan() {
    log "Running ApeGuard Layer 12 (TLS Certificate Scanner)"
    if [[ -x "$APEGUARD_BIN" ]]; then
        "$APEGUARD_BIN" scan --layers 12 --output-dir .apeguard/reports 2>&1 || warn "ApeGuard layer 12 completed with findings"
    else
        warn "ApeGuard binary not found at $APEGUARD_BIN"
    fi
}

# ─── Main ───
main() {
    check_deps

    if [[ "$SCAN_ALL" == "--all" ]]; then
        log "Scanning all certificate locations"
        local has_expiring=0

        for dir in /etc/ssl/certs /usr/local/etc/ssl/certs /opt/homebrew/etc/openssl/certs; do
            if ! scan_directory "$dir"; then
                has_expiring=1
            fi
        done

        [[ $has_expiring -eq 1 ]] && {
            warn "Some certs are expiring/expired"
            exit 1
        }
    elif [[ -n "$CERT_PATH" ]]; then
        check_cert "$CERT_PATH" || exit 1
    else
        warn "No cert path provided. Use: $0 <cert-path> or $0 --all"
        exit 1
    fi

    apeguard_scan
    log "TLS audit complete"
}

main "$@"

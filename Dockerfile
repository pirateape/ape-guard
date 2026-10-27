# =============================================================================
# ApeGuard — Multi-stage Docker build
# =============================================================================
# Stage 1: Build the binary
FROM rust:1.99-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked 2>/dev/null || true
RUN rm -rf src

COPY src/ src/
RUN touch src/main.rs && cargo build --release --locked

# =============================================================================
# Stage 2: Runtime image with scanner tools
FROM debian:bookworm-slim

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    curl \
    jq \
    unzip \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# Install Gitleaks (Layer 1) with checksum verification from upstream checksums file
ARG TARGETARCH
ARG GITLEAKS_VERSION=8.24.2
RUN ARCH=${TARGETARCH:-amd64} && \
    case "$ARCH" in \
      amd64) GL_ARCH="x64" ;; \
      arm64) GL_ARCH="arm64" ;; \
      *) echo "Unsupported arch: $ARCH"; exit 1 ;; \
    esac && \
    TARBALL="gitleaks_${GITLEAKS_VERSION}_linux_${GL_ARCH}.tar.gz" && \
    curl -sSfLO "https://github.com/gitleaks/gitleaks/releases/download/v${GITLEAKS_VERSION}/${TARBALL}" && \
    curl -sSfLO "https://github.com/gitleaks/gitleaks/releases/download/v${GITLEAKS_VERSION}/gitleaks_${GITLEAKS_VERSION}_checksums.txt" && \
    grep " ${TARBALL}\$" "gitleaks_${GITLEAKS_VERSION}_checksums.txt" | sha256sum -c - && \
    tar xz -f "${TARBALL}" -C /usr/local/bin gitleaks && \
    rm -f "${TARBALL}" "gitleaks_${GITLEAKS_VERSION}_checksums.txt"

# Install Trivy (Layer 3 + 4) with checksum verification from upstream checksums file
ARG TARGETARCH
ARG TRIVY_VERSION=0.70.0
RUN ARCH=${TARGETARCH:-amd64} && \
    case "$ARCH" in \
      amd64) TV_ARCH="64bit" ;; \
      arm64) TV_ARCH="ARM64" ;; \
      *) echo "Unsupported arch: $ARCH"; exit 1 ;; \
    esac && \
    TARBALL="trivy_${TRIVY_VERSION}_Linux-${TV_ARCH}.tar.gz" && \
    curl -sSfLO "https://github.com/aquasecurity/trivy/releases/download/v${TRIVY_VERSION}/${TARBALL}" && \
    curl -sSfLO "https://github.com/aquasecurity/trivy/releases/download/v${TRIVY_VERSION}/trivy_${TRIVY_VERSION}_checksums.txt" && \
    grep " ${TARBALL}\$" "trivy_${TRIVY_VERSION}_checksums.txt" | sha256sum -c - && \
    tar xz -f "${TARBALL}" -C /usr/local/bin trivy && \
    rm -f "${TARBALL}" "trivy_${TRIVY_VERSION}_checksums.txt"

# Install Semgrep (Layer 2) — pinned version in virtualenv to avoid PEP 668 conflicts
ARG SEMGREP_VERSION=1.110.0
RUN python3 -m venv /opt/semgrep && \
    /opt/semgrep/bin/pip install --no-cache-dir "semgrep==${SEMGREP_VERSION}" && \
    ln -sf /opt/semgrep/bin/semgrep /usr/local/bin/semgrep

# Install Nuclei (Layer 5) with checksum verification from upstream checksums file
ARG TARGETARCH
ARG NUCLEI_VERSION=3.8.0
RUN ARCH=${TARGETARCH:-amd64} && \
    ZIPFILE="nuclei_${NUCLEI_VERSION}_linux_${ARCH}.zip" && \
    curl -sSfLO "https://github.com/projectdiscovery/nuclei/releases/download/v${NUCLEI_VERSION}/${ZIPFILE}" && \
    curl -sSfLO "https://github.com/projectdiscovery/nuclei/releases/download/v${NUCLEI_VERSION}/nuclei_${NUCLEI_VERSION}_checksums.txt" && \
    grep " ${ZIPFILE}\$" "nuclei_${NUCLEI_VERSION}_checksums.txt" | sha256sum -c - && \
    unzip -o "${ZIPFILE}" nuclei -d /usr/local/bin/ && \
    rm -f "${ZIPFILE}" "nuclei_${NUCLEI_VERSION}_checksums.txt"

COPY --from=builder /app/target/release/apeguard /usr/local/bin/apeguard

# Create a non-root user for security
RUN groupadd -r apeguard && useradd -r -g apeguard -d /target -s /sbin/nologin apeguard && \
    mkdir -p /target && chown -R apeguard:apeguard /target

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD apeguard version > /dev/null || exit 1

USER apeguard
WORKDIR /target
ENTRYPOINT ["apeguard"]
CMD ["scan", "."]

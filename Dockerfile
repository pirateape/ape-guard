# =============================================================================
# ApeGuard — Multi-stage Docker build
# =============================================================================
# Stage 1: Build the binary
FROM rust:1.94-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY src/ src/
RUN touch src/main.rs
RUN cargo build --release

# =============================================================================
# Stage 2: Runtime image with scanner tools
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    curl \
    jq \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# Install Gitleaks (Layer 1)
ARG TARGETARCH
RUN ARCH=${TARGETARCH:-amd64} && \
    case "$ARCH" in \
      amd64) GL_ARCH="x64" ;; \
      arm64) GL_ARCH="arm64" ;; \
      *) echo "Unsupported arch: $ARCH"; exit 1 ;; \
    esac && \
    curl -sSfL "https://github.com/gitleaks/gitleaks/releases/download/v8.24.2/gitleaks_8.24.2_linux_${GL_ARCH}.tar.gz" \
    | tar xz -C /usr/local/bin gitleaks

# Install Trivy (Layer 3 + 4) — latest stable
ARG TARGETARCH
RUN ARCH=${TARGETARCH:-amd64} && \
    case "$ARCH" in \
      amd64) TV_ARCH="64bit" ;; \
      arm64) TV_ARCH="ARM64" ;; \
      *) echo "Unsupported arch: $ARCH"; exit 1 ;; \
    esac && \
    curl -sSfL "https://github.com/aquasecurity/trivy/releases/download/v0.70.0/trivy_0.70.0_Linux-${TV_ARCH}.tar.gz" \
    | tar xz -C /usr/local/bin trivy

# Install Semgrep (Layer 2) — use virtualenv to avoid PEP 668 conflicts
RUN python3 -m venv /opt/semgrep && \
    /opt/semgrep/bin/pip install --no-cache-dir semgrep && \
    ln -sf /opt/semgrep/bin/semgrep /usr/local/bin/semgrep

# Install Nuclei (Layer 5)
ARG TARGETARCH
RUN ARCH=${TARGETARCH:-amd64} && \
    apt-get update && apt-get install -y --no-install-recommends unzip && rm -rf /var/lib/apt/lists/* && \
    curl -sSfL "https://github.com/projectdiscovery/nuclei/releases/download/v3.8.0/nuclei_3.8.0_linux_${ARCH}.zip" \
    -o /tmp/nuclei.zip && \
    unzip -o /tmp/nuclei.zip nuclei -d /usr/local/bin/ && \
    rm /tmp/nuclei.zip

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

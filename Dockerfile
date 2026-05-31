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
    && rm -rf /var/lib/apt/lists/*

# Install Gitleaks (Layer 1)
RUN curl -sSfL https://github.com/gitleaks/gitleaks/releases/download/v8.18.2/gitleaks_8.18.2_linux_x64.tar.gz \
    | tar xz -C /usr/local/bin gitleaks

# Install Trivy (Layer 3 + 4)
RUN curl -sSfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh \
    | sh -s -- -b /usr/local/bin v0.50.1

# Install Semgrep (Layer 2)
RUN pip3 install --no-cache-dir semgrep

# Install Nuclei (Layer 5 — optional)
RUN curl -sSfL https://github.com/projectdiscovery/nuclei/releases/download/v3.2.9/nuclei_3.2.9_linux_amd64.tar.gz \
    | tar xz -C /usr/local/bin nuclei

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

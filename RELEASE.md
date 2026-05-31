# Release Process

This document describes how to publish a new ApeGuard release.

## Checklist

### 1. Update Version

```bash
# Edit Cargo.toml — bump version
cargo build  # verify build
cargo test   # verify tests pass
cargo clippy -- -D warnings
cargo fmt --check
```

### 2. Commit and Tag

```bash
git add Cargo.toml
git commit -m "chore: bump version to x.y.z"
git tag -a vx.y.z -m "vx.y.z — release title"
git push origin main
git push origin vx.y.z
```

### 3. Wait for CI

The release workflow triggers automatically on `v*` tags and:

- Builds binaries for 4 platforms (linux x86_64, macOS x86_64, macOS arm64, windows x86_64)
- Generates SHA256 checksums per binary
- Attests build provenance (SLSA Level 1+)
- Creates a GitHub release with auto-generated release notes

### 4. Update Homebrew Tap

```bash
# Compute new SHA256
SHA=$(curl -sL "https://github.com/pirateape/ape-guard/archive/refs/tags/vx.y.z.tar.gz" | shasum -a 256 | cut -d' ' -f1)

# Clone and update
git clone https://github.com/pirateape/homebrew-tap.git /tmp/homebrew-tap
cd /tmp/homebrew-tap
sed -i '' "s/version \"[^\"]*\"/version \"x.y.z\"/" Formula/apeguard.rb
sed -i '' "s/sha256 \".*\"/sha256 \"$SHA\"/" Formula/apeguard.rb
git add Formula/
git commit -m "Update apeguard to vx.y.z"
git push origin main

# Verify
brew untap pirateape/tap && brew tap pirateape/tap
brew info apeguard
```

### 5. Verify Installation

```bash
brew reinstall apeguard
apeguard version
apeguard scan . --layers 1 --format md --no-cache
```

## Versioning

ApeGuard follows [Semantic Versioning](https://semver.org/):
- **MAJOR** (x.0.0): Breaking changes to CLI interface, output formats, or config
- **MINOR** (0.x.0): New features, new scanners, new report formats
- **PATCH** (0.0.x): Bug fixes, performance improvements, scanner updates

## Version Matrix

| Version | Date       | Highlights                              |
|---------|------------|-----------------------------------------|
| 0.3.0   | 2026-05-31 | Checkov (IaC) + Syft (SBOM) layers, all 7 layers now available, Gitleaks reliability fix, CI mode safety |
| 0.2.0   | 2026-05-31 | Parallel scanning, HTML charts, severity-weighted ZT scorecard, --ci flag, multi-arch Docker |
| 0.1.0   | 2026-05-31 | Initial release: 5-layer scanning, MCP, ZT scorecard, HTML reports |

## Branch Protection

The `main` branch is protected:
- Direct pushes blocked — changes must go through PRs
- All CI checks must pass (4 checks: 3 platform builds + format)
- Pull request review required
- Stale reviews are dismissed on new commits
- Conversation must be resolved before merging

# Contributing to ApeGuard

Thanks for your interest in contributing.

## Before you start

- **Bugs** — open a [Bug Report](https://github.com/pirateape/ApeGuard/issues/new?labels=bug&template=bug_report.md) with reproduction steps and scanner versions.
- **Feature ideas** — open a [Feature Request](https://github.com/pirateape/ApeGuard/issues/new?labels=enhancement&template=feature_request.md) first to discuss before coding.
- **Security vulnerabilities** — do **not** open a public issue. Report via [GitHub Security Advisories](https://github.com/pirateape/ApeGuard/security/advisories).

## Pull requests

1. Open an issue first for non-trivial changes.
2. Ensure `cargo test` passes and `cargo fmt --check` is clean.
3. Keep PRs focused — one change per PR.
4. **Add tests for new functionality** — all major new features must include unit or integration tests.
5. **DCO sign-off** — all commits must include a `Signed-off-by` line. Use `git commit -s` to add it automatically.

## Coding standards

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo clippy -- -D warnings` — zero warnings policy
- Use `cargo fmt --check` — consistent formatting
- Prefer `Result` and `Option` over `unwrap()` / `expect()` in library code
- Document all public items with `///` doc-comments
- Keep functions small and focused — extract helpers when >50 lines

## Code of Conduct

All participants must follow our [Code of Conduct](CODE_OF_CONDUCT.md).

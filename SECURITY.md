# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.3.x   | ✅ Active          |
| 0.2.x   | ✅ Active          |
| 0.1.x   | ⚪ Legacy           |

## Reporting a Vulnerability

ApeGuard is a security scanning tool designed to **find** vulnerabilities in your
infrastructure. If you discover a security issue in ApeGuard itself, please report
it responsibly.

**Do not** open a public GitHub issue for security vulnerabilities.

Instead, email the maintainer at **pirateape@proton.me** or open a
[confidential advisory](https://github.com/pirateape/ape-guard/security/advisories/new).

You should receive a response within 48 hours. If you don't, please follow up.

## Security Expectations

- ApeGuard runs 100% locally with zero exfiltration.
- No telemetry, no analytics, no network calls for core functionality.
- LLM remediation via local Ollama is opt-in and never sends data externally.
- All dependencies are auditable via `cargo audit` and Dependabot.


# Security Policy

## Reporting a Vulnerability

RytScan is a security tool, so its own codebase must stay clean. If you find a
vulnerability in RytScan — a bug that produces false negatives/positives, a
crash, a flaw in the SARIF/JSON output, or a security issue in the tool itself —
**do not open a public issue.**

Report it privately via
[GitHub Security Advisories](../../security/advisories).

### What to include

1. **Affected version** — commit hash or crate version (`rytscan --version`)
2. **Description** — what the vulnerability is and its impact
3. **Reproduction** — minimal source snippet or fixture that triggers it
4. **Expected vs actual** — including exit codes and output (text, JSON, or SARIF)

### Response targets

| Timeframe | Promise |
|---|---|
| Acknowledgement | within **48 hours** |
| Fix for critical issues | within **7 days** |
| Coordinated disclosure | we coordinate with you before any public writeup |

## Security posture

RytScan is designed to be safe to run anywhere:

- **No network access** — the scanner never sends source code off-machine.
  All analysis is local file reads + in-memory rule matching.
- **Dependency auditing** — CI runs `cargo audit` on every push/PR; the
  `cargo audit` gate must stay green for merges.
- **Dependency automation** — Dependabot keeps crates updated and CI enforces
  `cargo clippy -- -D warnings`, so the tool itself stays lint-clean.
- **SARIF for Code Scanning** — findings are emitted in SARIF 2.1.0 and can be
  uploaded to GitHub Code Scanning for the Security tab.
- **Fail closed in CI** — the default `--fail-on high` exit code blocks merges
  on high-severity findings.

## Scope

In scope for security reports:

- The `rytscan-core` and `rytscan-cli` crates
- The rule engine and function-extraction logic (`crates/rytscan-core/src/rules.rs`)
- The SARIF / JSON report serializers
- The CI workflow definitions in `.github/workflows/`

Out of scope: vulnerabilities in the contracts RytScan *scans* (report those to
the contract maintainers), and general issues in third-party crates already
tracked by `cargo audit`.

## Supported Versions

| Version | Supported |
| --- | --- |
| `main` | ✅ |
| Latest tagged release (`vX.Y.Z`) | ✅ |

## Safe Harbor

We will not pursue legal action against researchers who report vulnerabilities
in good faith: you act in good faith, do not access or destroy data beyond
demonstrating the vulnerability, and allow us a reasonable window to respond
before any public disclosure. We thank you for helping keep RytScan — and the
Soroban contracts it protects — safe.

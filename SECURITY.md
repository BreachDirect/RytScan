# Security Policy

## Reporting a Vulnerability

RytScan is a security tool, and its own codebase must stay clean. If you find
a vulnerability in RytScan — a bug that produces false negatives/positives, a
crash, a flaw in the SARIF output, or a security issue in the tool itself —
**do not open a public issue.**

Report it privately via [GitHub Security Advisories](../../security/advisories).

### What to include

1. **Affected version** — commit hash or crate version
2. **Description** — what the vulnerability is and its impact
3. **Reproduction** — minimal source snippet or fixture that triggers it
4. **Expected vs actual** — including exit codes and output

We aim to acknowledge reports within **48 hours** and ship a fix within
**7 days** for critical issues.

## Scope

- The `rytscan-core` and `rytscan-cli` crates
- The SARIF / JSON report serializers
- The CI workflow definitions in `.github/workflows/`

## Supported Versions

| Version | Supported |
| --- | --- |
| main | ✅ |

## Safe Harbor

We will not pursue legal action against researchers who report vulnerabilities
in good faith, act in good faith, and do not access or destroy data beyond
demonstrating the vulnerability.

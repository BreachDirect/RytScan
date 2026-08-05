# RytScan

**Soroban smart contract security scanner for the Stellar ecosystem.**

![CI](https://github.com/BreachDirect/RytScan/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.95-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rules](https://img.shields.io/badge/rules-9-green)
![SARIF](https://img.shields.io/badge/SARIF-2.1.0-yellow)

RytScan is a lightweight static analysis CLI that helps Wave contributors and Soroban developers catch common security issues before testnet deployment — missing authorization, panic-prone error handling, unchecked token transfers, missing events, unsafe arithmetic, and unsafe storage patterns.

Built for [Stellar Wave 8](https://www.drips.network/wave/stellar) (August 2026).

## Why RytScan?

| Wave need | RytScan coverage |
|---|---|
| Soroban contract security before merge | 9 built-in rules (AUTH, PANIC, TOKEN, EVENT, TTL, STORE, ARITH, ASSERT, UNSAFE) |
| Event emission for indexers | EVENT-001 flags state changes without `env.events().publish` |
| CI gate for contract PRs | `--fail-on high` exit code + SARIF for GitHub Code Scanning |
| Contributor onboarding | Fixture contracts + `rytscan rules` catalog |

Complements ecosystem tools like [OpenZeppelin soroban-scanner](https://github.com/OpenZeppelin/soroban-security-detectors-sdk) and [Sanctifier](https://github.com/HyperSafeD/Sanctifier) with a Wave-focused, fast, zero-config entry point.

## Quick Start

```bash
# Build
cargo build --release

# Scan a contract directory
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src

# JSON output for CI
cargo run -p rytscan-cli -- scan ./my-contract --format json --fail-on high

# SARIF v2.1.0 output (GitHub Code Scanning)
cargo run -p rytscan-cli -- scan ./my-contract --format sarif > rytscan.sarif

# List rules
cargo run -p rytscan-cli -- rules
```

Exit codes: `0` = no findings at/above threshold, `1` = findings found,
`2` = invalid path or runtime error.

## Built-in Rules (Phase 1.5)

| Rule ID | Severity | Detects |
|---|---|---|
| `AUTH-001` | High | State-changing functions without `require_auth()` |
| `PANIC-001` | Medium | `unwrap()`, `expect()`, `panic!()` in contract code |
| `TOKEN-001` | High | Unchecked SEP-41 `transfer()` return values |
| `EVENT-001` | Low | State changes without Soroban event emission |
| `TTL-001` | Medium | Persistent storage writes without `extend_ttl()` |
| `STORE-001` | High | Durable state stored in temporary storage |
| `ARITH-001` | High | `unchecked_*` arithmetic that can overflow |
| `ASSERT-001` | Medium | `assert!` macros that abort the transaction |
| `UNSAFE-001` | High | `unsafe` blocks in on-chain code |

Full reference with examples: [docs/rules.md](docs/rules.md).

## Project Structure

```
RytScan/
├── crates/
│   ├── rytscan-core/     # Rule engine + scanner + SARIF serializer
│   └── rytscan-cli/      # rytscan binary
├── fixtures/             # Vulnerable + clean sample contracts
├── docs/
│   ├── prd.md
│   ├── architecture.md
│   ├── rules.md
│   └── ROADMAP.md
└── .github/workflows/    # CI (fmt, clippy, tests, smoke)
```

## Roadmap

| Phase | Focus | Status |
|---|---|---|
| **1** | CLI, core rules, fixtures, PRD/architecture | ✅ Complete |
| **1.5** | ARITH/ASSERT/UNSAFE rules, SARIF, CI workflow | ✅ Complete |
| **2** | syn AST parser, suppressions, GitHub Action | Planned |
| **3** | WASM bytecode checks, testnet invoke probes | Planned |
| **4** | Web dashboard + Wave issue triage integrator | Planned |

See [docs/ROADMAP.md](docs/ROADMAP.md) and [docs/prd.md](docs/prd.md).

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contributor guide and
[SECURITY.md](SECURITY.md) for reporting vulnerabilities.

## Contributing (Stellar Wave)

1. Browse [Drips Stellar Wave issues](https://www.drips.network/wave/stellar/issues)
2. Filter for security / Soroban / CI labels
3. Apply via Drips during the active Wave window
4. Open PRs against `BreachDirect/RytScan`

## License

MIT

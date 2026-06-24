# RytScan

**Soroban smart contract security scanner for the Stellar ecosystem.**

RytScan is a lightweight static analysis CLI that helps Wave contributors and Soroban developers catch common security issues before testnet deployment — missing authorization, panic-prone error handling, unchecked token transfers, missing events, and unsafe storage patterns.

Built for [Stellar Wave 6](https://www.drips.network/wave/stellar) (Jun 23–30, 2026).

## Why RytScan?

| Wave need | RytScan coverage |
|---|---|
| Soroban contract security before merge | 6 built-in rules (AUTH, PANIC, TOKEN, EVENT, TTL, STORE) |
| Event emission for indexers | EVENT-001 flags state changes without `env.events().publish` |
| CI gate for contract PRs | `--fail-on high` exit code for pipelines |
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

# List rules
cargo run -p rytscan-cli -- rules
```

## Built-in Rules (Phase 1)

| Rule ID | Severity | Detects |
|---|---|---|
| `AUTH-001` | High | State-changing functions without `require_auth()` |
| `PANIC-001` | Medium | `unwrap()`, `expect()`, `panic!()` in contract code |
| `TOKEN-001` | High | Unchecked SEP-41 `transfer()` return values |
| `EVENT-001` | Low | State changes without Soroban event emission |
| `TTL-001` | Medium | Persistent storage writes without `extend_ttl()` |
| `STORE-001` | High | Durable state stored in temporary storage |

## Project Structure

```
RytScan/
├── crates/
│   ├── rytscan-core/     # Rule engine + scanner
│   └── rytscan-cli/      # rytscan binary
├── fixtures/             # Vulnerable + clean sample contracts
├── docs/
│   ├── prd.md
│   └── architecture.md
└── .github/workflows/    # Phase 2
```

## Roadmap

| Phase | Focus | Status |
|---|---|---|
| **1** | CLI, core rules, fixtures, PRD/architecture | ✅ Complete |
| **2** | syn AST parser, SARIF output, GitHub Action | Planned |
| **3** | WASM bytecode checks, testnet invoke probes | Planned |
| **4** | Web dashboard + Wave issue triage integrator | Planned |

See [docs/WAVE6_ROADMAP.md](docs/WAVE6_ROADMAP.md) and [docs/prd.md](docs/prd.md).

## Contributing (Stellar Wave)

1. Browse [Drips Stellar Wave issues](https://www.drips.network/wave/stellar/issues)
2. Filter for security / Soroban / CI labels
3. Apply via Drips during the active Wave window
4. Open PRs against `BreachDirect/RytScan`

## License

MIT

# RytScan

**A fast, zero-config security scanner for Soroban smart contracts on Stellar.**

[![CI](https://github.com/BreachDirect/RytScan/actions/workflows/ci.yml/badge.svg)](https://github.com/BreachDirect/RytScan/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.95+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](#license)
[![Rules](https://img.shields.io/badge/rules-9-green)](#built-in-rules)
[![SARIF](https://img.shields.io/badge/SARIF-2.1.0-yellow)](https://sarifweb.azurewebsites.net/)
[![Code Scanning](https://github.com/BreachDirect/RytScan/actions/workflows/ci.yml/badge.svg?branch=main&job=smoke)](https://github.com/BreachDirect/RytScan/security/code-scanning)

RytScan is a lightweight static analysis CLI that catches common security
issues in Soroban contracts **before** they reach testnet — missing
authorization, panic-prone error handling, unchecked token transfers, missing
events, unsafe arithmetic, and unsafe storage patterns. It runs locally, never
sends your source code anywhere, and drops straight into CI as a merge gate or a
GitHub Code Scanning step.

Built for the [Stellar Wave 8](https://www.drips.network/wave/stellar) program
(August 2026).

---

## Why RytScan?

| Wave need | RytScan coverage |
|---|---|
| Soroban contract security before merge | 9 built-in rules (AUTH, PANIC, TOKEN, EVENT, TTL, STORE, ARITH, ASSERT, UNSAFE) |
| Event emission for indexers | `EVENT-001` flags state changes without `env.events().publish` |
| CI gate for contract PRs | `--fail-on high` exit code + SARIF for GitHub Code Scanning |
| Contributor onboarding | Fixture contracts + `rytscan rules` catalog |

Complements ecosystem tools like
[OpenZeppelin's Soroban security detectors](https://github.com/OpenZeppelin/soroban-security-detectors-sdk)
and [Sanctifier](https://github.com/HyperSafeD/Sanctifier) with a Wave-focused,
fast, zero-config entry point.

---

## Install

### From source (recommended)

```bash
git clone https://github.com/BreachDirect/RytScan.git
cd RytScan
cargo install --path crates/rytscan-cli --locked

rytscan --version
```

### Build the binary directly

```bash
cargo build --release
./target/release/rytscan --version
```

> MSRV: **Rust 1.95+** (see `rust-version` in the workspace `Cargo.toml`).

---

## Quick Start

```bash
# Scan a contract directory (text output, fail on high-severity findings)
rytscan scan ./my-contract

# JSON output for CI pipelines
rytscan scan ./my-contract --format json --fail-on high

# SARIF v2.1.0 output for GitHub Code Scanning
rytscan scan ./my-contract --format sarif > rytscan.sarif

# Scan a single file
rytscan scan path/to/contract/src/lib.rs

# Only run specific rules (repeatable)
rytscan scan ./my-contract --rule AUTH-001 --rule ARITH-001

# List all built-in rules
rytscan rules
```

### Example output

```
$ rytscan scan fixtures/vulnerable-vault/src
RytScan v0.1.0
Target: fixtures/vulnerable-vault/src
Scanned 1 file(s) with 9 rule(s) — 19 finding(s)

[HIGH] lib.rs:60 — Missing require_auth on privileged function (AUTH-001)
  Function `withdraw` modifies state but never calls require_auth()
  > pub fn withdraw(env: Env, user: Address, amount: i128) {
  fix: Call env.storage().instance().get(&DataKey::Admin) or the relevant
       signer address, then invoke require_auth() before mutating vault state.

[HIGH] lib.rs:60 — Unchecked SEP-41 token transfer (TOKEN-001)
  Token transfer return value is not checked
  >     token.transfer(&user, amount);
  fix: Check the bool returned by token.transfer() or use a helper that maps
       failure to ContractError.

...
```

### Exit codes

| Code | Meaning |
|---|---|
| `0` | Scan complete, no findings at/above the `--fail-on` threshold |
| `1` | Findings found at or above the threshold |
| `2` | Invalid scan path or runtime error |

---

## Scan options

| Flag | Default | Description |
|---|---|---|
| `scan <path>` | — | Contract file or directory to analyze |
| `--format <text\|json\|sarif>` | `text` | Output format; `sarif` is SARIF v2.1.0 |
| `--rule <ID>` | all rules | Restrict to specific rule IDs (repeatable, e.g. `--rule AUTH-001`) |
| `--include-tests` | off | Also scan Rust files under `tests/`/`test` dirs |
| `--fail-on <severity>` | `high` | Exit `1` when a finding is at/above this severity (`info`\|`low`\|`medium`\|`high`\|`critical`) |
| `--version` | — | Print the RytScan version |
| `rules` | — | List built-in rule IDs and titles |

Unknown rule IDs are rejected with a pointer to `rytscan rules`.

---

## Built-in Rules

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

Each rule documents its CWE class, detection strategy, and a fix example in
[`docs/rules.md`](docs/rules.md).

---

## CI integration

### As a merge gate

```yaml
# .github/workflows/rytscan.yml
name: RytScan

on:
  pull_request:

jobs:
  scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install RytScan
        run: cargo install --path crates/rytscan-cli --locked
      - name: Scan contracts
        run: rytscan scan contracts/ --fail-on high
```

### As a GitHub Code Scanning step

RytScan emits [SARIF 2.1.0](https://sarifweb.azurewebsites.net/) compatible
with GitHub's `codeql-action/upload-sarif`, so findings appear in the **Security
tab** of your repo:

```yaml
      - name: Generate SARIF
        run: rytscan scan contracts/ --format sarif --fail-on critical > rytscan.sarif
      - name: Upload to Code Scanning
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: rytscan.sarif
          category: rytscan
```

> Set `--fail-on` to your preferred blocking severity. For Code Scanning
> uploads we recommend `critical` so the scan step still exits `0` for
> actionable `high` findings that the SARIF upload reports in the Security tab.

---

## How it works

```
┌────────────┐   ┌───────────────────────────┐   ┌───────────────┐
│ rytscan-cli │──▶│ rytscan-core  (rule engine)│──▶│ Report model   │
│ (clap CLI)  │   │ 9 detectors · file walker  │   │ text / JSON /  │
└────────────┘   └─────────────┬─────────────┘   │ SARIF 2.1.0    │
                               │ walk *.rs files  └───────┬───────┘
                               ▼                          ▼
                        RuleContext                 GitHub Code Scanning
                        (file, source, lines)
```

Each rule is a small, independently testable detector implementing the `Rule`
trait. Phase 1 uses function-block extraction (brace counting) plus line
heuristics; Phase 2 replaces this with a `syn` AST visitor for fewer false
positives. See [`docs/architecture.md`](docs/architecture.md) for details.

---

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
└── .github/workflows/    # CI (fmt, clippy, audit, tests, smoke)
```

---

## Limitations

RytScan is a **static** analyzer. It cannot detect runtime-only bugs, economic
exploits, or logic errors that require execution. Phase 1 heuristics may produce
false positives on complex macros; suppression support arrives in Phase 2. For
on-chain verification, see the [Phase 3 roadmap](docs/ROADMAP.md).

---

## Roadmap

| Phase | Focus | Status |
|---|---|---|
| **1** | CLI, core rules, fixtures, PRD/architecture | ✅ Complete |
| **1.5** | ARITH/ASSERT/UNSAFE rules, SARIF, CI workflow | ✅ Complete |
| **2** | `syn` AST parser, suppressions, GitHub Action | Planned |
| **3** | WASM bytecode checks, testnet invoke probes | Planned |
| **4** | Web dashboard + Wave issue triage integrator | Planned |

See [`docs/ROADMAP.md`](docs/ROADMAP.md), [`docs/prd.md`](docs/prd.md), and the
[issue tracker](https://github.com/BreachDirect/RytScan/issues).

---

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo audit            # dependency vulnerability scan
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contributor guide,
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community standards, and
[SECURITY.md](SECURITY.md) for reporting vulnerabilities.

---

## License

MIT — see [LICENSE](LICENSE). Copyright (c) 2026 BreachDirect.

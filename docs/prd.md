# Product Requirements Document (PRD): RytScan

**Version:** 1.0  
**Last updated:** 2026-06-24  
**Wave program:** [Stellar Wave 6](https://www.drips.network/wave/stellar) — Jun 23–30, 2026

---

## 1. Overview

| Field | Value |
|---|---|
| **Project** | RytScan |
| **Tagline** | Soroban security scanner for Stellar smart contracts |
| **Repository** | [BreachDirect/RytScan](https://github.com/BreachDirect/RytScan) |
| **Category** | Security tooling · Static analysis · Soroban |

## 2. Problem Statement

Soroban contracts on Stellar handle real assets. Common vulnerability classes — missing authorization, panic aborts, unchecked token transfers, missing events, TTL archival bugs — recur across Wave repos but are often caught only at review time or after testnet incidents.

Existing tools ([OpenZeppelin soroban-scanner](https://github.com/OpenZeppelin/soroban-security-detectors-sdk), [Sanctifier](https://github.com/HyperSafeD/Sanctifier)) are powerful but heavyweight. Wave contributors need a **fast, zero-config scanner** they can run locally and in CI during the 7-day sprint.

## 3. Drips Wave Alignment

RytScan maps to recurring patterns in the [Stellar Wave issue catalog](https://www.drips.network/wave/stellar/issues):

| Wave pattern | RytScan response |
|---|---|
| Soroban event emission for indexers | EVENT-001 rule ([Lumenpulse #269](https://github.com/Pulsefy/Lumenpulse/issues/269)) |
| Contract security before testnet deploy | AUTH-001, TOKEN-001, PANIC-001 |
| CI security gates on contract PRs | `--fail-on high` + JSON reports |
| Backend/indexer reliability | Events flagged early reduce silent state changes |

**Wave 6 goal:** Ship Phase 1 CLI so contributors can scan contracts on day one of the sprint.

## 4. Solution

RytScan provides:

1. **`rytscan scan <path>`** — walk Soroban Rust sources and run security rules
2. **Rule catalog** — 6 Phase 1 detectors aligned with [stellar-dev-skill](https://github.com/stellar/stellar-dev-skill) vulnerability classes
3. **Fixture contracts** — vulnerable + clean samples for regression tests
4. **CI-ready output** — text + JSON, configurable failure threshold

## 5. Target Users

- Soroban developers submitting Wave PRs
- Repo maintainers triaging `Stellar Wave` security issues
- Auditors doing first-pass static review before deep analysis

## 6. Phased Delivery

### Phase 1: Core CLI & Rule Engine ✅

| Deliverable | Status |
|---|---|
| `rytscan-core` rule engine | ✅ |
| `rytscan-cli` binary (`scan`, `rules`) | ✅ |
| 6 built-in security rules | ✅ |
| Vulnerable + clean fixture contracts | ✅ |
| PRD + architecture documentation | ✅ |
| JSON + text report formats | ✅ |

**Success criteria:**

- [x] `cargo test` passes
- [x] Scanning `fixtures/vulnerable-vault` produces ≥ 4 findings
- [x] Scanning `fixtures/clean-token` produces 0 high/critical findings
- [x] `rytscan rules` lists all rule IDs
- [x] Documented Wave 6 alignment

### Phase 2: AST Analysis & CI Integration

- Replace line-based heuristics with `syn` AST traversal
- SARIF v2.1.0 output for GitHub Code Scanning
- GitHub Action: `BreachDirect/rytscan-action`
- Rule suppressions via `rytscan.toml` config

### Phase 3: On-Chain Verification

- WASM size / export surface checks post-build
- Testnet smoke probes via `stellar contract invoke`
- Cross-reference static findings with simulation traces

### Phase 4: Platform & Wave Integrator

- Web dashboard for scan history and severity trends
- Drips Wave issue matcher (suggest rules from issue title/body)
- VS Code diagnostics extension

## 7. Non-Goals (Phase 1)

- Formal verification (Z3)
- Runtime on-chain guards
- Full duplicate of OpenZeppelin detector SDK
- Web UI

## 8. Success Metrics

| Metric | Phase 1 | Phase 4 |
|---|---|---|
| Built-in rules | 6 | 20+ |
| False positive rate (fixtures) | ≤ 1 per clean fixture | ≤ 5% |
| Scan time (1 contract) | < 100ms | < 50ms |
| Wave repos adopting CI gate | 0 | 10+ |

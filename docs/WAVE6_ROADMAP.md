# RytScan — Stellar Wave 6 Roadmap

**Program:** [Stellar Wave 6](https://www.drips.network/wave/stellar)  
**Window:** Jun 23 – Jun 30, 2026  
**Repo:** [ToryMic/RytScan](https://github.com/ToryMic/RytScan)

## Phase 1 ✅ Complete

- Rust workspace (`rytscan-core`, `rytscan-cli`)
- 6 security rules (AUTH, PANIC, TOKEN, EVENT, TTL, STORE)
- Fixture contracts + unit tests
- PRD, architecture, README

```bash
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src
cargo run -p rytscan-cli -- rules
```

## Phase 2 — AST & CI (GitHub Issues)

- syn-based AST rule engine
- SARIF output for GitHub Code Scanning
- `rytscan.toml` suppressions
- GitHub Action workflow

## Phase 3 — On-Chain Verification

- Post-build WASM checks (size, exports)
- `stellar contract invoke` smoke probes
- Simulation trace correlation

## Phase 4 — Platform

- Scan history web dashboard
- Drips Wave issue → rule matcher
- VS Code extension

## Contributing

Apply for Wave issues at [Drips](https://www.drips.network/wave/stellar/issues). KYC required upfront since Wave 5.

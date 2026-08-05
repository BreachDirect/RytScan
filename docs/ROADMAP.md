# RytScan — Stellar Wave 8 Roadmap

**Program:** [Stellar Wave 8](https://www.drips.network/wave/stellar)  
**Window:** August 2026  
**Repo:** [BreachDirect/RytScan](https://github.com/BreachDirect/RytScan)

## Phase 1 — Core CLI ✅ Complete

- Rust workspace (`rytscan-core`, `rytscan-cli`)
- 6 security rules (AUTH, PANIC, TOKEN, EVENT, TTL, STORE)
- Fixture contracts + unit tests
- PRD, architecture, README

## Phase 1.5 — Rule Expansion & CI ✅ Complete

- 3 new rules: ARITH-001 (unchecked arithmetic), ASSERT-001 (assert macros),
  UNSAFE-001 (unsafe blocks) → 9 total
- SARIF v2.1.0 output (`--format sarif`) for GitHub Code Scanning
- CI workflow: rustfmt, clippy (`-D warnings`), unit tests, fixture smoke
- `rustfmt.toml` workspace formatting config

```bash
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src --format sarif
cargo run -p rytscan-cli -- rules
```

## Phase 2 — AST & CI (GitHub Issues)

- [ ] syn-based AST rule engine
- [ ] `rytscan.toml` suppressions
- [ ] GitHub Action: `BreachDirect/rytscan-action`

## Phase 3 — On-Chain Verification

- [ ] Post-build WASM checks (size, exports)
- [ ] `stellar contract invoke` smoke probes
- [ ] Simulation trace correlation

## Phase 4 — Platform

- [ ] Scan history web dashboard
- [ ] Drips Wave issue → rule matcher
- [ ] VS Code extension

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) and the
[issue tracker](https://github.com/BreachDirect/RytScan/issues). Wave issue
bounties are listed at [Drips](https://www.drips.network/wave/stellar/issues).

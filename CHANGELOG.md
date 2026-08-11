# Changelog

All notable changes to RytScan are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repo hygiene: `CODE_OF_CONDUCT.md`, `CHANGELOG.md`, Dependabot config,
  `CODEOWNERS`, and a release workflow for tagged builds.
- README: install-from-source instructions, CI integration examples for
  merge-gates and GitHub Code Scanning, example output, and a limitations section.

## [0.1.0] - 2026-08-09

Phase 1 + 1.5 of the Soroban scanner for Stellar Wave 8.

### Added

- Rust workspace: `rytscan-core` (rule engine + scanner + SARIF serializer)
  and `rytscan-cli` (Clap CLI).
- 9 built-in rules: AUTH-001, PANIC-001, TOKEN-001, EVENT-001, TTL-001,
  STORE-001, ARITH-001, ASSERT-001, UNSAFE-001.
- Text, JSON, and SARIF v2.1.0 output formats.
- `--fail-on <severity>` exit-code gating for CI.
- `rytscan rules` catalog command.
- Vulnerable + clean fixture contracts.
- CI: rustfmt, clippy (`-D warnings`), `cargo audit`, workspace tests, and a
  SARIF smoke test that uploads to GitHub Code Scanning.

[Unreleased]: https://github.com/BreachDirect/RytScan/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/BreachDirect/RytScan/releases/tag/v0.1.0

# Architecture: RytScan

**Version:** 1.1  
**Last updated:** 2026-08-05

## 1. System Overview

RytScan is a Rust workspace with a library crate (rule engine) and a CLI binary.

```mermaid
graph LR
    CLI[rytscan-cli] --> CORE[rytscan-core]
    CORE --> RULES[Rule Engine]
    CORE --> SCAN[Directory Scanner]
    RULES --> OUT[Report Text/JSON]
    CORE --> SARIF[SARIF 2.1.0 Serializer]
    SARIF --> GCS[GitHub Code Scanning]
    SCAN --> FS[(.rs Soroban sources)]
```

## 2. Crate Layout

| Crate | Role |
|---|---|
| `rytscan-core` | Rule trait, 9 detectors, report model, file walker, SARIF serializer |
| `rytscan-cli` | Clap CLI, output formatting, exit codes |

## 3. Scan Pipeline

```mermaid
sequenceDiagram
    participant User
    participant CLI as rytscan-cli
    participant Scanner
    participant Rules
    participant Report

    User->>CLI: rytscan scan ./contracts
    CLI->>Scanner: scan_path(target)
    Scanner->>Scanner: collect *.rs files
    loop each file
        Scanner->>Rules: run(ctx)
        Rules-->>Scanner: Vec<Finding>
    end
    Scanner->>Report: aggregate + sort
    Report-->>CLI: Report
    CLI-->>User: text or JSON
```

### 3.1 File Discovery

- Recursively walks target directory via `walkdir`
- Includes `*.rs` files; excludes `/tests/` and `*_test.rs` by default
- `--include-tests` overrides exclusion

### 3.2 Rule Context

Each rule receives:

```rust
RuleContext {
    file: &str,      // relative path
    source: &str,    // full file contents
    lines: &[String] // line-indexed for snippets
}
```

Phase 1 uses function-block extraction (brace counting) and line heuristics. Phase 2 replaces this with `syn` AST visitors.

## 4. Rule Catalog

| ID | Engine | Severity | Detection strategy |
|---|---|---|---|
| AUTH-001 | Function analysis | High | State-changing `pub fn` without `require_auth` |
| PANIC-001 | Line scan | Medium | `unwrap`, `expect`, `panic!` |
| TOKEN-001 | Line scan | High | `.transfer(` without result check |
| EVENT-001 | Function analysis | Low | State change without `env.events()` |
| TTL-001 | Function analysis | Medium | `.persistent().set` without `extend_ttl` |
| STORE-001 | Line scan | High | `.temporary().set` with durable keys |
| ARITH-001 | Line scan | High | `unchecked_add`/`unchecked_sub`/`unchecked_mul` (overflow risk) |
| ASSERT-001 | Line scan | Medium | `assert!`/`assert_eq!` macros (abort on failure) |
| UNSAFE-001 | Line scan | High | `unsafe { }` blocks (memory safety) |

Full rule descriptions: [docs/rules.md](rules.md).

## 5. Report Model

```json
{
  "tool": "RytScan",
  "version": "0.1.0",
  "target": "fixtures/vulnerable-vault/src",
  "summary": {
    "files_scanned": 1,
    "rules_run": 9,
    "findings": 19,
    "by_severity": { "high": 10, "medium": 3, "low": 6 }
  },
  "findings": [ ... ]
}
```

`--format sarif` produces a SARIF 2.1.0 log with one rule entry per detector
(`helpUri` → [docs/rules.md](rules.md#rule-id)) and per-finding `level` mapped
from severity (high → `error`, medium → `warning`, low → `note`).

Exit codes:

| Code | Meaning |
|---|---|
| 0 | Scan complete, no findings ≥ `--fail-on` threshold |
| 1 | Findings at or above threshold |
| 2 | Invalid path / runtime error |

## 6. Fixtures

```
fixtures/
├── vulnerable-vault/src/lib.rs   # intentional issues for regression
└── clean-token/src/lib.rs        # passes high-severity checks
```

Used by `cargo test` in `rytscan-core` and documented in README quick start.

## 7. Phase 2 Architecture (Planned)

```mermaid
graph TB
    CLI --> CORE
    CORE --> SYN[syn AST Visitor]
    GHA[GitHub Action] --> CLI
    CORE --> CONFIG[rytscan.toml suppressions]
```

## 8. Security & Limitations

- **Static only:** Cannot detect runtime-only bugs or economic exploits
- **Heuristic:** Phase 1 may false-positive on complex macros; suppressions come in Phase 2
- **No network:** Scanner never sends source code off-machine
- **Fail closed in CI:** Default `--fail-on high` blocks merges on auth/token issues

## 9. Wave Integration Points

| Phase | Drips contributor workflow |
|---|---|
| 1/1.5 ✅ | `rytscan scan .` + `--format sarif` before opening Wave PR |
| 2 | GitHub Action comment on PR with findings |
| 3 | Testnet deploy checklist includes RytScan + invoke probe |
| 4 | Match scan gaps to open Wave security issues |

Browse: [Drips Stellar Wave Issues](https://www.drips.network/wave/stellar/issues)

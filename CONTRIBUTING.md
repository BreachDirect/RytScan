# Contributing to RytScan

Thank you for your interest in RytScan! We're building a fast, zero-config
security scanner for Soroban contracts on Stellar and welcome contributors of
**all skill levels** — from first-time open-source contributors to seasoned
Rust engineers.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style & Standards](#code-style--standards)
- [Adding a New Rule](#adding-a-new-rule)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Commit Message Conventions](#commit-message-conventions)
- [Issue Reporting](#issue-reporting)
- [Good First Contributions](#good-first-contributions)

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating you agree to uphold a welcoming, respectful environment for everyone.

---

## Getting Started

### Prerequisites

| Tool | Version | Notes |
| --- | --- | --- |
| Rust | 1.95+ | Install via [rustup](https://rustup.rs) |
| Cargo | 1.95+ | Ships with rustup |

### 1. Fork & Clone

```bash
git clone https://github.com/<your-username>/RytScan.git
cd RytScan
```

### 2. Build & Test

```bash
cargo build --workspace
cargo test --workspace
```

### 3. Run the Scanner

```bash
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src
cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src --format sarif
cargo run -p rytscan-cli -- rules
```

---

## Development Workflow

### Branching Strategy

| Branch | Purpose |
| --- | --- |
| `main` | Stable, always passing CI |
| `feature/<topic>` | New features or enhancements |
| `fix/<topic>` | Bug fixes |
| `docs/<topic>` | Documentation-only changes |
| `chore/<topic>` | Tooling, CI, dependency updates |

**Always branch off `main`:**

```bash
git checkout main
git pull origin main
git checkout -b feature/my-feature
```

Prefer rebasing over merging to keep a clean history:

```bash
git fetch origin
git rebase origin/main
```

---

## Code Style & Standards

CI enforces formatting, linting, and tests. Run these before every commit:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Additional guidelines:

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for public APIs.
- Document public items with `///` doc comments.
- Avoid `unwrap()` in production paths — use `?` or explicit error handling.
- No `unsafe` code in the scanner itself (our UNSAFE-001 rule must stay clean).

---

## Adding a New Rule

1. Add a `struct <Name>Rule` in `crates/rytscan-core/src/rules.rs` implementing
   the `Rule` trait (`id`, `title`, `run`).
2. Register it in `all_rules()`.
3. Add a row to `docs/rules.md` with severity, CWE class, detection strategy,
   and a fix example.
4. Add a unit test (happy path + the finding) in the same file, mirroring the
   existing `ARITH-001` / `ASSERT-001` / `UNSAFE-001` tests.
5. Run `cargo test -p rytscan-core` and confirm the fixture smoke still passes:
   `cargo run -p rytscan-cli -- scan fixtures/vulnerable-vault/src --format sarif`.
6. If the fixture should trigger your rule, extend
   `fixtures/vulnerable-vault/src/lib.rs` accordingly.

---

## Testing Requirements

Every code contribution **must** include appropriate tests.

| Change type | Required tests |
| --- | --- |
| New rule | Unit test for the finding + negative test on clean code |
| Bug fix | Regression test that would have caught the bug |
| New utility / helper | Unit tests covering happy path and edge cases |
| Refactor | Existing tests must continue to pass |

### Running Tests

```bash
cargo test --workspace          # all crates
cargo test -p rytscan-core      # rule engine only
cargo test -p rytscan-cli       # CLI only
```

---

## Pull Request Process

### Before Opening a PR

- [ ] Your branch is rebased on the latest `main`
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] New or updated tests are included
- [ ] No `todo!()` / `unimplemented!()` / debug `println!` left in production paths

### Opening the PR

1. Push your branch to your fork.
2. Open a PR against `BreachDirect/RytScan:main`.
3. Fill in the PR template (summary, motivation / linked issue, testing performed).
4. Request a review — maintainers aim to respond within **48 hours**.

### Review Checklist (for reviewers)

- [ ] Code is correct and handles errors gracefully
- [ ] Tests are meaningful and cover edge cases
- [ ] Public APIs are documented
- [ ] No unnecessary complexity introduced
- [ ] CI is green

---

## Commit Message Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer: "Closes #<issue>"]
```

### Types

| Type | When to use |
| --- | --- |
| `feat` | New feature or rule |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Code restructuring, no behaviour change |
| `test` | Adding or fixing tests |
| `chore` | Tooling, CI, dependency bumps |
| `perf` | Performance improvement |

### Scope (optional)

Use the area affected: `core`, `cli`, `sarif`, `rules`, `fixtures`, `docs`, `ci`.

### Example

```
feat(rules): add ARITH-001 unchecked arithmetic detector

Fires on unchecked_add/sub/mul/div and friends, which wrap silently
on overflow. Maps to CWE-190.

Closes #12
```

---

## Issue Reporting

### Bug Reports

Please include:

1. **Rust version**: `rustc --version`
2. **Steps to reproduce** (minimal source snippet)
3. **Expected vs actual behaviour** (including exit code)
4. **Relevant output** (text, JSON, or SARIF)

Use the **Bug Report** issue template on GitHub.

### Feature Requests

- Describe the problem you're solving, not just the solution.
- Link to any related issues or discussions.
- Check [docs/ROADMAP.md](docs/ROADMAP.md) first — your feature may already be planned.

### Security Vulnerabilities

**Do not open a public issue.** Contact the maintainers privately via GitHub's
[Security Advisories](../../security/advisories) feature — see
[SECURITY.md](SECURITY.md).

---

## Good First Contributions

Not sure where to start?

- Issues tagged [`good-first-issue`](../../issues?q=label%3Agood-first-issue)
- Expand `docs/rules.md` with real-world examples
- Add tests for existing rules (clean-fixture negative cases)
- Harden the SARIF serializer against edge cases

**New to Rust?** Start with a documentation or testing issue. Maintainers are
happy to review Rust code and suggest idiomatic improvements.

**New to Soroban?** Read the
[Soroban docs](https://developers.stellar.org/docs/build/smart-contracts) and
our [docs/architecture.md](docs/architecture.md) to get a feel for the domain.

---

**Thank you for contributing to RytScan!** Every PR helps make Soroban
contracts safer for the Stellar ecosystem. 🚀

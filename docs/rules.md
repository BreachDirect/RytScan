# RytScan Rule Reference

This page documents every rule shipped by RytScan. Rule IDs are stable; each
entry lists the severity, the vulnerability class, and remediation guidance.

- **High** findings should block merges (`--fail-on high`, the default).
- **Medium** findings are likely bugs or footguns.
- **Low** findings are hygiene/observability improvements.

| Severity | SARIF level |
|---|---|
| High | `error` |
| Medium | `warning` |
| Low | `note` |

---

## ARITH-001 — Unchecked arithmetic (High)

**Vulnerability class:** CWE-190 Integer Overflow or Wraparound

Detects calls to `unchecked_add`, `unchecked_sub`, `unchecked_mul`,
`unchecked_div`, `unchecked_rem`, `unchecked_neg`, `unchecked_shl`, and
`unchecked_shr`. These wrap silently on overflow and can corrupt balances or
totals in contracts handling real assets.

**Fix:** use `checked_*` arithmetic and map `None` to a typed contract error:

```rust
let total = amount.checked_add(1).ok_or(ContractError::Overflow)?;
```

---

## ASSERT-001 — Assertion macros (Medium)

**Vulnerability class:** CWE-617 Reachable Assertion

Detects `assert!`, `assert_eq!`, `assert_ne!`, and `debug_assert!`. A failed
assertion panics and aborts the entire Soroban transaction, which can be
weaponized as a griefing vector when user-controlled input reaches an assert.

**Fix:** validate explicitly and return a typed error:

```rust
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}
```

---

## AUTH-001 — Missing `require_auth` (High)

**Vulnerability class:** CWE-862 Missing Authorization

Flags state-changing `pub fn`s that never call `require_auth`. Contracts that
mutate vault state without authenticating the caller allow anyone to drain,
reconfigure, or mint.

**Fix:** load the privileged signer and authenticate before mutating:

```rust
let admin: Address = env.storage().instance().get(&DataKey::Admin)?;
admin.require_auth();
```

---

## EVENT-001 — Missing Soroban events (Low)

**Vulnerability class:** CWE-778 Insufficient Logging

Flags state-changing functions that never call `env.events().publish(...)`.
Indexers and off-chain monitoring depend on event emission to track deposits,
withdrawals, and admin actions.

**Fix:** publish structured events for every user-visible state change:

```rust
env.events().publish((AdminOnly, "deposited"), (user, amount));
```

---

## PANIC-001 — Panic-prone error handling (Medium)

**Vulnerability class:** CWE-754 Improper Check for Unusual Conditions

Detects `.unwrap()`, `.expect(...)`, and `panic!(...)`. Panics abort the whole
transaction and make failure modes hard to distinguish from hostile input.

**Fix:** propagate errors with `Result<T, ContractError>` instead of panicking.

---

## STORE-001 — Temporary storage for durable state (High)

**Vulnerability class:** CWE-911 Improper Update of Reference Count

Flags `.temporary().set(...)` used with durable protocol keys (Admin, Owner,
Balance, TotalSupply, Vault, Shares). Temporary entries are ledger-scoped and
can be evicted, losing protocol state.

**Fix:** use instance or persistent storage for balances, totals, and admin
state.

---

## TOKEN-001 — Unchecked token transfer (High)

**Vulnerability class:** CWE-252 Unchecked Return Value

Flags `.transfer(` calls whose boolean result is never checked. SEP-41
transfers return a result; ignoring it can silently leave users uncredited
while the caller believes the transfer succeeded.

**Fix:** check the returned `bool` and map failure to a contract error:

```rust
token.transfer(&env, &to, &amount)?;
```

---

## TTL-001 — Persistent write without TTL extension (Medium)

**Vulnerability class:** CWE-345 Improper Verification of Data Authenticity

Flags functions that write `.persistent().set(...)` without `extend_ttl` /
`extend_instance_ttl`. Soroban entries expire; long-lived contract data must be
refreshed to avoid unexpected archival.

**Fix:** extend the entry TTL after writes:

```rust
env.storage().persistent().extend_ttl(&key, live_until, live_until);
```

---

## UNSAFE-001 — `unsafe` blocks (High)

**Vulnerability class:** CWE-119 Improper Restriction of Operations within the
Bounds of a Memory Buffer

Detects `unsafe {` blocks and `unsafe fn`. Undefined behavior in on-chain code
can brick a contract holding user funds.

**Fix:** remove `unsafe` from contract code entirely. If unavoidable, isolate it
behind a reviewed, documented safety boundary with unit tests.

---

*Generated from the [RytScan rule engine](../crates/rytscan-core/src/rules.rs).
Scan with `rytscan scan <path>`; SARIF output via `--format sarif`.*

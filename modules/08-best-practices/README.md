# Module 08 — Best Practices

> Production-quality patterns for Soroban smart contracts.

## 1. Project Structure

```
my-project/
├── Cargo.toml               # Workspace root
├── contracts/
│   ├── my-contract/
│   │   ├── Cargo.toml        # crate-type = ["cdylib"]
│   │   └── src/
│   │       ├── lib.rs        # Contract entry point
│   │       ├── storage.rs    # Storage helpers
│   │       ├── types.rs      # Custom types
│   │       ├── errors.rs     # Error enum
│   │       └── test.rs       # Tests
│   └── another-contract/
│       └── ...
└── target/
```

### Root Cargo.toml (always use workspace)
```toml
[workspace]
resolver = "2"
members = ["contracts/*"]

[workspace.dependencies]
soroban-sdk = "22"

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

### Contract Cargo.toml
```toml
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

## 2. Storage Best Practices

### Choose the Right Storage Type

| Data | Storage Type | Why |
|------|-------------|-----|
| Admin address | Instance | Small, always needed, shares TTL with contract |
| Config/settings | Instance | Same as admin |
| User balances | Persistent | Must survive long term, per-user |
| Token metadata | Persistent | Rarely changes, must be permanent |
| Approval/allowance | Temporary | Has natural expiration |
| Session/nonce | Temporary | Cheapest, OK to lose |

### Always Extend TTL

```rust
// After every write or important read, extend TTL
const DAY: u32 = 17280;

// Instance: extend the whole contract
env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);

// Persistent: extend specific keys
env.storage().persistent().extend_ttl(&key, 29 * DAY, 30 * DAY);

// Pattern: threshold = extend_to - 1 day (avoids unnecessary extensions)
```

### Use Enum Keys (not raw strings)

```rust
// GOOD — type-safe, refactorable
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Config,
}
env.storage().instance().set(&DataKey::Admin, &admin);

// BAD — easy to typo, no compiler help
env.storage().instance().set(&symbol_short!("admin"), &admin);
```

### Never Rely on TTL Expiry for Logic

```rust
// BAD — anyone can extend TTL, so this lock is bypassable
env.storage().temporary().set(&DataKey::Lock, &true);
// Hoping it expires to "unlock"...

// GOOD — use explicit timestamp logic
pub fn unlock(env: Env, caller: Address) {
    let lock_until: u64 = env.storage().persistent().get(&DataKey::LockUntil).unwrap();
    if env.ledger().timestamp() < lock_until {
        panic!("Still locked");
    }
    // ... unlock logic
}
```

## 3. Authorization Best Practices

### Always Auth State-Changing Functions

```rust
// GOOD
pub fn withdraw(env: Env, user: Address, amount: i128) {
    user.require_auth();
    // ... withdraw logic
}

// BAD — anyone can withdraw anyone's funds!
pub fn withdraw(env: Env, user: Address, amount: i128) {
    // Missing require_auth!
    // ... withdraw logic
}
```

### Store Admin, Don't Hardcode

```rust
// GOOD — admin can be transferred
pub fn set_admin(env: Env, new_admin: Address) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
    env.storage().instance().set(&DataKey::Admin, &new_admin);
}
```

## 4. Error Handling Best Practices

### Use Custom Errors (not raw panics)

```rust
// GOOD — clear error codes, clients can handle them
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    NotAuthorized = 2,
    InsufficientBalance = 3,
    InvalidAmount = 4,
    AlreadyExists = 5,
}

pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
    if amount <= 0 { return Err(Error::InvalidAmount); }
    // ...
}

// BAD — opaque panic, hard to debug
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    if amount <= 0 { panic!("bad"); }
}
```

## 5. WASM Size Management

Keep your compiled WASM under **64KB**:

| Technique | Impact | How |
|-----------|--------|-----|
| `opt-level = "z"` | ~30% reduction | Cargo.toml profile |
| `lto = true` | ~20% reduction | Link-time optimization |
| `strip = "symbols"` | ~10% reduction | Remove debug symbols |
| `panic = "abort"` | ~5% reduction | Smaller panic handler |
| `stellar contract optimize` | ~10-20% more | Post-build WASM optimizer |
| Avoid large dependencies | Varies | Don't import heavy crates |

```bash
# Check your WASM size
ls -la target/wasm32-unknown-unknown/release/*.wasm

# Optimize
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/my_contract.wasm
```

## 6. Testing Best Practices

### Test Every Code Path

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_happy_path() { /* normal operation */ }

    #[test]
    fn test_edge_cases() { /* zero amount, max values */ }

    #[test]
    #[should_panic(expected = "Not authorized")]
    fn test_unauthorized() { /* wrong user tries to call */ }

    #[test]
    fn test_error_returns() {
        // Use try_ variants for Result-returning functions
        let result = client.try_transfer(&wrong_user, &to, &amount);
        assert!(result.is_err());
    }
}
```

### Use `mock_all_auths()` in Tests

```rust
let env = Env::default();
env.mock_all_auths(); // Auto-approve all auth checks in tests
```

## 7. Security Checklist

Before deploying to mainnet:

- [ ] Every state-changing function has `require_auth()`
- [ ] Admin functions check admin from storage
- [ ] No hardcoded secret keys in code
- [ ] `.env` is in `.gitignore`
- [ ] All `.unwrap()` calls have a safe fallback or are in non-critical paths
- [ ] Integer overflow is handled (`overflow-checks = true` in Cargo.toml)
- [ ] WASM is under 64KB
- [ ] All tests pass
- [ ] TTL is extended in critical functions
- [ ] Custom errors give meaningful feedback
- [ ] Contract has an upgrade path (for future fixes)

## 8. Gas / Fee Optimization

```rust
// GOOD — batch storage reads
let (balance, config) = (
    env.storage().persistent().get(&DataKey::Balance(user.clone())),
    env.storage().instance().get(&DataKey::Config),
);

// BAD — unnecessary storage writes
pub fn read_only_check(env: Env) -> bool {
    let val = env.storage().instance().get(&key).unwrap();
    env.storage().instance().set(&key, &val); // Pointless write!
    val > 0
}
```

## 9. Symbol Key Rules

```rust
// symbol_short! — up to 9 chars, a-zA-Z0-9_
// More efficient than full Symbol
let key = symbol_short!("count");

// Full Symbol — up to 32 chars
let key = Symbol::new(&env, "longer_key_name");

// DataKey enum — unlimited complexity, type-safe
#[contracttype]
pub enum DataKey {
    Balance(Address),     // No length limit
    Allowance(Address, Address),
}
```

# SKILL: Soroban Smart Contract Development

> Use this reference when writing Soroban smart contracts. Follow these patterns exactly.

## Environment

- Language: Rust (no_std)
- Target: wasm32-unknown-unknown
- SDK: soroban-sdk v22
- Max WASM size: 64KB

## Contract Template (start every contract from this)

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Address, Env, String, Symbol, symbol_short, BytesN, Vec, Map, log,
};

// ============================================================
// CONSTANTS
// ============================================================

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const PERSISTENT_TTL: u32 = 30 * DAY_IN_LEDGERS;
const PERSISTENT_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;

// ============================================================
// DATA TYPES — Define all storage keys and custom types here
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    // Add your keys here
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAuthorized = 3,
    NotFound = 4,
    InvalidInput = 5,
    // Add your errors here
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    /// Constructor — called once at deploy time
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    // Add your functions here
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_example() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(MyContract, MyContractArgs::__constructor(&admin));
        let client = MyContractClient::new(&env, &contract_id);

        // Your test assertions here
    }
}
```

## Storage Patterns

### Instance (config, shared state — ALL share one TTL)
```rust
env.storage().instance().set(&DataKey::Admin, &admin);
env.storage().instance().get(&DataKey::Admin).unwrap();
env.storage().instance().has(&DataKey::Admin);
env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
```

### Persistent (user data — per-key TTL, recoverable on expiry)
```rust
env.storage().persistent().set(&DataKey::Balance(user.clone()), &amount);
env.storage().persistent().get(&DataKey::Balance(user)).unwrap_or(0);
env.storage().persistent().has(&DataKey::Balance(user));
env.storage().persistent().remove(&DataKey::Balance(user));
env.storage().persistent().extend_ttl(&DataKey::Balance(user), PERSISTENT_THRESHOLD, PERSISTENT_TTL);
```

### Temporary (disposable — per-key TTL, GONE FOREVER on expiry)
```rust
env.storage().temporary().set(&DataKey::Session(user.clone()), &data);
env.storage().temporary().get(&DataKey::Session(user)).unwrap_or(default);
env.storage().temporary().extend_ttl(&DataKey::Session(user), 0, DAY_IN_LEDGERS);
```

## Auth Patterns

```rust
// User must sign
user.require_auth();

// Admin-only
let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
admin.require_auth();

// Custom args scope
user.require_auth_for_args((amount,).into_val(&env));
```

## Error Handling

```rust
// Return Result (preferred — client gets typed error)
pub fn my_func(env: Env) -> Result<u32, ContractError> {
    if !condition { return Err(ContractError::InvalidInput); }
    Ok(value)
}

// Panic with error (simpler but less client-friendly)
pub fn my_func(env: Env) -> u32 {
    if !condition { panic_with_error!(&env, ContractError::InvalidInput); }
    value
}
```

## Events

```rust
// Simple event
env.events().publish((symbol_short!("transfer"),), amount);

// Multi-topic event (indexed, searchable)
env.events().publish(
    (symbol_short!("transfer"), from.clone(), to.clone()),
    amount,
);
```

## Custom Types

```rust
#[contracttype]
#[derive(Clone, Debug)]
pub struct MyStruct {
    pub field1: Address,
    pub field2: i128,
    pub field3: String,
}

#[contracttype]
pub enum MyEnum {
    VariantA,
    VariantB(u32),
    VariantC(Address),
}
```

## Common Patterns

### Auto-increment ID
```rust
let id: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0) + 1;
env.storage().instance().set(&DataKey::Counter, &id);
```

### Timestamp
```rust
let now: u64 = env.ledger().timestamp();
```

### Current contract address
```rust
let self_addr: Address = env.current_contract_address();
```

### Token interaction
```rust
use soroban_sdk::token;
let client = token::Client::new(&env, &token_address);
client.transfer(&from, &to, &amount);
client.balance(&address);
```

## Rules

1. ALWAYS start with `#![no_std]`
2. ALWAYS use `require_auth()` on state-changing functions
3. ALWAYS extend TTL after writes
4. ALWAYS use `#[contracterror]` for error types
5. ALWAYS use `#[contracttype]` for storage enums and structs
6. NEVER store secrets in contract code
7. NEVER use `std` library functions
8. Keep WASM under 64KB
9. Symbol keys: max 32 chars, `a-zA-Z0-9_` only
10. `symbol_short!()` for keys up to 9 chars (more efficient)

## Cargo.toml

### Contract crate
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

### Workspace root
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

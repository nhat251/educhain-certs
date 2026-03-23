# Official Soroban Examples — Learning Guide

> 34 official smart contract examples from the Stellar team.
> Cloned from [stellar/soroban-examples v23.0.0](https://github.com/stellar/soroban-examples/tree/v23.0.0).

## How to Use These Examples

```bash
# Pick any example
cd soroban-examples/hello_world

# Build it
stellar contract build

# Run its tests
cargo test

# Deploy to testnet
stellar keys generate student --network testnet --fund
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source-account student \
  --network testnet
```

## Examples by Learning Level

### Beginner — Start Here

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 1 | **hello_world** | Basic contract structure, `#[contract]`, `#[contractimpl]` | `src/lib.rs` |
| 2 | **increment** | Counter + instance storage + TTL extension | `src/lib.rs` |
| 3 | **custom_types** | Define `#[contracttype]` structs for storage | `src/lib.rs` |
| 4 | **logging** | Debug with `log!(&env, ...)` | `src/lib.rs` |
| 5 | **errors** | Custom `#[contracterror]` types + `Result<T, Error>` | `src/lib.rs` |
| 6 | **events** | Publish on-chain events with `#[contractevent]` | `src/lib.rs` |

**Recommended order**: hello_world → increment → custom_types → errors → events → logging

### Intermediate — Auth & Tokens

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 7 | **auth** | `require_auth()` — per-user counters | `src/lib.rs` |
| 8 | **token** | Full token implementation (mint, transfer, burn, approve) | `src/contract.rs` |
| 9 | **single_offer** | Order-book trading for a token pair | `src/lib.rs` |
| 10 | **atomic_swap** | Two-party token exchange with price guarantees | `src/lib.rs` |
| 11 | **timelock** | Lock tokens with time-based release conditions | `src/lib.rs` |
| 12 | **mint-lock** | Token minting with authorization controls | `src/lib.rs` |

### Intermediate — Architecture Patterns

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 13 | **cross_contract** | Call another contract from your contract | `contract_b/src/lib.rs` |
| 14 | **deployer** | Factory pattern — deploy contracts programmatically | `deployer/src/lib.rs` |
| 15 | **upgradeable_contract** | Upgrade a live contract's WASM code | `new_contract/src/lib.rs` |
| 16 | **workspace** | Multi-contract Cargo workspace setup | `Cargo.toml` |
| 17 | **ttl** | TTL management for all 3 storage types | `src/lib.rs` |
| 18 | **alloc** | Using Rust's allocator in no_std contracts | `src/lib.rs` |
| 19 | **other_custom_types** | Advanced custom type patterns | `src/lib.rs` |

### Advanced — DeFi & Multi-sig

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 20 | **liquidity_pool** | AMM with deposit/swap/withdraw + LP tokens | `src/lib.rs` |
| 21 | **atomic_multiswap** | Multi-party batch token swaps | `src/lib.rs` |
| 22 | **simple_account** | Custom account with single-key auth | `src/lib.rs` |
| 23 | **account** | Multi-sig account with custom policies | `src/lib.rs` |
| 24 | **multisig_1_of_n_account** | 1-of-N multi-sig (any signer can authorize) | `src/lib.rs` |
| 25 | **deep_contract_auth** | Auth flows through nested contract calls | `src/lib.rs` |

### Advanced — Cryptography & Specialized

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 26 | **bls_signature** | BLS signature verification on-chain | `src/lib.rs` |
| 27 | **groth16_verifier** | Zero-knowledge proof verification (Groth16) | `src/lib.rs` |
| 28 | **import_ark_bn254** | BN254 curve cryptography | `src/lib.rs` |
| 29 | **eth_abi** | Decode Ethereum ABI data in Soroban | `src/lib.rs` |
| 30 | **assets** | Working with Stellar assets on Soroban | `src/lib.rs` |

### Testing & Safety

| # | Example | What You Learn | Key File |
|---|---------|---------------|----------|
| 31 | **fuzzing** | Fuzz testing setup for Soroban contracts | `src/lib.rs` |
| 32 | **increment_with_fuzz** | Increment contract with fuzzing | `src/lib.rs` |
| 33 | **pause** | Contract pause/unpause (circuit breaker) | `src/lib.rs` |
| 34 | **increment_with_pause** | Increment with pause functionality | `src/lib.rs` |

## Key Patterns to Study

### Pattern 1: Basic Contract Structure (hello_world)
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, symbol_short};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        // Your logic here
    }
}
```

### Pattern 2: Storage + TTL (increment)
```rust
pub fn increment(env: Env) -> u32 {
    let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
    count += 1;
    env.storage().instance().set(&COUNTER, &count);
    env.storage().instance().extend_ttl(50, 100);
    count
}
```

### Pattern 3: Auth (auth)
```rust
pub fn increment(env: Env, user: Address, value: u32) -> u32 {
    user.require_auth();
    // Each user has their own counter
    let key = DataKey::Counter(user);
    let mut count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    count += value;
    env.storage().persistent().set(&key, &count);
    count
}
```

### Pattern 4: Token (token)
```rust
// Full token with: mint, burn, transfer, approve, balance_of
// See: soroban-examples/token/src/contract.rs
// Modules: admin.rs, allowance.rs, balance.rs, metadata.rs, storage_types.rs
```

## Running All Tests

```bash
cd soroban-examples
cargo test --workspace
```

## Next Steps

After studying these examples:
1. Go to [modules/](../modules/) for copy-paste code snippets
2. Go to [skills/](../skills/) for AI-accelerated development
3. Go to [scaffold/](../scaffold/) to build a full-stack dApp

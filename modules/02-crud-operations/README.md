# Module 02 — CRUD Operations on Soroban

> Learn to Create, Read, Update, and Delete data on the Stellar blockchain.

## The Contract

See [src/lib.rs](src/lib.rs) for the complete code.

## Concepts

### Storage Types (Quick Overview)

| Type | Use For | Survives Expiry? | Cost |
|------|---------|-----------------|------|
| `instance()` | Config, admin address | Archived (recoverable) | Medium |
| `persistent()` | User data, balances | Archived (recoverable) | Highest |
| `temporary()` | Cache, nonces, sessions | **Deleted forever** | Cheapest |

For CRUD, we use **persistent** storage — it's the most like a traditional database.

### Data Keys

Soroban uses enum-based keys to organize storage:

```rust
#[contracttype]
pub enum DataKey {
    Record(u64),       // Individual record by ID
    Counter,           // Auto-increment counter
    Admin,             // Admin address
}
```

## CRUD Functions

### Create
```rust
pub fn create(env: Env, caller: Address, title: String, content: String) -> u64 {
    caller.require_auth();
    // Auto-increment ID
    let id: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0) + 1;
    // Save record
    let record = Record { id, owner: caller, title, content, created_at: env.ledger().timestamp() };
    env.storage().persistent().set(&DataKey::Record(id), &record);
    env.storage().instance().set(&DataKey::Counter, &id);
    id
}
```

### Read
```rust
pub fn read(env: Env, id: u64) -> Record {
    env.storage().persistent()
        .get(&DataKey::Record(id))
        .unwrap_or_else(|| panic!("Record not found"))
}
```

### Update
```rust
pub fn update(env: Env, caller: Address, id: u64, title: String, content: String) {
    caller.require_auth();
    let mut record: Record = Self::read(env.clone(), id);
    if record.owner != caller { panic!("Not the owner"); }
    record.title = title;
    record.content = content;
    env.storage().persistent().set(&DataKey::Record(id), &record);
}
```

### Delete
```rust
pub fn delete(env: Env, caller: Address, id: u64) {
    caller.require_auth();
    let record: Record = Self::read(env.clone(), id);
    if record.owner != caller { panic!("Not the owner"); }
    env.storage().persistent().remove(&DataKey::Record(id));
}
```

## Build & Test

```bash
stellar contract build
cargo test
```

## Deploy & Use

```bash
# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/crud_contract.wasm \
  --source-account student \
  --network testnet

# Create a record
stellar contract invoke --id <CONTRACT_ID> --source-account student --network testnet \
  -- create --caller <YOUR_ADDRESS> --title "Hello" --content "My first record"

# Read it back
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- read --id 1
```

## Key Takeaways

1. **Always use `require_auth()`** for write operations
2. **Owner checks** prevent unauthorized modifications
3. **Auto-increment IDs** use instance storage (shared counter)
4. **Record data** uses persistent storage (per-record)
5. **TTL extension** keeps your data alive on-chain

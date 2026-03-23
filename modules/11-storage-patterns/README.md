# Module 11 — Storage Patterns

> Master Soroban's three storage types — Instance, Persistent, and Temporary.

## The Three Storage Types

```
┌─────────────────────────────────────────────────────────────┐
│                    SOROBAN STORAGE                          │
├───────────────┬──────────────────┬──────────────────────────┤
│   Instance    │   Persistent     │   Temporary              │
│               │                  │                          │
│ Lives with    │ Independent      │ Independent              │
│ the contract  │ entries          │ entries                  │
│               │                  │                          │
│ Shared TTL    │ Per-key TTL      │ Per-key TTL              │
│               │                  │                          │
│ Archived on   │ Archived on      │ DELETED on               │
│ expiry        │ expiry           │ expiry                   │
│ (recoverable) │ (recoverable)    │ (GONE FOREVER)           │
│               │                  │                          │
│ Small data    │ Unlimited keys   │ Unlimited keys           │
│ only          │                  │                          │
│               │                  │                          │
│ Cost: Medium  │ Cost: Highest    │ Cost: Cheapest           │
└───────────────┴──────────────────┴──────────────────────────┘
```

## When to Use Each

### Instance Storage — `env.storage().instance()`

**Use for**: Data that EVERY contract call might need.

```rust
// Admin address, config, feature flags, counters
env.storage().instance().set(&DataKey::Admin, &admin);
env.storage().instance().set(&DataKey::Config, &config);
env.storage().instance().set(&DataKey::TokenCount, &0_u64);
```

**Key facts:**
- All instance data shares ONE TTL (the contract instance TTL)
- Extending any instance data extends ALL instance data + the contract itself
- Limited by ledger entry size (~64KB total)
- Archived on expiry (can be restored)

### Persistent Storage — `env.storage().persistent()`

**Use for**: Data that must live long-term and can be per-user/per-record.

```rust
// User balances, profiles, NFT ownership, records
env.storage().persistent().set(&DataKey::Balance(user), &amount);
env.storage().persistent().set(&DataKey::Profile(user), &profile);
env.storage().persistent().set(&DataKey::Owner(token_id), &owner);
```

**Key facts:**
- Each key has its own TTL
- Unlimited number of keys
- Most expensive per write
- Archived on expiry (can be restored with `RestoreFootprintOp`)

### Temporary Storage — `env.storage().temporary()`

**Use for**: Cheap, disposable data with natural expiration.

```rust
// Allowances, session data, price oracles, nonces
env.storage().temporary().set(&DataKey::Allowance(from, spender), &allowance_data);
env.storage().temporary().set(&DataKey::Nonce(user), &nonce);
env.storage().temporary().set(&DataKey::PriceCache, &price);
```

**Key facts:**
- Cheapest storage option
- Each key has its own TTL
- **DELETED FOREVER** when TTL expires — no recovery
- Anyone can extend any temporary entry's TTL
- **Never rely on expiry for security logic**

## TTL Management

### How TTL Works

```
TTL = number of ledgers until the entry expires
1 ledger ≈ 5 seconds
1 day ≈ 17,280 ledgers
```

### TTL Extension API

```rust
const DAY: u32 = 17280;

// Instance: extends ALL instance storage + contract
env.storage().instance().extend_ttl(
    6 * DAY,   // threshold: only extend if TTL < 6 days
    7 * DAY,   // extend_to: set TTL to 7 days
);

// Persistent: per-key
env.storage().persistent().extend_ttl(
    &DataKey::Balance(user),
    29 * DAY,  // threshold
    30 * DAY,  // extend_to
);

// Temporary: per-key
env.storage().temporary().extend_ttl(
    &DataKey::Session(id),
    0,         // threshold: always extend
    DAY,       // extend_to: 1 day
);
```

### Why Threshold?

The threshold prevents unnecessary TTL extensions:
```rust
// Without threshold: extends every time (wasteful)
env.storage().instance().extend_ttl(0, 7 * DAY);

// With threshold: only extends when TTL drops below 6 days
env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
```

### Standard TTL Constants

```rust
const DAY_IN_LEDGERS: u32 = 17280;

// Contract/config: 7-day cycle
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = INSTANCE_TTL - DAY_IN_LEDGERS;

// User data: 30-day cycle
const BALANCE_TTL: u32 = 30 * DAY_IN_LEDGERS;
const BALANCE_THRESHOLD: u32 = BALANCE_TTL - DAY_IN_LEDGERS;

// Temporary data: 1-day cycle
const TEMP_TTL: u32 = DAY_IN_LEDGERS;
```

## The Contract

See [src/lib.rs](src/lib.rs) for a demonstration of all three storage types.

## Storage Decision Tree

```
Is this data needed by every contract call?
├── Yes → Instance (admin, config, counters)
└── No
    ├── Must this data survive long-term?
    │   ├── Yes → Persistent (balances, profiles, records)
    │   └── No → Temporary (sessions, caches, nonces)
    └── Is it OK if this data is lost?
        ├── Yes → Temporary (cheapest)
        └── No → Persistent (recoverable)
```

## Testing Storage & TTL

```rust
#[test]
fn test_ttl() {
    let env = Env::default();
    env.mock_all_auths();

    // Configure ledger for TTL testing
    env.ledger().with_mut(|li| {
        li.sequence_number = 100_000;
        li.min_persistent_entry_ttl = 500;
        li.min_temp_entry_ttl = 100;
        li.max_entry_ttl = 15000;
    });

    // ... do operations ...

    // Check TTL values
    env.as_contract(&contract_id, || {
        let ttl = env.storage().instance().get_ttl();
        assert!(ttl > 5000);

        let bal_ttl = env.storage().persistent().get_ttl(&DataKey::Balance(user));
        assert!(bal_ttl > 10000);
    });
}
```

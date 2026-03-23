# Module 10 — Events & Logging

> Emit on-chain events so frontends, indexers, and explorers can track your contract.

## Why Events?

- **Transparency**: Anyone can see what happened in your contract
- **Indexing**: Off-chain services track events to build databases
- **Frontend**: Your UI can listen for events to update in real-time
- **Debugging**: Events help you trace contract execution

## The Contract

See [src/lib.rs](src/lib.rs) for complete code.

## Publishing Events

### Basic Event
```rust
// Publish an event with topics and data
env.events().publish(
    (symbol_short!("transfer"),),     // topics (indexed, searchable)
    (from.clone(), to.clone(), amount) // data (the payload)
);
```

### Event Structure
```
Event:
├── Topics (indexed) — used to filter/search events
│   ├── Topic 1: event type (e.g., "transfer")
│   ├── Topic 2: sender address
│   └── Topic 3: receiver address (up to 4 topics)
└── Data — the actual payload (any serializable type)
```

### Multiple Topics
```rust
// More topics = more searchable
env.events().publish(
    (
        symbol_short!("transfer"),  // topic 1: event type
        from.clone(),               // topic 2: sender
        to.clone(),                 // topic 3: receiver
    ),
    amount  // data
);
```

## Common Event Patterns

### Token Transfer Event
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    from.require_auth();
    // ... transfer logic ...

    env.events().publish(
        (symbol_short!("transfer"), from.clone(), to.clone()),
        amount,
    );
}
```

### Mint Event
```rust
env.events().publish(
    (symbol_short!("mint"), to.clone()),
    amount,
);
```

### Admin Change Event
```rust
env.events().publish(
    (symbol_short!("admin"),),
    (old_admin, new_admin),
);
```

### Custom Struct as Event Data
```rust
#[contracttype]
#[derive(Clone, Debug)]
pub struct TradeEvent {
    pub buyer: Address,
    pub seller: Address,
    pub token_id: u64,
    pub price: i128,
}

env.events().publish(
    (symbol_short!("trade"),),
    TradeEvent { buyer, seller, token_id, price },
);
```

## Debug Logging (dev only)

```rust
use soroban_sdk::log;

pub fn my_function(env: Env, value: i128) {
    log!(&env, "Processing value: {}", value);
    // ... logic ...
    log!(&env, "Result: {}", result);
}
```

> **Note**: `log!` output only appears in tests and debug builds. It's stripped from release WASM.

Build with logs:
```toml
# Cargo.toml
[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```
```bash
cargo build --target wasm32-unknown-unknown --profile release-with-logs
```

## Reading Events

### From CLI
```bash
stellar events list \
  --id <CONTRACT_ID> \
  --network testnet \
  --start-ledger 0
```

### From RPC
```bash
curl -s https://soroban-testnet.stellar.org \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"getEvents",
    "params": {
      "startLedger": 0,
      "filters": [{
        "type": "contract",
        "contractIds": ["YOUR_CONTRACT_ID"],
        "topics": [["AAAADwAAAAh0cmFuc2Zlcg=="]]
      }]
    }
  }' | jq
```

### From JavaScript (frontend)
```javascript
const events = await server.getEvents({
  startLedger: 0,
  filters: [{
    type: "contract",
    contractIds: [CONTRACT_ID],
  }],
});
```

## Important Notes

1. **Events are free** — they don't cost extra gas
2. **Events are discarded** if the transaction fails (panic, error, budget exceeded)
3. **Max 4 topics** per event
4. **Topics are indexed** — use them for fields you'll search by
5. **Data is not indexed** — put the full payload here
6. **Events persist** in ledger history — useful for building off-chain indexes

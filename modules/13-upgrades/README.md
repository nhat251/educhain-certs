# Module 13 — Contract Upgrades

> Upgrade a deployed contract's code without changing its address.

## Why Upgrades?

- Fix bugs in production
- Add new features
- Optimize gas usage
- Respond to security vulnerabilities

## How It Works

```
1. Upload new WASM to the network
2. Call upgrade() on your contract with the new WASM hash
3. Contract address stays the same
4. All existing storage is preserved
5. New code takes effect immediately
```

## The Upgrade Pattern

### Step 1: Add an `upgrade` function to your contract

```rust
use soroban_sdk::BytesN;

#[contracttype]
pub enum DataKey {
    Admin,
    Version,
}

pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
    // Only admin can upgrade
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();

    // Replace the contract's WASM code
    env.deployer().update_current_contract_wasm(new_wasm_hash);

    // Optionally update version
    let version: u32 = env.storage().instance()
        .get(&DataKey::Version).unwrap_or(0) + 1;
    env.storage().instance().set(&DataKey::Version, &version);
}
```

### Step 2: Deploy the new WASM

```bash
# Upload new WASM (doesn't deploy a new contract — just uploads the code)
stellar contract install \
  --wasm target/wasm32-unknown-unknown/release/my_contract_v2.wasm \
  --source-account admin \
  --network testnet

# Output: NEW_WASM_HASH (a hex string)
```

### Step 3: Call upgrade on the existing contract

```bash
stellar contract invoke \
  --id <EXISTING_CONTRACT_ID> \
  --source-account admin \
  --network testnet \
  -- upgrade \
  --new_wasm_hash <NEW_WASM_HASH>
```

## Important Notes

1. **Constructor is NOT called** on upgrade — only on first deploy
2. **Storage is preserved** — all existing data remains accessible
3. **Address stays the same** — all references to the contract still work
4. **A `SYSTEM` event is emitted** with old and new WASM hashes
5. **New code takes effect immediately** after the upgrade transaction

## Post-Upgrade Migration

If your new version changes the storage schema, add a migration function:

```rust
pub fn migrate(env: Env) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();

    let version: u32 = env.storage().instance()
        .get(&DataKey::Version).unwrap_or(0);

    // Only run migration once
    if version < 2 {
        // Migrate data from old format to new format
        // ... migration logic ...

        env.storage().instance().set(&DataKey::Version, &2_u32);
    }
}
```

## Upgrade Checklist

- [ ] Contract has an `upgrade()` function
- [ ] `upgrade()` is admin-gated with `require_auth()`
- [ ] New WASM is tested locally with `cargo test`
- [ ] Storage schema changes have a migration path
- [ ] Version number is tracked in storage
- [ ] Old WASM is backed up (for rollback if needed)

## Security Considerations

- **Always gate upgrades with admin auth** — otherwise anyone can replace your contract
- **Test the upgrade flow** on testnet before mainnet
- **Consider a timelock** — delay upgrades by X hours so users can review
- **Emit events** on upgrade so monitoring systems can alert

## See Also

- [examples/soroban-examples/upgradeable_contract](../../examples/soroban-examples/upgradeable_contract/)

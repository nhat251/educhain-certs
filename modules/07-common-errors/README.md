# Module 07 — Common Errors & Fixes

> Every error you'll hit during Soroban development, organized by stage.

## Setup Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `rustc: command not found` | Rust not installed or PATH not set | Run `source "$HOME/.cargo/env"` or restart terminal |
| `error[E0463]: can't find crate for std` | Missing `#![no_std]` in your contract | Add `#![no_std]` as the first line of `lib.rs` |
| `wasm32-unknown-unknown target not found` | WASM target not installed | `rustup target add wasm32-unknown-unknown` |
| `stellar: command not found` | Stellar CLI not installed | `cargo install --locked stellar-cli` |
| `linker cc not found` | Missing C compiler (Mac) | `xcode-select --install` |
| `cargo build` hangs on Mac M1 | Rosetta issue | Try: `arch -arm64 cargo build` |

## Build Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `expected X arguments, found Y` | Wrong number of function params | Check function signature matches your call |
| `mismatched types` | Wrong type passed to function | Check: i128 vs u64 vs u32 |
| `cannot find value/type` | Missing import or typo | Add the right `use soroban_sdk::{...}` |
| `the trait X is not implemented` | Missing derive macro | Add `#[derive(Clone, Debug)]` to your struct |
| `binary is too large (>64KB)` | Too much code | Use `opt-level = "z"`, LTO, strip symbols in `Cargo.toml` |
| `use of undeclared crate` | Wrong dependency name | Check `Cargo.toml` — use `soroban-sdk` |
| `duplicate definitions` | Same function name twice | Rename one of the functions |

### Fix: Cargo.toml Release Profile for Small WASM
```toml
[profile.release]
opt-level = "z"          # Optimize for size
overflow-checks = true   # Keep safety checks
debug = 0                # No debug info
strip = "symbols"        # Strip symbols
debug-assertions = false
panic = "abort"          # Smaller panic handler
codegen-units = 1        # Better optimization
lto = true               # Link-time optimization
```

## Deploy Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `Account not found` | Account not funded on this network | Run Friendbot: `curl "https://friendbot.stellar.org/?addr=YOUR_KEY"` |
| `op_underfunded` | Not enough XLM for fees + min balance | Fund account again with Friendbot |
| `tx_bad_seq` | Sequence number out of sync | Wait 5 seconds, retry |
| `Transaction simulation failed` | Contract has a bug | Run `cargo test` locally first |
| `Network not configured` | Didn't add network to CLI | See Module 06 — add network command |
| `Connection refused / timeout` | RPC server down or unreachable | Wait and retry, or try a different RPC URL |
| `wasm hash already exists` | Same WASM already deployed | This is OK — use the returned contract ID |

## Runtime / Invocation Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `HostError: Error(Contract, #1)` | Your custom error (check `#[contracterror]`) | Map error number to your enum |
| `HostError: Error(Auth, InvalidAction)` | `require_auth()` failed | Ensure the right account is `--source-account` |
| `HostError: Error(Storage, MissingValue)` | `.get().unwrap()` on missing key | Use `.get().unwrap_or(default)` or check `.has()` first |
| `HostError: Error(Budget, ExceededLimit)` | Too much computation | Simplify logic, reduce loops, use fewer storage calls |
| `HostError: Error(Object, ...)` | Invalid object passed | Check argument types match the contract |
| `panic: unwrap on None` | Called `.unwrap()` on empty storage | Use `.unwrap_or()` or `.ok_or()` |
| `op_no_trust` | Missing trustline for classic asset | Add `changeTrust` operation first |

### Decoding HostError Numbers

When you see `Error(Contract, #N)`, the `N` maps to your `#[contracterror]` enum:

```rust
#[contracterror]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,    // #1
    NotAuthorized = 2,     // #2
    InsufficientFunds = 3, // #3
}
```

## Frontend / SDK Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `Freighter not connected` | Extension not installed or wrong network | Install Freighter, switch to Testnet |
| `Account not found` (SDK) | Account not funded | Run Friendbot |
| `prepareTransaction failed` | Contract simulation error | Check contract ID is correct |
| `signTransaction failed` | User rejected in Freighter | User must click Approve |
| `sendTransaction timeout` | Network congestion | Retry after a few seconds |
| `CONTRACT_ID undefined` | Forgot to update after deploy | Copy contract ID from deploy output |
| `CORS error` | Browser blocking RPC call | Use the correct RPC URL, check browser extensions |

## Testing Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `env.register` fails | Wrong constructor args | Check if contract has `__constructor` |
| `mock_all_auths` doesn't work | Called after the operation | Call `env.mock_all_auths()` BEFORE client calls |
| `Address::generate` not found | Missing `testutils` feature | Add `soroban-sdk = { ..., features = ["testutils"] }` in `[dev-dependencies]` |
| Test panics unexpectedly | Contract panic in logic | Add more context to your panic messages |

## Debugging Workflow

```
1. cargo test           ← catch logic bugs locally
2. stellar contract build  ← catch compile errors
3. Deploy to testnet       ← catch network issues
4. Check Stellar Expert    ← verify tx went through
5. Check Stellar Lab       ← decode XDR if needed
```

### Pro Tip: Add Debug Logging

```rust
// In your contract (only works in debug builds)
log!(&env, "Balance for user: {}", balance);

// Build with logs enabled
cargo build --target wasm32-unknown-unknown --profile release-with-logs
```

Add to Cargo.toml:
```toml
[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```

## Emergency Fixes

### "I deployed the wrong contract"
You can't delete a deployed contract, but you can deploy a new one and use the new contract ID.

### "I lost my secret key"
If you're on futurenet/testnet: just generate a new identity. No real money lost.
On mainnet: your funds are gone. Always back up keys.

### "My contract data expired"
Persistent data can be restored with `RestoreFootprintOp`. Temporary data is gone forever.

### "I need to upgrade my contract"
See [Module 13 — Upgrades](../13-upgrades/) for the upgrade pattern.

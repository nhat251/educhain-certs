# Module 03 — Token Operations

> Issue, transfer, burn, and manage custom tokens on Soroban.

## What You'll Learn

- How to create a fungible token (like USDC or a loyalty point)
- Mint tokens to users
- Transfer between accounts
- Burn tokens
- Check balances
- Admin controls (freeze, set admin)

## The Contract

See [src/lib.rs](src/lib.rs) for the complete code.

## Core Concepts

### Token Interface

Soroban has a standard token interface. Any token that implements these functions is compatible with wallets, DEXes, and other contracts:

| Function | Purpose | Auth Required |
|----------|---------|---------------|
| `initialize` | Set name, symbol, decimals, admin | Once only |
| `mint` | Create new tokens → send to address | Admin |
| `transfer` | Move tokens between accounts | Sender |
| `burn` | Destroy tokens from an account | Token holder |
| `balance` | Check balance of an address | None |
| `allowance` | Check approved spending amount | None |
| `approve` | Allow another address to spend your tokens | Token holder |
| `transfer_from` | Spend on behalf of another (using allowance) | Approved spender |

### Storage Layout

```
Token State:
├── Admin          → Address (who can mint)
├── Name           → String ("My Token")
├── Symbol         → String ("MTK")
├── Decimals       → u32 (7)
├── Balance(addr)  → i128 (per-user balance)
└── Allowance(from, spender) → AllowanceData { amount, expiry }
```

## Quick Usage

### Deploy and Initialize
```bash
# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/simple_token.wasm \
  --source-account student --network testnet

# Initialize token
stellar contract invoke --id <CONTRACT_ID> --source-account student --network testnet \
  -- initialize \
  --admin <YOUR_ADDRESS> \
  --name "Campus Coin" \
  --symbol "CAMP" \
  --decimals 7
```

### Mint Tokens
```bash
stellar contract invoke --id <CONTRACT_ID> --source-account student --network testnet \
  -- mint --to <RECIPIENT_ADDRESS> --amount 1000000000
# 1000000000 = 100 tokens with 7 decimals
```

### Transfer Tokens
```bash
stellar contract invoke --id <CONTRACT_ID> --source-account student --network testnet \
  -- transfer --from <YOUR_ADDRESS> --to <RECIPIENT> --amount 500000000
```

### Check Balance
```bash
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- balance --id <ADDRESS>
```

## Key Patterns

### Decimal Handling
Tokens use integer math with a decimal offset. For 7 decimals:
- 1 token = 10,000,000 (10^7)
- 0.5 tokens = 5,000,000
- 100 tokens = 1,000,000,000

### Authorization Chain
```
mint:     admin.require_auth()           → only admin can create tokens
transfer: from.require_auth()            → only sender can send
burn:     from.require_auth()            → only holder can burn
approve:  from.require_auth()            → only holder can approve spending
```

### TTL Management for Balances
```rust
const DAY_IN_LEDGERS: u32 = 17280;
const BALANCE_TTL: u32 = 30 * DAY_IN_LEDGERS;      // 30 days
const BALANCE_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS; // extend when < 29 days
```

## Common Mistakes

| Mistake | Why It Fails | Fix |
|---------|-------------|-----|
| Forget decimals | Send 100 instead of 100 * 10^7 | Always multiply by 10^decimals |
| Double initialize | Contract already has admin | Check if already initialized |
| Transfer > balance | Insufficient funds panic | Check balance first |
| Wrong auth | Non-admin calling mint | Use the admin account |

## Next Steps

- [Module 04 — NFT Operations](../04-nft-operations/) for non-fungible tokens
- [Module 05 — Auth Patterns](../05-auth-patterns/) for advanced authorization

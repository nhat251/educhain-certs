# Module 09 — Tools & Inspection

> Debug, inspect, and explore your contracts and transactions.

## Essential Tools

### 1. Stellar Expert (Block Explorer)

**What**: View transactions, contracts, accounts, and assets on any Stellar network.

| Network | URL |
|---------|-----|
| Testnet | `https://stellar.expert/explorer/testnet` |
| Futurenet | `https://stellar.expert/explorer/futurenet` |
| Mainnet | `https://stellar.expert/explorer/public` |

**How to use:**
- Search by contract ID → see all invocations
- Search by account address → see all transactions
- Search by transaction hash → see full transaction details
- Click any operation to see inputs, outputs, and state changes

### 2. Stellar Laboratory

**URL**: `https://laboratory.stellar.org`

**What**: Build, sign, and submit transactions manually. Decode XDR.

**Key features:**
- **Transaction Builder**: Create transactions without code
- **XDR Viewer**: Decode any XDR string (useful for debugging `prepareTransaction` output)
- **Endpoint Explorer**: Test Horizon and Soroban RPC calls directly
- **Account Creator**: Fund testnet/futurenet accounts

### 3. Stellar CLI (your main tool)

```bash
# Check contract info
stellar contract info interface --id <CONTRACT_ID> --network testnet

# Fetch a deployed contract's WASM
stellar contract fetch --id <CONTRACT_ID> --network testnet > contract.wasm

# Read contract data directly
stellar contract read \
  --id <CONTRACT_ID> \
  --network testnet \
  --key <KEY>

# Restore expired persistent data
stellar contract restore \
  --id <CONTRACT_ID> \
  --key <KEY> \
  --source-account student \
  --network testnet
```

### 4. Freighter Wallet

**URL**: `https://freighter.app`

**What**: Browser extension wallet. Signs transactions from your dApp frontend.

**Setup for development:**
1. Install extension
2. Create/import account
3. Switch to Testnet (Settings → Network)
4. Fund via Friendbot

**Debugging tips:**
- Check "Recent transactions" in Freighter to see what was signed
- Make sure network matches (Testnet vs Futurenet)
- Check if the account has enough XLM for fees

### 5. Soroban RPC (direct API)

```bash
# Health check
curl -s https://soroban-testnet.stellar.org \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' | jq

# Get latest ledger
curl -s https://soroban-testnet.stellar.org \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getLatestLedger"}' | jq

# Get contract data
curl -s https://soroban-testnet.stellar.org \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"getContractData",
    "params": {
      "contract": "YOUR_CONTRACT_ID",
      "key": {"type":"ledgerKeyContractInstance"},
      "durability": "persistent"
    }
  }' | jq
```

## Debugging Workflow

### Level 1: Local Testing (fastest feedback)
```bash
# Write test → run test → fix → repeat
cargo test
cargo test -- --nocapture  # See println! output
```

### Level 2: CLI Invocation (test on network)
```bash
# Deploy and invoke via CLI
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account student \
  --network testnet \
  -- your_function --arg value

# Dry-run (simulate without submitting)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account student \
  --network testnet \
  --send no \
  -- your_function --arg value
```

### Level 3: Explorer Investigation
1. Copy transaction hash from CLI output
2. Paste into Stellar Expert
3. Check: Did the transaction succeed? What operations ran? What events emitted?

### Level 4: XDR Decoding
```bash
# If you have raw XDR from an error
# Paste into Stellar Laboratory → XDR Viewer → select type
```

## Monitoring Your Contract

### Watch for events
```bash
# Get recent events from your contract
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
        "contractIds": ["YOUR_CONTRACT_ID"]
      }]
    }
  }' | jq
```

### Check contract TTL
If your contract or its data entries expire, they become inaccessible.
Monitor TTL and extend when needed:

```bash
stellar contract extend \
  --id <CONTRACT_ID> \
  --source-account student \
  --network testnet \
  --ledgers-to-extend 518400  # ~30 days
```

## Tool Comparison

| Task | Best Tool |
|------|-----------|
| View transaction details | Stellar Expert |
| Decode XDR | Stellar Laboratory |
| Test contract functions | Stellar CLI |
| Build transactions manually | Stellar Laboratory |
| Sign transactions in browser | Freighter |
| Query account/balances | Stellar CLI or Horizon API |
| Debug contract logic | `cargo test` with `--nocapture` |
| Monitor events | Soroban RPC `getEvents` |
| Check WASM size | `ls -la target/.../*.wasm` |

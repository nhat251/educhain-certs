# SKILL: Soroban Deploy & Operations

> Use this reference when deploying, invoking, and managing Soroban contracts.

## Quick Deploy Flow

```bash
# 1. Build
stellar contract build

# 2. Create identity (one-time)
stellar keys generate deployer --network testnet --fund

# 3. Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/CONTRACT_NAME.wasm \
  --source-account deployer \
  --network testnet \
  --alias my-contract

# 4. Invoke
stellar contract invoke \
  --id my-contract \
  --source-account deployer \
  --network testnet \
  -- function_name --arg1 value1
```

## Network Setup (one-time)

```bash
# Testnet (default for development)
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Futurenet (experimental)
stellar network add futurenet \
  --rpc-url https://rpc-futurenet.stellar.org \
  --network-passphrase "Test SDF Future Network ; October 2022"
```

## Identity Management

```bash
# Generate new identity (auto-funded)
stellar keys generate NAME --network NETWORK --fund

# List identities
stellar keys ls

# Get address
stellar keys address NAME

# Import existing secret key
stellar keys add NAME --secret-key
```

## Fund Account

```bash
# Testnet (default)
curl "https://friendbot.stellar.org/?addr=$(stellar keys address NAME)"

# Futurenet
curl "https://friendbot-futurenet.stellar.org/?addr=$(stellar keys address NAME)"
```

## Build Commands

```bash
# Standard build
stellar contract build

# Build specific contract in workspace
stellar contract build --package my-contract

# Optimize WASM (reduces size significantly)
stellar contract optimize \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm

# Check WASM size
ls -lh target/wasm32-unknown-unknown/release/*.wasm
```

## Deploy Commands

```bash
# Basic deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source-account deployer \
  --network testnet

# Deploy with alias (easier to reference later)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source-account deployer \
  --network testnet \
  --alias my-contract

# Deploy with constructor args
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source-account deployer \
  --network testnet \
  -- --admin $(stellar keys address deployer)
```

## Invoke Commands

```bash
# Basic invoke
stellar contract invoke \
  --id CONTRACT_ID_OR_ALIAS \
  --source-account deployer \
  --network testnet \
  -- function_name --arg1 value1

# Read-only (no source needed, simulated locally)
stellar contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- get_value --key "test"

# Dry-run (simulate without submitting)
stellar contract invoke \
  --id CONTRACT_ID \
  --source-account deployer \
  --network testnet \
  --send no \
  -- my_function --arg value
```

## Argument Types

```bash
# String
-- set_name --name "Hello World"

# Integer
-- set_value --amount 1000

# Address
-- transfer --from GABC... --to GXYZ...

# Boolean
-- set_flag --enabled true

# Bytes (hex)
-- upgrade --new_wasm_hash abc123def456...
```

## Contract Inspection

```bash
# View contract interface (functions + types)
stellar contract info interface \
  --id CONTRACT_ID \
  --network testnet

# Fetch deployed WASM
stellar contract fetch \
  --id CONTRACT_ID \
  --network testnet > contract.wasm

# Read contract storage
stellar contract read \
  --id CONTRACT_ID \
  --network testnet
```

## Upgrade Flow

```bash
# 1. Build new version
stellar contract build

# 2. Upload new WASM (get hash, doesn't deploy new contract)
stellar contract install \
  --wasm target/wasm32-unknown-unknown/release/my_contract_v2.wasm \
  --source-account deployer \
  --network testnet
# Output: NEW_WASM_HASH

# 3. Call upgrade on existing contract
stellar contract invoke \
  --id EXISTING_CONTRACT_ID \
  --source-account deployer \
  --network testnet \
  -- upgrade --new_wasm_hash NEW_WASM_HASH
```

## TTL Extension

```bash
# Extend contract TTL
stellar contract extend \
  --id CONTRACT_ID \
  --source-account deployer \
  --network testnet \
  --ledgers-to-extend 518400  # ~30 days

# Restore expired data
stellar contract restore \
  --id CONTRACT_ID \
  --source-account deployer \
  --network testnet
```

## Scaffold Stellar (Full-Stack)

```bash
# Install scaffold CLI
cargo install --locked stellar-scaffold-cli

# Create full-stack project
stellar scaffold init my-dapp
cd my-dapp
npm install

# Build contracts + generate TS clients
stellar scaffold build --build-clients

# Start dev server
npm run dev

# Generate contract from template
stellar scaffold generate contract --from token
```

## Troubleshooting Quick Reference

| Error | Fix |
|-------|-----|
| Account not found | Run Friendbot |
| tx_bad_seq | Wait 5s, retry |
| op_underfunded | Fund account again |
| Simulation failed | Check contract ID, run tests first |
| WASM too large | Run `stellar contract optimize` |
| Network not found | Run `stellar network add` |

## Verification

```bash
# View on Stellar Expert
echo "https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"

# View transaction
echo "https://stellar.expert/explorer/testnet/tx/$TX_HASH"
```

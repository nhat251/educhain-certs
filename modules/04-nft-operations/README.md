# Module 04 — NFT Operations

> Mint, transfer, and manage non-fungible tokens (NFTs) on Soroban.

## What Is an NFT on Soroban?

Unlike Ethereum's ERC-721, Soroban doesn't have a built-in NFT standard yet.
We implement NFTs as **unique token IDs** with metadata stored on-chain.

Each NFT has:
- A unique `token_id` (u64)
- An `owner` (Address)
- `metadata` (name, description, URI)

## The Contract

See [src/lib.rs](src/lib.rs) for the complete code.

## Core Functions

| Function | Purpose | Auth |
|----------|---------|------|
| `mint` | Create a new NFT | Admin |
| `transfer` | Transfer NFT to another address | Owner |
| `owner_of` | Get the owner of an NFT | Public |
| `metadata` | Get NFT metadata | Public |
| `burn` | Destroy an NFT | Owner |
| `total_minted` | Count of NFTs created | Public |

## Usage

### Mint an NFT
```bash
stellar contract invoke --id <CONTRACT_ID> --source-account admin --network testnet \
  -- mint \
  --to <RECIPIENT_ADDRESS> \
  --name "My Art #1" \
  --description "Digital artwork" \
  --uri "https://example.com/nft/1.json"
```

### Transfer
```bash
stellar contract invoke --id <CONTRACT_ID> --source-account owner --network testnet \
  -- transfer \
  --from <OWNER_ADDRESS> \
  --to <NEW_OWNER> \
  --token_id 1
```

### Check Owner
```bash
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- owner_of --token_id 1
```

## Key Design Decisions

1. **Sequential IDs**: Auto-incrementing for simplicity (vs. random hashes)
2. **On-chain metadata**: Small metadata stored directly on-chain (name, description, URI)
3. **Off-chain assets**: Actual images/files stored via URI (IPFS, Arweave, or HTTP)
4. **Admin-only minting**: Only the admin can create NFTs (modify for open minting)

## NFT vs Token Comparison

| Feature | Fungible Token (Module 03) | NFT (This Module) |
|---------|---------------------------|---------------------|
| Each unit is... | Interchangeable | Unique |
| Balance type | `i128` (amount) | `bool` (own or don't) |
| Key storage | `Balance(Address) → amount` | `Owner(token_id) → Address` |
| Use cases | Currency, points, shares | Art, tickets, certificates |

## Next Steps

- [Module 05 — Auth Patterns](../05-auth-patterns/) for advanced access control
- Combine with Module 03 to build a marketplace (list NFT → pay with token)

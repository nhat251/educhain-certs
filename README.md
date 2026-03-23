# Soroban Get Started — Complete Learning Hub

> Your one-stop repo for learning Stellar smart contract development with Soroban.
> From zero blockchain knowledge to deploying production-ready dApps.

**Author:** Verner Huang — DevRel, Rise In x Stellar

## Who Is This For?

- Students with **zero blockchain experience** who want to build on Stellar
- Developers who want to **learn Soroban fast** with real code examples
- Hackathon participants who need to **ship a dApp in hours, not weeks**

## Repository Structure

```
soroban-bootcamp/
│
├── scaffold/                  ← Official Stellar scaffold (install via CLI)
│   └── README.md              ← Step-by-step scaffold setup guide
│
├── examples/                  ← Official Soroban examples (34 contracts)
│   ├── soroban-examples/      ← Cloned from stellar/soroban-examples v23.0.0
│   └── README.md              ← Guide to each example + what to learn
│
├── modules/                   ← Bite-sized code modules + best practices
│   ├── 01-environment-setup/  ← Install Rust, Stellar CLI, wallets
│   ├── 02-crud-operations/    ← Create, Read, Update, Delete on-chain
│   ├── 03-token-operations/   ← Issue, transfer, burn custom tokens
│   ├── 04-nft-operations/     ← Mint, transfer, metadata for NFTs
│   ├── 05-auth-patterns/      ← require_auth, admin, multi-sig
│   ├── 06-deploy-guide/       ← Deploy to testnet/futurenet step-by-step
│   ├── 07-common-errors/      ← Every error you'll hit + how to fix it
│   ├── 08-best-practices/     ← Production-quality contract patterns
│   ├── 09-tools-and-inspection/ ← Stellar Expert, Laboratory, debugging
│   ├── 10-events-and-logging/ ← Emit and read on-chain events
│   ├── 11-storage-patterns/   ← Instance vs Persistent vs Temporary
│   ├── 12-cross-contract/     ← Call other contracts from your contract
│   └── 13-upgrades/           ← Upgrade deployed contracts safely
│
├── skills/                    ← AI skills files for god-speed development
│   ├── soroban-contract.md    ← Write smart contracts 10x faster
│   ├── soroban-deploy.md      ← Deploy + invoke in one flow
│   ├── frontend-dapp.md       ← Build dApp UI with AI acceleration
│   ├── testing.md             ← Test contracts properly
│   └── full-stack-dapp.md     ← End-to-end dApp in 1 hour
│
└── CLAUDE.md                  ← Claude Code project instructions
```

## Quick Start (5 minutes)

### 1. Set Up Environment

> Full setup guide: [Stellar Docs — Getting Started](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli

# Install Node.js (for frontend)
# Download from https://nodejs.org
```

### 2. Clone This Repo

```bash
git clone https://github.com/hien17/soroban-bootcamp.git
cd soroban-bootcamp
```

### 3. Try Your First Contract

```bash
cd examples/soroban-examples/hello_world
stellar contract build
cargo test
```

### 4. Deploy to Testnet

```bash
# Create and fund a test account
stellar keys generate student --network testnet --fund

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source-account student \
  --network testnet
```

### 5. Build a Full dApp

Read [scaffold/README.md](scaffold/README.md) to set up a full-stack project with React frontend.

## Learning Path

| Step | What | Time | Folder |
|------|------|------|--------|
| 1 | Environment setup | 15 min | `modules/01-environment-setup/` |
| 2 | Read hello_world example | 10 min | `examples/soroban-examples/hello_world/` |
| 3 | Understand storage types | 15 min | `modules/11-storage-patterns/` |
| 4 | CRUD operations | 20 min | `modules/02-crud-operations/` |
| 5 | Token operations | 20 min | `modules/03-token-operations/` |
| 6 | Auth patterns | 15 min | `modules/05-auth-patterns/` |
| 7 | Deploy to testnet | 15 min | `modules/06-deploy-guide/` |
| 8 | Build full dApp | 60 min | `scaffold/` + `skills/` |

## Using AI to Accelerate Development

The `skills/` folder contains Claude Code skill files that turn AI into your pair programmer.
Load them into Claude Code and develop at god-speed while maintaining quality.

See [skills/README.md](skills/README.md) for setup instructions.

## Resources

- [Stellar Developer Docs](https://developers.stellar.org)
- [Soroban Smart Contracts](https://developers.stellar.org/docs/build/smart-contracts)
- [Scaffold Stellar](https://scaffoldstellar.org)
- [Stellar Expert (Block Explorer)](https://stellar.expert)
- [Stellar Laboratory](https://laboratory.stellar.org)
- [Freighter Wallet](https://freighter.app)
- [Soroban Examples (GitHub)](https://github.com/stellar/soroban-examples)

## License

Educational use. Example code from Stellar is Apache 2.0 licensed.

# Scaffold Stellar — Official Full-Stack Starter

> The fastest way to go from zero to a working Stellar dApp with smart contracts + React frontend.

## What Is Scaffold Stellar?

[Scaffold Stellar](https://scaffoldstellar.org) is the official developer toolkit that gives you:

- Pre-configured Rust smart contract workspace
- React + TypeScript frontend with auto-generated contract bindings
- One-command build, deploy, and hot-reload
- On-chain contract registry for publishing reusable contracts
- OpenZeppelin contract templates (token, NFT, etc.)

## Prerequisites

| Tool | Install Command |
|------|----------------|
| **Rust** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **WASM target** | `rustup target add wasm32-unknown-unknown` |
| **Node.js v18+** | Download from [nodejs.org](https://nodejs.org) |
| **Stellar CLI** | `cargo install --locked stellar-cli` |
| **Docker** (optional) | Download from [docker.com](https://docker.com) — for local node |

## Quick Start (5 minutes)

### Step 1: Install Scaffold Stellar CLI

```bash
cargo install --locked stellar-scaffold-cli
```

This installs as a plugin under the `stellar` command.

### Step 2: Create a New Project

```bash
stellar scaffold init my-dapp
cd my-dapp
```

### Step 3: Install Frontend Dependencies

```bash
npm install
```

### Step 4: Start Development Server

```bash
npm run dev
```

Open `http://localhost:5173` — your dApp is running!

## Project Structure (What You Get)

```
my-dapp/
├── contracts/            # Rust smart contracts (compiled to WASM)
│   └── hello_world/      # Default starter contract
│       ├── src/
│       │   └── lib.rs    # Your contract code
│       └── Cargo.toml
├── packages/             # Auto-generated TypeScript clients
├── src/                  # React frontend
│   ├── components/       # Reusable UI components
│   ├── contracts/        # Contract interaction logic
│   ├── App.tsx           # Main component
│   └── main.tsx          # Entry point
├── environments.toml     # Network configuration (testnet/futurenet/mainnet)
├── .env                  # Local environment variables
├── package.json          # Frontend dependencies
└── target/               # Build outputs (git-ignored)
```

## CLI Commands Reference

### Create project
```bash
stellar scaffold init <project-path>
```

### Generate contract from template
```bash
# List available templates
stellar scaffold generate contract --ls

# Generate from OpenZeppelin example
stellar scaffold generate contract --from token

# Use browser-based wizard
stellar scaffold generate contract --from-wizard
```

### Build contracts + generate TypeScript clients
```bash
stellar scaffold build --build-clients
```

### Watch mode (hot reload)
```bash
stellar scaffold watch --build-clients
```

### Upgrade existing Soroban project to scaffold
```bash
stellar scaffold upgrade ./my-existing-project
```

## Environment Configuration

Edit `environments.toml` to configure networks:

```toml
[development]
name = "testnet"
accounts = ["student"]

[development.contracts.hello_world]
client = true

[staging]
name = "futurenet"
accounts = [{ name = "deployer", default = true }]

[staging.contracts.hello_world]
client = true
```

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `STELLAR_SCAFFOLD_ENV` | Select which environment to use |
| `STELLAR_ACCOUNT` | Default transaction account |
| `STELLAR_RPC_URL` | RPC endpoint (auto-set from config) |

## Contract Registry

Scaffold Stellar includes an on-chain registry for publishing and deploying reusable contracts.

### Publish a contract
```bash
stellar registry publish \
  --wasm target/stellar/my_contract.wasm \
  --wasm-name unverified/my-contract \
  --binver "1.0.0"
```

### Deploy from registry
```bash
stellar registry deploy \
  --contract-name unverified/my-contract-instance \
  --wasm-name unverified/my-contract
```

### Deploy with constructor args (e.g., token)
```bash
stellar registry deploy \
  --contract-name unverified/my-token \
  --wasm-name unverified/token \
  --version "1.0.0" \
  -- \
  --name "My Token" \
  --symbol "MTK" \
  --decimals 7
```

## Video Tutorials

- [Scaffold Stellar Playlist (SDF YouTube)](https://www.youtube.com/playlist?list=PLmr3tp_7-7Gjj6gn5-bBn-QTMyaWzwOU5)

## Next Steps

1. Read the [modules/](../modules/) folder for code patterns you can use in your contracts
2. Check [skills/](../skills/) for AI-accelerated development workflows
3. Browse [examples/](../examples/) to see 34 official Soroban contract examples

## Links

- [Scaffold Stellar Docs](https://scaffoldstellar.org/docs/intro)
- [GitHub Repo](https://github.com/theahaco/scaffold-stellar)

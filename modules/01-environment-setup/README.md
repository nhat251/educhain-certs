# Module 01 — Environment Setup

> Get your machine ready for Stellar/Soroban development in 15 minutes.

## Prerequisites Checklist

- [ ] macOS, Linux, or Windows (WSL2)
- [ ] Terminal access
- [ ] Internet connection
- [ ] ~2GB disk space

## Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, choose option 1 (default installation).

After install, restart your terminal or run:
```bash
source "$HOME/.cargo/env"
```

Verify:
```bash
rustc --version
# Should show: rustc 1.84.0 or higher
```

## Step 2: Add WebAssembly Target

Soroban contracts compile to WASM. You need this target:

```bash
rustup target add wasm32-unknown-unknown
```

> **Note**: Re-run this command after every `rustup update`.

## Step 3: Install Stellar CLI

```bash
cargo install --locked stellar-cli
```

Alternative methods:
```bash
# macOS with Homebrew
brew install stellar-cli

# Quick install script
curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh
```

Verify:
```bash
stellar --version
# Should show: stellar 25.x.x or higher
```

Enable shell autocompletion (optional but recommended):
```bash
# For bash
echo 'source <(stellar completion --shell bash)' >> ~/.bashrc

# For zsh
echo 'source <(stellar completion --shell zsh)' >> ~/.zshrc
```

## Step 4: Install Node.js (for frontend)

Download from [nodejs.org](https://nodejs.org) — choose the LTS version (v18+).

Verify:
```bash
node --version
npm --version
```

## Step 5: Install Freighter Wallet

1. Go to [freighter.app](https://freighter.app)
2. Install the browser extension (Chrome/Firefox/Brave)
3. Create a new wallet — **save your recovery phrase!**
4. Switch network to **Testnet**:
   - Click the gear icon in Freighter
   - Go to Network Settings
   - Select "Testnet"

## Step 6: Fund Your Test Account

### Option A: Friendbot (browser)
Open this URL, replacing `YOUR_PUBLIC_KEY` with your Freighter address:
```
https://friendbot.stellar.org/?addr=YOUR_PUBLIC_KEY
```

### Option B: Stellar CLI
```bash
# Generate a new identity and fund it automatically
stellar keys generate student --network testnet --fund

# Check your address
stellar keys address student
```

### Option C: Script
```bash
curl -s "https://friendbot.stellar.org/?addr=$(stellar keys address student)" | head -1
```

## Step 7: Editor Setup (VS Code)

Install these extensions:
1. **rust-analyzer** — Rust language support (autocomplete, errors, go-to-definition)
2. **CodeLLDB** — Rust debugger
3. **Even Better TOML** — Cargo.toml syntax highlighting

Recommended VS Code settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
  "editor.formatOnSave": true
}
```

## Step 8: Verify Everything Works

```bash
# Create a test project
stellar contract init test-project
cd test-project

# Build
stellar contract build

# Run tests
cargo test

# Clean up
cd ..
rm -rf test-project
```

If all commands succeed, you're ready to build!

## Networks Reference

| Network | Purpose | RPC URL | Friendbot |
|---------|---------|---------|-----------|
| **Testnet** | Default development | `https://soroban-testnet.stellar.org` | `https://friendbot.stellar.org` |
| **Futurenet** | Experimental | `https://rpc-futurenet.stellar.org` | `https://friendbot-futurenet.stellar.org` |
| **Mainnet** | Production | `https://mainnet.sorobanrpc.com` | N/A |

> **Note**: RPC URLs are JSON-RPC endpoints (POST only). Visiting them in a browser will show HTTP 405 — this is expected. Use `stellar contract invoke` or `curl -X POST` to interact with them.

### Add networks to CLI (one-time)
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

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `rustc` not found | Run `source "$HOME/.cargo/env"` or restart terminal |
| `wasm32` target missing | Run `rustup target add wasm32-unknown-unknown` |
| `stellar` not found | Ensure `~/.cargo/bin` is in your PATH |
| Freighter won't connect | Make sure you're on the right network (Testnet) |
| Friendbot returns error | Your address may already be funded. Check on Stellar Expert |
| Build fails on Mac M1/M2 | Install Xcode CLI: `xcode-select --install` |

## Next Steps

Go to [Module 02 — CRUD Operations](../02-crud-operations/) to write your first contract.

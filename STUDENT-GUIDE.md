# Student Guide — From Zero to Deployed dApp on Stellar

> Rise In x Stellar University Tour
>
> **Author:** Verner Huang — DevRel, Rise In x Stellar
>
> This guide walks you through everything — from preparing your laptop at home to submitting your project after the session. Follow it step by step.

---

## Table of Contents

- [Before the Session (Do This at Home)](#before-the-session-do-this-at-home)
- [Part 1: Understand the Basics](#part-1-understand-the-basics)
- [Part 2: Choose Your Project Idea](#part-2-choose-your-project-idea)
- [Part 3: Write Your Smart Contract](#part-3-write-your-smart-contract)
- [Part 4: Deploy to Stellar Testnet](#part-4-deploy-to-stellar-testnet)
- [Part 5: Build Your Frontend (Optional)](#part-5-build-your-frontend-optional)
- [Part 6: Submit Your Project](#part-6-submit-your-project)
- [After the Session](#after-the-session)
- [Help & Resources](#help--resources)

---

## Before the Session (Do This at Home)

> Budget ~30 minutes the night before. This saves you from wasting session time on installation.
>
> Full setup reference: [Stellar Docs — Getting Started](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

### 1. Install Rust

Rust is the language used to write Stellar smart contracts.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, choose **option 1** (default). After it finishes, restart your terminal or run:

```bash
source "$HOME/.cargo/env"
```

Verify it works:
```bash
rustc --version
# You should see: rustc 1.84.0 or higher
```

### 2. Install the WebAssembly Target

Soroban contracts compile to WebAssembly (WASM). Add this target:

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Stellar CLI

This is the tool you use to build, deploy, and interact with contracts.

```bash
cargo install --locked stellar-cli
```

> This may take a few minutes. If you have trouble, try: `brew install stellar-cli` (Mac) or use the [quick install script](https://github.com/stellar/stellar-cli).

Verify:
```bash
stellar --version
# Should show: stellar 25.x.x or higher
```

### 4. Install Node.js (for frontend)

Download from [nodejs.org](https://nodejs.org) — choose the **LTS** version (v18 or higher).

### 5. Install Freighter Wallet

Freighter is the browser wallet for Stellar — like MetaMask but for Stellar.

1. Go to [freighter.app](https://freighter.app)
2. Click "Add to Chrome" (works on Chrome, Brave, Firefox)
3. Create a new wallet — **write down your recovery phrase and keep it safe!**
4. Switch to **Testnet** network:
   - Open Freighter → click the gear icon
   - Go to Network Settings
   - Select **Testnet**

### 6. Install VS Code + Extensions

Download [VS Code](https://code.visualstudio.com) and install these extensions:

- **rust-analyzer** — Rust autocomplete + error checking
- **Even Better TOML** — Cargo.toml highlighting

### 7. Clone This Repository

```bash
git clone https://github.com/hien17/soroban-bootcamp.git
cd soroban-bootcamp
```

### Pre-Session Checklist

Before you arrive, make sure you can run:

```bash
rustc --version     # ✓ Rust installed
stellar --version   # ✓ Stellar CLI installed
node --version      # ✓ Node.js installed
```

And confirm:
- [ ] Freighter wallet installed and switched to **Testnet**
- [ ] VS Code with rust-analyzer extension
- [ ] This repo cloned on your machine

> **Can't install something?** Don't worry — the DevRel team will have backup options (USB drives, cloud environments) at the session. But try your best to set up beforehand.

---

## Part 1: Understand the Basics

> You don't need to memorize any of this. Just read through it once so the vocabulary feels familiar during the session.

### What Is a Blockchain?

A blockchain is a shared database that nobody controls alone. Every transaction is recorded permanently and publicly — nobody can secretly change it.

**Simple analogy:** Imagine a Google Sheet shared with 10,000 people. Everyone can read it, nobody can secretly edit it, and every change is timestamped forever.

### What Is Stellar?

Stellar is a blockchain designed for **fast, cheap financial transactions**. Key facts:

| Feature | Stellar |
|---------|---------|
| Transaction time | ~5 seconds |
| Transaction fee | ~$0.000003 (basically free) |
| Built-in exchange | Yes (native DEX) |
| Smart contracts | Yes (Soroban — Rust-based) |
| Used by | MoneyGram, Franklin Templeton, UNHCR, Paxos |

### What Is Soroban?

Soroban is Stellar's smart contract platform. You write contracts in **Rust**, compile them to **WebAssembly (WASM)**, and deploy them on the Stellar network.

**Smart contract** = code that runs on the blockchain. Once deployed, it runs exactly as written — no one can change it. Think of it as backend logic that lives on-chain instead of on a server.

### Key Vocabulary

| Term | What It Means | Analogy |
|------|--------------|---------|
| **Wallet** | Your identity on the blockchain (a keypair) | Username + password |
| **Public key** | Your address (safe to share) | Email address |
| **Private/Secret key** | Proves you own the wallet (NEVER share) | Password |
| **Transaction (tx)** | An action recorded on-chain | API request |
| **Contract** | Code deployed on the blockchain | Backend server logic |
| **Token** | A digital asset on-chain | Database record of ownership |
| **Testnet** | Stellar's default testing network (free fake money) | Staging server |
| **Futurenet** | Stellar's experimental/cutting-edge network | Dev playground |
| **Gas/Fee** | Cost to process your transaction | API call cost |
| **Friendbot** | Free faucet that gives you test XLM | "Get free credits" button |

### The 3 Things Every dApp Needs

```
1. Smart Contract (Rust)  → The business logic that runs on-chain
2. Frontend (HTML/JS)     → The UI that users interact with
3. Wallet (Freighter)     → How users sign and approve transactions
```

### Recommended Reading (Optional)

If you want to go deeper before the session:

- [Stellar Developer Docs — Learn](https://developers.stellar.org/docs/learn/fundamentals) — Official intro
- [Soroban Smart Contracts Docs](https://developers.stellar.org/docs/build/smart-contracts) — Official smart contract docs
- [`modules/01-environment-setup/`](modules/01-environment-setup/) — Detailed setup guide in this repo
- [`modules/11-storage-patterns/`](modules/11-storage-patterns/) — How data is stored on Soroban

---

## Part 2: Choose Your Project Idea

> Goal: Have a clear, buildable idea in ~20 minutes. Don't overthink it — keep it simple.

### The One-Question Test

> **"What is the ONE on-chain transaction my app does?"**

If you can't answer this in one sentence, your idea is too complex. Simplify.

Good examples:
- "User sends USDC to another user" (payment app)
- "Admin mints a certificate token to a student" (certificate system)
- "User votes on a proposal" (voting app)
- "User deposits tokens into an escrow" (escrow/marketplace)

### Write Your PRD (1 page)

Fill this out before you start coding:

```
PROJECT NAME: _______________________

PROBLEM (1 sentence):
Who suffers from what problem today?

SOLUTION (1 sentence):
How does your dApp solve it using Stellar?

STELLAR FEATURE USED:
[ ] XLM/USDC transfer    [ ] Custom token    [ ] Soroban contract
[ ] Built-in DEX          [ ] Trustline       [ ] Clawback/Compliance

TARGET USER:
Who specifically uses this? (students / farmers / freelancers / etc.)

CORE FEATURE (MVP):
What is the ONE transaction that proves this works?

WHY STELLAR:
What would this cost/take on traditional finance or another chain?
```

### Default Project Ideas (Pick One If Stuck)

These are pre-scoped to be buildable in the session time. **Bold = recommended for first-timers.**

#### Payments & Remittances
- **Remittance Visualizer** — Show fee comparison (SWIFT 5% vs Stellar $0.000003), send USDC on testnet
- **Freelancer Payment Portal** — International clients pay freelancers in USDC
- **Split Bill dApp** — Split a restaurant bill, Stellar converts currencies
- **Campus Canteen Credit** — Pre-load meal credits as custom token
- **Tip Jar** — Content creators receive micropayment tips

#### Tokenized Assets
- **Coffee Farm Yield Token** — Farmer tokenizes harvest, investors buy shares
- **University Scholarship Fund** — Issue scholarship tokens, track donations transparently
- **Event Ticket Token** — NFT-style ticket with clawback for fraud prevention
- **University Certificate Token** — Tamper-proof digital diploma on-chain

#### Campus & Student Life
- **Campus Loyalty Token** — Earn tokens for attending lectures, redeem for benefits
- **Peer Tutoring Payment** — Pay tutors in campus token after session
- **Student Club Membership** — Join a club by minting a membership token
- **Textbook Marketplace** — Buy/sell textbooks with custom campus currency
- **Scholarship DAO** — Community votes on who receives scholarship pool

#### Voting & Governance
- **Community Voting Portal** — Token-weighted votes for decisions
- **Study Group DAO** — Pool XLM for shared subscriptions, vote on spending
- **Hackathon Leaderboard** — On-chain scoring with prize distribution

#### DeFi
- **Simple Savings Club** — 5 friends pool XLM monthly, distribute to one member each month
- **Freelance Escrow** — Milestone-based payment release with dispute resolution
- **Token Vesting Contract** — 6-month cliff vesting for startup equity tokens

> See the full list of 100 project ideas in the repo docs (shared by DevRel during the session).

### Scoping Tips

| If your idea is... | Do this... |
|---------------------|-----------|
| Too big | Build just the payment/transfer part today. The rest is homework. |
| Not blockchain-related | Ask: "Can this use Stellar's tokens or payments?" |
| A clone of something on Ethereum | Great! Stellar is cheaper and faster. Focus on that difference. |
| Something you've never seen | Even better — that's innovation. Keep the MVP tiny. |

---

## Part 3: Write Your Smart Contract

> Goal: A working contract with at least 1-2 functions, compiled and tested locally.

### Step 1: Set Up Your Project

```bash
cd soroban-bootcamp

# Create your contract directory
mkdir -p contracts/my-project/src

# Create Cargo.toml for your contract
cat > contracts/my-project/Cargo.toml << 'EOF'
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = "22"

[dev-dependencies]
soroban-sdk = { version = "22", features = ["testutils"] }
EOF
```

### Step 2: Write Your Contract

Create `contracts/my-project/src/lib.rs`. Pick the template closest to your idea:

#### Template A: Token / Points System

Use this for: loyalty tokens, campus credits, tip jars, payment systems.

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalSupply,
    Name,
    Symbol,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
}

#[contract]
pub struct MyToken;

#[contractimpl]
impl MyToken {
    /// Called once when the contract is deployed
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
    }

    /// Admin creates new tokens and sends them to an address
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let balance: i128 = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), 29 * DAY, 30 * DAY);

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);

        env.events().publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    /// Transfer tokens from one account to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        from.require_auth(); // Sender must sign this transaction

        let from_bal = Self::balance(env.clone(), from.clone());
        if from_bal < amount { return Err(Error::InsufficientBalance); }
        let to_bal = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_bal + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(from), 29 * DAY, 30 * DAY);
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), 29 * DAY, 30 * DAY);

        env.events().publish((symbol_short!("transfer"), from, to), amount);
        Ok(())
    }

    /// Check balance of any address (anyone can call this)
    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn name(env: Env) -> String { env.storage().instance().get(&DataKey::Name).unwrap() }
    pub fn symbol(env: Env) -> String { env.storage().instance().get(&DataKey::Symbol).unwrap() }
    pub fn total_supply(env: Env) -> i128 { env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0) }
}

// ============================================================
// TESTS — Run with: cargo test
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_full_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(MyToken, ());
        let client = MyTokenClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        // Initialize
        client.initialize(
            &admin,
            &String::from_str(&env, "Campus Coin"),
            &String::from_str(&env, "CAMP"),
        );

        // Mint 1000 to Alice
        client.mint(&alice, &1000);
        assert_eq!(client.balance(&alice), 1000);

        // Alice sends 300 to Bob
        client.transfer(&alice, &bob, &300);
        assert_eq!(client.balance(&alice), 700);
        assert_eq!(client.balance(&bob), 300);
    }
}
```

#### Template B: Registry / Certificate / Record System

Use this for: certificates, tickets, attendance, profiles, any create-and-read system.

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin,
    Count,
    Record(u64),
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Record {
    pub id: u64,
    pub owner: Address,
    pub data: String,
    pub timestamp: u64,
}

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Count, &0_u64);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
    }

    /// Create a new record (anyone can call, must sign)
    pub fn register(env: Env, owner: Address, data: String) -> u64 {
        owner.require_auth();
        let id: u64 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        let record = Record { id, owner, data, timestamp: env.ledger().timestamp() };
        env.storage().persistent().set(&DataKey::Record(id), &record);
        env.storage().instance().set(&DataKey::Count, &id);
        env.storage().persistent().extend_ttl(&DataKey::Record(id), 89 * DAY, 90 * DAY);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
        env.events().publish((symbol_short!("register"),), id);
        id
    }

    /// Read a record by ID (anyone can call)
    pub fn get(env: Env, id: u64) -> Record {
        env.storage().persistent().get(&DataKey::Record(id))
            .unwrap_or_else(|| panic!("Record not found"))
    }

    /// Total records created
    pub fn count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_register_and_get() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(Registry, ());
        let client = RegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let user = Address::generate(&env);
        let id = client.register(&user, &String::from_str(&env, "My Certificate"));

        assert_eq!(id, 1);
        let record = client.get(&id);
        assert_eq!(record.owner, user);
        assert_eq!(client.count(), 1);
    }
}
```

#### Template C: Voting System

Use this for: DAOs, polls, governance, community decisions.

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin,
    ProposalCount,
    Proposal(u64),
    Voted(u64, Address),
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub id: u64,
    pub creator: Address,
    pub title: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub active: bool,
}

#[contract]
pub struct Voting;

#[contractimpl]
impl Voting {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProposalCount, &0_u64);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
    }

    pub fn create_proposal(env: Env, creator: Address, title: String) -> u64 {
        creator.require_auth();
        let id: u64 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0) + 1;
        let proposal = Proposal { id, creator, title, yes_votes: 0, no_votes: 0, active: true };
        env.storage().persistent().set(&DataKey::Proposal(id), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &id);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
        id
    }

    pub fn vote(env: Env, voter: Address, proposal_id: u64, approve: bool) {
        voter.require_auth();
        // Check if already voted
        if env.storage().persistent().has(&DataKey::Voted(proposal_id, voter.clone())) {
            panic!("Already voted");
        }
        let mut p: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap();
        if !p.active { panic!("Proposal is closed"); }
        if approve { p.yes_votes += 1; } else { p.no_votes += 1; }
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &p);
        env.storage().persistent().set(&DataKey::Voted(proposal_id, voter), &true);
        env.events().publish((symbol_short!("vote"), proposal_id), approve);
    }

    pub fn get_proposal(env: Env, id: u64) -> Proposal {
        env.storage().persistent().get(&DataKey::Proposal(id)).unwrap()
    }

    pub fn close_proposal(env: Env, proposal_id: u64) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        let mut p: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap();
        p.active = false;
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &p);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_vote_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(Voting, ());
        let client = VotingClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let creator = Address::generate(&env);
        let id = client.create_proposal(&creator, &String::from_str(&env, "Add a gym"));

        let voter1 = Address::generate(&env);
        let voter2 = Address::generate(&env);
        client.vote(&voter1, &id, &true);
        client.vote(&voter2, &id, &false);

        let p = client.get_proposal(&id);
        assert_eq!(p.yes_votes, 1);
        assert_eq!(p.no_votes, 1);
    }
}
```

> **Need more patterns?** Check the [`modules/`](modules/) folder for CRUD, NFT, auth, and cross-contract examples.

### Step 3: Build & Test

```bash
cd contracts/my-project

# Build
stellar contract build

# Run tests
cargo test
```

If tests pass, your contract is ready to deploy!

### Key Concepts to Understand

Before moving on, make sure you understand these 4 things:

1. **`require_auth()`** — Every function that changes someone's data must verify they signed the transaction. Without this, anyone could steal anyone's tokens.

2. **Storage types** — Soroban has 3 storage types:
   - **Instance**: Config data (admin address, settings). Small, shared TTL.
   - **Persistent**: User data (balances, profiles). Per-key, survives long term.
   - **Temporary**: Disposable data (sessions, caches). Cheapest but deleted on expiry.

3. **TTL (Time to Live)** — All on-chain data expires eventually. You must extend its lifetime with `extend_ttl()`. If you forget, your data disappears after a few days.

4. **Events** — `env.events().publish(...)` emits an on-chain log that explorers and frontends can read. Use them for transparency.

> For deeper understanding, read [`modules/08-best-practices/`](modules/08-best-practices/) and [`modules/07-common-errors/`](modules/07-common-errors/).

---

## Part 4: Deploy to Stellar Testnet

> Goal: Get a **contract ID** and at least **1 transaction hash**. This is your proof of work.

### Step 1: Create & Fund Your Identity

```bash
# Generate a new identity and automatically fund it with test XLM
stellar keys generate student --network testnet --fund

# Check your address
stellar keys address student
```

> If `--fund` doesn't work, fund manually:
> ```bash
> curl "https://friendbot.stellar.org/?addr=$(stellar keys address student)"
> ```

### Step 2: Add Testnet Network (one-time)

```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Step 3: Deploy Your Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_project.wasm \
  --source-account student \
  --network testnet
```

**Output: `CABC123...`** — This is your **CONTRACT_ID**. Save it!

### Step 4: Invoke Your Contract

```bash
# Example: initialize your contract
stellar contract invoke \
  --id YOUR_CONTRACT_ID \
  --source-account student \
  --network testnet \
  -- initialize \
  --admin $(stellar keys address student) \
  --name "My Token" \
  --symbol "MTK"

# Example: mint tokens
stellar contract invoke \
  --id YOUR_CONTRACT_ID \
  --source-account student \
  --network testnet \
  -- mint \
  --to $(stellar keys address student) \
  --amount 1000
```

> **Important:** The `--` separator is required. Everything after it is your contract function arguments.

### Step 5: Verify on Stellar Expert

1. Open [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet)
2. Paste your **contract ID** in the search bar
3. You should see your contract and its transactions
4. Click on a transaction to get the **transaction hash**
5. **Copy the transaction URL** — you need it for submission

### Troubleshooting

| Error | Fix |
|-------|-----|
| `Account not found` | Fund your account: `curl "https://friendbot.stellar.org/?addr=$(stellar keys address student)"` |
| `tx_bad_seq` | Wait 5 seconds, try again |
| `simulation failed` | Your contract has a bug — run `cargo test` first |
| `op_underfunded` | Fund your account again with Friendbot |
| `wasm32 target not found` | Run `rustup target add wasm32-unknown-unknown` |

> Full error reference: [`modules/07-common-errors/`](modules/07-common-errors/)

---

## Part 5: Build Your Frontend (Optional)

> This is optional for submission but makes your project look much more complete. Use AI to generate the UI shell, then connect it to your contract.

### Option A: Use AI to Generate UI (fastest — 10 min)

Paste this prompt into [Claude](https://claude.ai), [ChatGPT](https://chatgpt.com), or [v0.dev](https://v0.dev):

```
Create a clean, modern web UI for a [YOUR PROJECT NAME] dApp on Stellar blockchain.

The app should have:
- A header with project name and "Connect Wallet" button
- Main section with [DESCRIBE YOUR MAIN FEATURE]
- A status panel showing wallet address
- Result area for transaction hash
- Dark theme, minimal design

Use plain HTML, CSS, and vanilla JavaScript in a single file.
Do NOT include blockchain logic — just the UI layout and event listeners.
```

### Option B: Use the Scaffold Starter

```bash
cd soroban-bootcamp/frontend
```

Use the pre-built HTML/JS template — see [`skills/frontend-dapp.md`](skills/frontend-dapp.md) for the complete pattern with wallet connection and contract interaction code.

### Connecting Wallet + Contract

The key code you need (add to your JS file):

```javascript
import * as StellarSdk from "@stellar/stellar-sdk";
import { isConnected, getAddress, signTransaction } from "@stellar/freighter-api";

const CONTRACT_ID = "YOUR_CONTRACT_ID_HERE"; // from deploy step
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";

// Connect wallet
async function connectWallet() {
  if (!(await isConnected())) return alert("Install Freighter!");
  const { address } = await getAddress();
  document.getElementById("wallet-address").textContent = address;
  return address;
}

// Call your contract
async function callContract(funcName, ...args) {
  const address = await connectWallet();
  const server = new StellarSdk.SorobanRpc.Server(RPC_URL);
  const account = await server.getAccount(address);
  const contract = new StellarSdk.Contract(CONTRACT_ID);

  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(funcName, ...args))
    .setTimeout(30)
    .build();

  const prepared = await server.prepareTransaction(tx);
  const { signedTxXdr } = await signTransaction(prepared.toXDR(), {
    networkPassphrase: NETWORK_PASSPHRASE,
  });

  const signed = StellarSdk.TransactionBuilder.fromXDR(signedTxXdr, NETWORK_PASSPHRASE);
  const result = await server.sendTransaction(signed);
  return result;
}
```

---

## Part 6: Submit Your Project

### What You Need

| Field | What to Submit | Example |
|-------|---------------|---------|
| **Developer info** | Telegram, Gmail, 1 line about you | @yourname \| you@gmail.com \| HCMUT CS Year 2 |
| **Project description** | 1 paragraph: problem + solution + why Stellar | "CampusCoin helps students earn loyalty tokens..." |
| **GitHub link** | Public repo with README | `https://github.com/yourname/campus-coin` |
| **Contract ID** | Your deployed contract on Testnet | `CABC123...` |
| **Transaction link** | At least 1 tx on Stellar Expert | `https://stellar.expert/explorer/testnet/tx/abc123...` |

### Step-by-Step

#### 1. Create a GitHub Repository

```bash
cd your-project-folder

git init
git add -A
git commit -m "Initial commit: [project name] on Stellar Testnet"

# Create repo on GitHub (github.com → New Repository)
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git
git branch -M main
git push -u origin main
```

> **Make the repo PUBLIC** so DevRel can review it.

#### 2. Write Your README

Use this template — copy it into your repo's `README.md`:

```markdown
# [Project Name]

## Problem
[One sentence: what pain point are you solving?]

## Solution
[One sentence: what did you build?]

## Why Stellar
[One sentence: which Stellar feature makes this possible/better?]

## Target User
[Who is this for?]

## Live Demo
- **Network**: Stellar Testnet
- **Contract ID**: `CABC123...`
- **Transaction**: https://stellar.expert/explorer/testnet/tx/YOUR_TX_HASH

## How to Run

1. Clone: `git clone https://github.com/yourname/project.git`
2. Build: `cd contracts/my-project && stellar contract build`
3. Test: `cargo test`
4. Deploy: `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/my_project.wasm --source-account student --network testnet`
5. Frontend: `cd frontend && npx serve .`

## Tech Stack
- Smart Contract: Rust / Soroban SDK v22
- Frontend: HTML / JavaScript / @stellar/stellar-sdk
- Wallet: Freighter
- Network: Stellar Testnet

## Team
- [Your Name] | [@telegram] | [email] | [university + year]
```

#### 3. Final Checklist Before Submitting

- [ ] GitHub repo is **PUBLIC**
- [ ] README includes contract ID and transaction link
- [ ] Transaction link opens correctly on Stellar Expert
- [ ] You can explain your code (DevRel may ask questions)
- [ ] No private keys or `.env` files in the repo

#### 4. Submit

The submission link will be shared by Rise In or DevRel during the session. Fill in all fields and submit before the deadline.

---

## After the Session

### Milestone System

| | Milestone 1 (Today) | Milestone 2 (2 weeks) |
|---|---|---|
| **Requirement** | GitHub repo + 1 tx hash on Testnet | Full demo + README + 2-min video |
| **Support** | DevRel in person | Community group + async mentorship |

### Milestone 2 — What To Do Next

If you want to take your project further:

1. **Polish your contract** — Add more functions, better error handling, tests
2. **Build a real frontend** — Use React/Next.js with [`scaffold/`](scaffold/) starter
3. **Record a 2-min demo video** — Show your dApp in action
4. **Write a proper README** — Explain the problem, solution, architecture

### Milestone 2 Rewards

- Top 3 projects featured on Rise In social media
- Best project: 1:1 mentorship from DevRel + intro to Stellar ecosystem contacts
- All completions: Rise In digital certificate
- Hackathon fast-track: priority for Rise In-sponsored hackathon teams
- SDF grant guidance: DevRel helps qualifying projects write Community Fund applications

### Keep Learning

| Resource | Link |
|----------|------|
| Stellar Developer Docs | [developers.stellar.org](https://developers.stellar.org) |
| Soroban Smart Contracts | [developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts) |
| Soroban Examples (34 contracts) | [`examples/soroban-examples/`](examples/soroban-examples/) in this repo |
| Code Modules & Best Practices | [`modules/`](modules/) in this repo |
| AI Development Skills | [`skills/`](skills/) in this repo |
| Scaffold Stellar (Full-Stack) | [scaffoldstellar.org](https://scaffoldstellar.org) |
| Stellar Expert (Explorer) | [stellar.expert](https://stellar.expert) |
| Stellar Laboratory | [laboratory.stellar.org](https://laboratory.stellar.org) |
| Stellar Community Fund | [communityfund.stellar.org](https://communityfund.stellar.org) |

### Community

After the session, you'll be invited to the builder community group. Here you can:
- Ask questions and get help from DevRel
- Share your progress
- Find teammates for hackathons
- Get feedback on your Milestone 2 submission

---

## Help & Resources

### I'm Stuck — Quick Fixes

| Problem | Solution |
|---------|----------|
| Can't install Rust | Try: `brew install rust` (Mac) or use [Gitpod](https://gitpod.io) |
| `stellar` command not found | Run: `cargo install --locked stellar-cli` |
| Build fails | Read the error — Rust errors tell you the exact line. Check [`modules/07-common-errors/`](modules/07-common-errors/) |
| Deploy fails | Fund your account with Friendbot, then retry |
| Freighter won't connect | Make sure you're on **Testnet** network in Freighter settings |
| Don't know what to build | Pick any idea from the list above. The simplest one is fine. |
| Running out of time | Deploy the starter contract as-is — 1 tx hash = 1 KPI |

### Using AI to Go Faster

You can use Claude Code, ChatGPT, or Copilot to accelerate your development. The [`skills/`](skills/) folder has reference files that make AI much more effective for Stellar/Soroban:

- Use [`skills/soroban-contract.md`](skills/soroban-contract.md) as context when asking AI to write contracts
- Use [`skills/frontend-dapp.md`](skills/frontend-dapp.md) when building your UI
- Use [`skills/full-stack-dapp.md`](skills/full-stack-dapp.md) for end-to-end guidance

> **AI policy:** Use AI as an accelerator, not a replacement. AI generates boilerplate. You write the logic. DevRel may ask you to explain your code — make sure you understand what every line does.

### Important Security Reminders

- **NEVER share your private/secret key** — not in code, not in chat, not on GitHub
- **Add `.env` to `.gitignore`** before your first commit
- **Use Testnet/Futurenet** for development — never deploy untested code to Mainnet
- **Private keys in code = leaked funds** — always use environment variables

---

> **You've got this.** Even if you've never touched blockchain before, you can deploy a working dApp today. The hardest part is starting — and you've already done that by reading this guide. See you at the session!

*Rise In x Stellar University Tour — March 2026*

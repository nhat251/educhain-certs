# SKILL: Full-Stack Stellar dApp in 1 Hour

> End-to-end guide: idea → contract → frontend → deploy → verify.
> Designed for hackathons and bootcamps. Maximum speed, production quality.

## The 1-Hour Plan

```
00:00-05:00  Project setup + scaffold
05:00-20:00  Smart contract (Rust)
20:00-30:00  Tests
30:00-35:00  Deploy to Testnet
35:00-50:00  Frontend (HTML/JS)
50:00-55:00  Connect frontend to contract
55:00-60:00  Verify on Stellar Expert + push to GitHub
```

## Phase 1: Project Setup (5 min)

### Option A: Scaffold Stellar (recommended)
```bash
cargo install --locked stellar-scaffold-cli
stellar scaffold init my-dapp
cd my-dapp
npm install
```

### Option B: Manual Setup
```bash
mkdir my-dapp && cd my-dapp
mkdir -p contracts/my-contract/src frontend

# Root Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["contracts/*"]

[workspace.dependencies]
soroban-sdk = "22"

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
EOF

# Contract Cargo.toml
cat > contracts/my-contract/Cargo.toml << 'EOF'
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
EOF
```

### Generate identity
```bash
stellar keys generate student --network testnet --fund
```

## Phase 2: Smart Contract (15 min)

Write your contract in `contracts/my-contract/src/lib.rs`.

### Template by Project Type

#### Type A: Token/Points System
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin, Name, Symbol, Decimals,
    Balance(Address), TotalSupply,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1, InsufficientBalance = 2, InvalidAmount = 3,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn __constructor(env: Env, admin: Address, name: String, symbol: String) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &7_u32);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        let bal: i128 = env.storage().persistent().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(bal + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), 29 * DAY, 30 * DAY);
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
        env.events().publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        from.require_auth();
        let from_bal: i128 = env.storage().persistent().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_bal < amount { return Err(Error::InsufficientBalance); }
        let to_bal: i128 = env.storage().persistent().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_bal + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(from), 29 * DAY, 30 * DAY);
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), 29 * DAY, 30 * DAY);
        env.events().publish((symbol_short!("transfer"), from, to), amount);
        Ok(())
    }

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn name(env: Env) -> String { env.storage().instance().get(&DataKey::Name).unwrap() }
    pub fn symbol(env: Env) -> String { env.storage().instance().get(&DataKey::Symbol).unwrap() }
    pub fn total_supply(env: Env) -> i128 { env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0) }
}
```

#### Type B: Registry/Certificate System
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey { Admin, Count, Record(u64) }

#[contracttype]
#[derive(Clone, Debug)]
pub struct Record { pub id: u64, pub owner: Address, pub data: String, pub timestamp: u64 }

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Count, &0_u64);
    }

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

    pub fn get(env: Env, id: u64) -> Record {
        env.storage().persistent().get(&DataKey::Record(id)).unwrap()
    }

    pub fn count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }
}
```

#### Type C: Voting/DAO System
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short, Map};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey { Admin, Proposal(u64), ProposalCount, Voted(u64, Address) }

#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal { pub id: u64, pub creator: Address, pub title: String, pub yes_votes: u32, pub no_votes: u32, pub active: bool }

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProposalCount, &0_u64);
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
        if env.storage().persistent().has(&DataKey::Voted(proposal_id, voter.clone())) {
            panic!("Already voted");
        }
        let mut p: Proposal = env.storage().persistent().get(&DataKey::Proposal(proposal_id)).unwrap();
        if !p.active { panic!("Proposal closed"); }
        if approve { p.yes_votes += 1; } else { p.no_votes += 1; }
        env.storage().persistent().set(&DataKey::Proposal(proposal_id), &p);
        env.storage().persistent().set(&DataKey::Voted(proposal_id, voter.clone()), &true);
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
```

## Phase 3: Test (10 min)

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, MyContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let id = env.register(MyContract, MyContractArgs::__constructor(&admin));
        (env, MyContractClient::new(&env, &id), admin)
    }

    #[test]
    fn test_happy_path() {
        let (env, client, admin) = setup();
        // Test your main flow
    }

    #[test]
    fn test_edge_case() {
        let (env, client, admin) = setup();
        // Test error cases
    }
}
```

```bash
cargo test
```

## Phase 4: Deploy (5 min)

```bash
stellar contract build
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source-account student \
  --network testnet \
  --alias my-contract

# Note the CONTRACT_ID output!

# Test invoke
stellar contract invoke \
  --id my-contract \
  --source-account student \
  --network testnet \
  -- your_function --arg value
```

## Phase 5: Frontend (15 min)

Use AI to generate the HTML shell, then wire up the SDK manually.
See `frontend-dapp.md` for the complete pattern.

Quick approach: copy the HTML template from `frontend-dapp.md`, customize the form fields,
and add contract calls in `app.js`.

## Phase 6: Verify + Push (5 min)

### Verify on Stellar Expert
```bash
echo "https://stellar.expert/explorer/testnet/contract/$(stellar contract id --alias my-contract --network testnet)"
```

### Push to GitHub
```bash
git init
git add -A
git commit -m "Initial dApp: [project name]"
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git
git branch -M main
git push -u origin main
```

### README Template
```markdown
# [Project Name]

## Problem
[One sentence]

## Solution
[One sentence about what your dApp does]

## Why Stellar
[Which Stellar feature you used: fast payments / tokenization / built-in DEX / low fees]

## Live Demo
- Network: Stellar Testnet
- Contract ID: `CAAA...`
- Transaction: [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/HASH)

## How to Run
1. Clone: `git clone [url]`
2. Build: `stellar contract build`
3. Deploy: `stellar contract deploy --wasm ... --network testnet`
4. Frontend: `cd frontend && npx serve .`

## Tech Stack
- Smart Contract: Rust / Soroban
- Frontend: HTML / JavaScript / @stellar/stellar-sdk
- Wallet: Freighter

## Team
- [Name] | [Telegram] | [Email] | [Background]
```

## Speed Tips

1. **Don't overthink the idea** — pick one from the 100 ideas list
2. **Start from a template** — copy Type A/B/C above, modify function names and fields
3. **Use AI for UI** — v0.dev or Claude generates the HTML shell in seconds
4. **Test minimally** — happy path + one error case is enough for a hackathon
5. **Deploy early** — get your contract ID before building the frontend
6. **One transaction = KPI** — even a simple `set_value` counts as a deployed dApp

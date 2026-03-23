# Module 12 — Cross-Contract Calls

> Call one contract from another — the building block of composable dApps.

## When to Use

- Your dApp has multiple contracts that need to interact
- You want to use an existing deployed contract (like a token)
- You're building a factory that deploys other contracts

## Pattern 1: Import WASM and Generate Client

The most common pattern. Import a compiled contract to auto-generate a typed client.

### Contract A (the one being called)
```rust
#[contract]
pub struct Calculator;

#[contractimpl]
impl Calculator {
    pub fn add(x: u32, y: u32) -> u32 { x + y }
    pub fn multiply(x: u32, y: u32) -> u32 { x * y }
}
```

### Contract B (the caller)
```rust
// Import Contract A's WASM — auto-generates a client
mod calculator {
    soroban_sdk::contractimport!(
        file = "../calculator/target/wasm32-unknown-unknown/release/calculator.wasm"
    );
}

#[contract]
pub struct MyApp;

#[contractimpl]
impl MyApp {
    pub fn compute(env: Env, calc_address: Address, x: u32, y: u32) -> u32 {
        // Create a client for Contract A
        let client = calculator::Client::new(&env, &calc_address);
        // Call Contract A's function
        let sum = client.add(&x, &y);
        let product = client.multiply(&x, &y);
        sum + product
    }
}
```

## Pattern 2: Token Interaction

The most practical use case — your contract interacts with a token contract.

```rust
use soroban_sdk::token;

pub fn deposit(env: Env, user: Address, token_address: Address, amount: i128) {
    user.require_auth();

    // Create a token client
    let token_client = token::Client::new(&env, &token_address);

    // Transfer tokens from user to this contract
    token_client.transfer(
        &user,
        &env.current_contract_address(),
        &amount,
    );

    // Update internal balance
    let balance: i128 = env.storage().persistent()
        .get(&DataKey::Balance(user.clone())).unwrap_or(0);
    env.storage().persistent()
        .set(&DataKey::Balance(user), &(balance + amount));
}

pub fn withdraw(env: Env, user: Address, token_address: Address, amount: i128) {
    user.require_auth();

    let balance: i128 = env.storage().persistent()
        .get(&DataKey::Balance(user.clone())).unwrap_or(0);
    if balance < amount { panic!("Insufficient balance"); }

    // Transfer tokens from contract back to user
    let token_client = token::Client::new(&env, &token_address);
    token_client.transfer(
        &env.current_contract_address(),
        &user,
        &amount,
    );

    env.storage().persistent()
        .set(&DataKey::Balance(user), &(balance - amount));
}
```

## Pattern 3: Factory (Deploy Other Contracts)

```rust
pub fn deploy_child(
    env: Env,
    admin: Address,
    wasm_hash: BytesN<32>,
    salt: BytesN<32>,
) -> Address {
    admin.require_auth();

    // Deploy a new contract
    let deployed_address = env.deployer()
        .with_address(admin, salt)
        .deploy_v2(wasm_hash, ());

    deployed_address
}
```

## Auth in Cross-Contract Calls

When Contract B calls Contract A on behalf of a user, the auth chain flows through:

```
User signs → Contract B (require_auth on user)
                └── calls Contract A (require_auth on user)
                    └── Soroban host verifies user authorized both calls
```

The user's signature covers the full call tree — they authorize all sub-invocations.

## Testing Cross-Contract

```rust
#[test]
fn test_cross_contract() {
    let env = Env::default();
    env.mock_all_auths();

    // Register both contracts
    let calc_id = env.register(Calculator, ());
    let app_id = env.register(MyApp, ());

    let app_client = MyAppClient::new(&env, &app_id);

    // Call app which internally calls calculator
    let result = app_client.compute(&calc_id, &3, &4);
    assert_eq!(result, 19); // (3+4) + (3*4) = 7 + 12 = 19
}
```

## See Also

- [examples/soroban-examples/cross_contract](../../examples/soroban-examples/cross_contract/)
- [examples/soroban-examples/deployer](../../examples/soroban-examples/deployer/)
- [examples/soroban-examples/deep_contract_auth](../../examples/soroban-examples/deep_contract_auth/)

# SKILL: Soroban Contract Testing

> Use this reference when writing tests for Soroban smart contracts.

## Test Setup

### Cargo.toml (dev-dependencies)
```toml
[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

### Test Module Structure
```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    // Tests go here
}
```

## Basic Test Pattern

```rust
#[test]
fn test_basic() {
    // 1. Create environment
    let env = Env::default();
    env.mock_all_auths();  // Auto-approve all auth checks

    // 2. Register contract
    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);

    // 3. Generate test addresses
    let user = Address::generate(&env);

    // 4. Call functions + assert
    let result = client.my_function(&user, &100_i128);
    assert_eq!(result, expected_value);
}
```

## Test with Constructor

```rust
#[test]
fn test_with_constructor() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);

    // Register with constructor args
    let contract_id = env.register(
        MyContract,
        MyContractArgs::__constructor(&admin),
    );
    let client = MyContractClient::new(&env, &contract_id);

    assert_eq!(client.admin(), admin);
}
```

## Auth Testing

### Verify Authorization
```rust
#[test]
fn test_auth() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    client.deposit(&user, &100);

    // Verify what was authorized
    assert_eq!(
        env.auths(),
        std::vec![(
            user.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id.clone(),
                    symbol_short!("deposit"),
                    (user.clone(), 100_i128).into_val(&env),
                )),
                sub_invocations: std::vec![],
            },
        )],
    );
}
```

### Test Unauthorized Access (should panic)
```rust
#[test]
#[should_panic(expected = "Not the owner")]
fn test_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);

    let id = client.create(&owner, &String::from_str(&env, "data"));

    // This should panic
    client.update(&attacker, &id, &String::from_str(&env, "hacked"));
}
```

### Test Error Returns
```rust
#[test]
fn test_error_result() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Use try_ prefix for functions that return Result
    let result = client.try_transfer(&user, &Address::generate(&env), &999999);
    assert!(result.is_err());
    // Or check specific error
    assert_eq!(result.unwrap_err(), Ok(ContractError::InsufficientBalance));
}
```

## TTL Testing

```rust
#[test]
fn test_ttl() {
    let env = Env::default();
    env.mock_all_auths();

    // Configure ledger settings for TTL
    env.ledger().with_mut(|li| {
        li.sequence_number = 100_000;
        li.min_persistent_entry_ttl = 500;
        li.min_temp_entry_ttl = 100;
        li.max_entry_ttl = 15_000;
    });

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    client.deposit(&user, &100);

    // Check TTL values
    env.as_contract(&contract_id, || {
        let ttl = env.storage().instance().get_ttl();
        assert!(ttl >= 5000, "Instance TTL should be extended");

        let bal_ttl = env.storage().persistent().get_ttl(&DataKey::Balance(user.clone()));
        assert!(bal_ttl >= 10000, "Balance TTL should be extended");
    });
}
```

## Event Testing

```rust
#[test]
fn test_events() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);

    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.transfer(&from, &to, &100);

    // Check emitted events
    let events = env.events().all();
    assert!(!events.is_empty(), "Should have emitted events");
}
```

## Cross-Contract Testing

```rust
#[test]
fn test_cross_contract() {
    let env = Env::default();
    env.mock_all_auths();

    // Register the dependency contract
    let token_id = env.register(TokenContract, ());
    let token_client = TokenContractClient::new(&env, &token_id);

    // Register the main contract
    let app_id = env.register(MyApp, ());
    let app_client = MyAppClient::new(&env, &app_id);

    let user = Address::generate(&env);

    // Setup: mint tokens to user
    token_client.mint(&user, &1000);

    // Test: app interacts with token contract
    app_client.deposit(&user, &token_id, &500);

    assert_eq!(token_client.balance(&user), 500);
    assert_eq!(app_client.get_balance(&user), 500);
}
```

## Test Helper Pattern

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    // Reusable setup function
    fn setup() -> (Env, MyContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(
            MyContract,
            MyContractArgs::__constructor(&admin),
        );
        let client = MyContractClient::new(&env, &contract_id);

        (env, client, admin)
    }

    #[test]
    fn test_a() {
        let (env, client, admin) = setup();
        // ...
    }

    #[test]
    fn test_b() {
        let (env, client, admin) = setup();
        // ...
    }
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run with output (see println/log messages)
cargo test -- --nocapture

# Run specific test
cargo test test_transfer

# Run tests for specific contract
cargo test --package my-contract
```

## Test Checklist

For every contract function, test:
- [ ] Happy path (normal usage)
- [ ] Edge cases (zero values, max values)
- [ ] Authorization (wrong user should fail)
- [ ] Error returns (invalid inputs)
- [ ] State consistency (balances add up after transfers)
- [ ] TTL extension (data doesn't expire)
- [ ] Events emitted (if applicable)

## Common Testing Mistakes

| Mistake | Fix |
|---------|-----|
| Forgetting `env.mock_all_auths()` | Auth checks will fail in tests |
| Forgetting `testutils` feature | `Address::generate` won't be available |
| Testing with `#[should_panic]` on Result functions | Use `try_` prefix + `assert!(result.is_err())` |
| Not testing both success and failure paths | Always test the unhappy path too |
| Sharing state between tests | Each test creates its own `Env` — they're isolated |

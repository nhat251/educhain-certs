# Module 05 — Authorization Patterns

> Secure your contracts. Every state-changing function needs proper auth.

## The Golden Rule

> **If a function changes someone's data or assets, it MUST call `require_auth()` on the affected address.**

## Pattern 1: Simple User Auth

The most common pattern. User signs to prove they approve this action.

```rust
pub fn deposit(env: Env, user: Address, amount: i128) {
    user.require_auth();  // User must sign this transaction
    // ... debit user's tokens, credit contract
}
```

**How it works:**
1. User builds a transaction that calls `deposit(user_address, 100)`
2. User signs the transaction with their private key
3. Soroban host verifies: "Did the address in the `user` argument actually sign?"
4. If yes → continue. If no → transaction fails.

## Pattern 2: Admin-Only Functions

Restrict sensitive operations to an admin address.

```rust
#[contracttype]
pub enum DataKey {
    Admin,
}

pub fn mint(env: Env, to: Address, amount: i128) {
    // Load admin from storage
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();  // Only admin can call this
    // ... mint tokens
}

pub fn set_admin(env: Env, new_admin: Address) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();  // Current admin must authorize
    env.storage().instance().set(&DataKey::Admin, &new_admin);
}
```

## Pattern 3: Owner-Based Access Control

Each resource has an owner. Only the owner can modify it.

```rust
pub fn update_profile(env: Env, caller: Address, user_id: u64, new_bio: String) {
    caller.require_auth();

    let profile: Profile = env.storage().persistent()
        .get(&DataKey::Profile(user_id)).unwrap();

    if profile.owner != caller {
        panic!("Not the owner");
    }

    // ... update profile
}
```

## Pattern 4: Role-Based Access

Multiple roles with different permissions.

```rust
#[contracttype]
pub enum Role {
    Admin,
    Moderator,
    User,
}

#[contracttype]
pub enum DataKey {
    Role(Address),  // address → role
    SuperAdmin,
}

pub fn set_role(env: Env, admin: Address, target: Address, role: Role) {
    let super_admin: Address = env.storage().instance()
        .get(&DataKey::SuperAdmin).unwrap();
    if admin != super_admin {
        panic!("Only super admin can assign roles");
    }
    admin.require_auth();
    env.storage().persistent().set(&DataKey::Role(target), &role);
}

pub fn moderate(env: Env, moderator: Address, content_id: u64) {
    moderator.require_auth();

    let role: Role = env.storage().persistent()
        .get(&DataKey::Role(moderator.clone()))
        .unwrap_or(Role::User);

    match role {
        Role::Admin | Role::Moderator => { /* allowed */ },
        Role::User => panic!("Not authorized"),
    }
    // ... moderate content
}
```

## Pattern 5: Multi-Party Authorization

Both parties must agree (used in swaps, escrows).

```rust
pub fn swap(
    env: Env,
    party_a: Address,
    party_b: Address,
    amount_a: i128,
    amount_b: i128,
) {
    // BOTH parties must authorize
    party_a.require_auth();
    party_b.require_auth();

    // ... execute swap
}
```

## Pattern 6: Constructor Auth (Initialize Once)

```rust
pub fn __constructor(env: Env, admin: Address) {
    env.storage().instance().set(&DataKey::Admin, &admin);
}
```
The constructor runs exactly once at deploy time. No re-initialization possible.

## Testing Auth

```rust
#[test]
fn test_auth() {
    let env = Env::default();
    env.mock_all_auths();  // Auto-approve all auth checks

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
                sub_invocations: std::vec![]
            }
        )]
    );
}
```

## Common Auth Mistakes

| Mistake | Impact | Fix |
|---------|--------|-----|
| Missing `require_auth()` | Anyone can call the function | Always auth state-changing functions |
| Auth on wrong address | Wrong person authorizes | Auth the address being affected |
| No admin check for mint | Anyone can create tokens | Load admin from storage, auth it |
| Hardcoded admin | Can't change admin later | Store admin in storage with `set_admin()` |

## See Also

- [soroban-examples/auth](../../examples/soroban-examples/auth/) — Official auth example
- [soroban-examples/account](../../examples/soroban-examples/account/) — Multi-sig account
- [soroban-examples/deep_contract_auth](../../examples/soroban-examples/deep_contract_auth/) — Nested auth

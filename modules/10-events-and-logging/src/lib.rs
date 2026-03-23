#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, symbol_short, log};

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalSupply,
}

// ============================================================
// CONTRACT — Demonstrates various event patterns
// ============================================================

#[contract]
pub struct EventDemoContract;

#[contractimpl]
impl EventDemoContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);

        // Event: contract initialized
        env.events().publish(
            (symbol_short!("init"),),
            admin,
        );
    }

    // --------------------------------------------------------
    // MINT — with mint event
    // --------------------------------------------------------
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));

        // Event: tokens minted
        // Topics: event type + recipient (both indexed/searchable)
        // Data: amount
        env.events().publish(
            (symbol_short!("mint"), to),
            amount,
        );

        log!(&env, "Minted {} tokens", amount);
    }

    // --------------------------------------------------------
    // TRANSFER — with transfer event
    // --------------------------------------------------------
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_bal: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone())).unwrap_or(0);
        let to_bal: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone())).unwrap_or(0);

        if from_bal < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_bal + amount));

        // Event: token transfer
        // 3 topics: event type, sender, receiver (all searchable)
        // Data: amount
        env.events().publish(
            (symbol_short!("transfer"), from.clone(), to.clone()),
            amount,
        );

        log!(&env, "Transfer {} from {:?} to {:?}", amount, from, to);
    }

    // --------------------------------------------------------
    // BURN — with burn event
    // --------------------------------------------------------
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone())).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(balance - amount));

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply - amount));

        // Event: tokens burned
        env.events().publish(
            (symbol_short!("burn"), from),
            amount,
        );
    }

    // --------------------------------------------------------
    // SET ADMIN — with admin change event
    // --------------------------------------------------------
    pub fn set_admin(env: Env, new_admin: Address) {
        let old_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        old_admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &new_admin);

        // Event: admin changed — includes both old and new in data
        env.events().publish(
            (symbol_short!("admin"),),
            (old_admin, new_admin),
        );
    }

    // --------------------------------------------------------
    // READ FUNCTIONS
    // --------------------------------------------------------

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }
}

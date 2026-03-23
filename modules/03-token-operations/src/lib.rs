#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String};

// ============================================================
// CONSTANTS
// ============================================================

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const BALANCE_TTL: u32 = 30 * DAY_IN_LEDGERS;
const BALANCE_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    Initialized,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TokenError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InsufficientBalance = 3,
    NotAdmin = 4,
    InvalidAmount = 5,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct SimpleToken;

#[contractimpl]
impl SimpleToken {
    // --------------------------------------------------------
    // INITIALIZE — Set up the token (call once)
    // --------------------------------------------------------
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), TokenError> {
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(TokenError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().set(&DataKey::Initialized, &true);

        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        Ok(())
    }

    // --------------------------------------------------------
    // MINT — Create new tokens (admin only)
    // --------------------------------------------------------
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)?;
        admin.require_auth();

        // Update balance
        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), BALANCE_THRESHOLD, BALANCE_TTL);

        // Update total supply
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);

        Ok(())
    }

    // --------------------------------------------------------
    // TRANSFER — Move tokens between accounts
    // --------------------------------------------------------
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        // Debit sender
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(from), BALANCE_THRESHOLD, BALANCE_TTL);

        // Credit receiver
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(to), BALANCE_THRESHOLD, BALANCE_TTL);

        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        Ok(())
    }

    // --------------------------------------------------------
    // BURN — Destroy tokens
    // --------------------------------------------------------
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(balance - amount));
        env.storage().persistent().extend_ttl(&DataKey::Balance(from), BALANCE_THRESHOLD, BALANCE_TTL);

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply - amount));
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);

        Ok(())
    }

    // --------------------------------------------------------
    // READ FUNCTIONS (no auth needed)
    // --------------------------------------------------------

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }

    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    // --------------------------------------------------------
    // ADMIN FUNCTIONS
    // --------------------------------------------------------

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), TokenError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)?;
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &new_admin);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        Ok(())
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, SimpleTokenClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(SimpleToken, ());
        let client = SimpleTokenClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(
            &admin,
            &String::from_str(&env, "Test Token"),
            &String::from_str(&env, "TST"),
            &7_u32,
        );

        (env, client, admin)
    }

    #[test]
    fn test_initialize() {
        let (_env, client, admin) = setup();
        assert_eq!(client.name(), String::from_str(&_env, "Test Token"));
        assert_eq!(client.symbol(), String::from_str(&_env, "TST"));
        assert_eq!(client.decimals(), 7);
        assert_eq!(client.admin(), admin);
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_mint_and_balance() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);

        client.mint(&user, &1_000_000_000); // 100 tokens
        assert_eq!(client.balance(&user), 1_000_000_000);
        assert_eq!(client.total_supply(), 1_000_000_000);
    }

    #[test]
    fn test_transfer() {
        let (env, client, _admin) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &1_000_000_000);
        client.transfer(&alice, &bob, &300_000_000);

        assert_eq!(client.balance(&alice), 700_000_000);
        assert_eq!(client.balance(&bob), 300_000_000);
    }

    #[test]
    fn test_burn() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);

        client.mint(&user, &1_000_000_000);
        client.burn(&user, &400_000_000);

        assert_eq!(client.balance(&user), 600_000_000);
        assert_eq!(client.total_supply(), 600_000_000);
    }

    #[test]
    fn test_insufficient_transfer() {
        let (env, client, _admin) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &100);
        let result = client.try_transfer(&alice, &bob, &200);
        assert!(result.is_err());
    }
}

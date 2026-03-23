#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// ============================================================
// CONSTANTS — Standard TTL values
// ============================================================

const DAY: u32 = 17280; // ~17,280 ledgers per day (5s per ledger)

// Instance: 7-day cycle (extend when < 6 days remaining)
const INSTANCE_TTL: u32 = 7 * DAY;
const INSTANCE_THRESHOLD: u32 = 6 * DAY;

// Persistent: 30-day cycle
const PERSISTENT_TTL: u32 = 30 * DAY;
const PERSISTENT_THRESHOLD: u32 = 29 * DAY;

// Temporary: 1-day cycle
const TEMP_TTL: u32 = DAY;
const TEMP_THRESHOLD: u32 = 0; // always extend

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
pub enum DataKey {
    // Instance storage keys
    Admin,
    Config,
    Counter,

    // Persistent storage keys
    Balance(Address),
    Profile(Address),

    // Temporary storage keys
    Session(Address),
    PriceCache,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub name: String,
    pub max_balance: i128,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct UserProfile {
    pub name: String,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct SessionData {
    pub login_time: u64,
    pub action_count: u32,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct StorageDemoContract;

#[contractimpl]
impl StorageDemoContract {
    // ========================================
    // INSTANCE STORAGE — Config & shared state
    // ========================================

    pub fn init(env: Env, admin: Address, app_name: String) {
        let config = AppConfig {
            name: app_name,
            max_balance: 1_000_000_000_000, // 100,000 tokens
            paused: false,
        };

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::Counter, &0_u64);

        // Extend all instance data at once
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    pub fn get_config(env: Env) -> AppConfig {
        env.storage().instance().get(&DataKey::Config).unwrap()
    }

    pub fn update_config(env: Env, config: AppConfig) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    // ========================================
    // PERSISTENT STORAGE — User data
    // ========================================

    pub fn set_balance(env: Env, user: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage().persistent().set(&DataKey::Balance(user.clone()), &amount);

        // Each persistent key has its own TTL
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(user),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );

        // Also refresh instance TTL
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn create_profile(env: Env, user: Address, name: String) {
        user.require_auth();

        let profile = UserProfile {
            name,
            created_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::Profile(user.clone()), &profile);
        env.storage().persistent().extend_ttl(
            &DataKey::Profile(user),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );
    }

    pub fn get_profile(env: Env, user: Address) -> UserProfile {
        env.storage().persistent()
            .get(&DataKey::Profile(user))
            .unwrap_or_else(|| panic!("Profile not found"))
    }

    // ========================================
    // TEMPORARY STORAGE — Disposable data
    // ========================================

    pub fn start_session(env: Env, user: Address) {
        user.require_auth();

        let session = SessionData {
            login_time: env.ledger().timestamp(),
            action_count: 0,
        };

        env.storage().temporary().set(&DataKey::Session(user.clone()), &session);

        // Temporary data expires and is GONE FOREVER
        env.storage().temporary().extend_ttl(
            &DataKey::Session(user),
            TEMP_THRESHOLD,
            TEMP_TTL,
        );
    }

    pub fn get_session(env: Env, user: Address) -> SessionData {
        env.storage().temporary()
            .get(&DataKey::Session(user))
            .unwrap_or_else(|| panic!("No active session"))
    }

    pub fn has_session(env: Env, user: Address) -> bool {
        env.storage().temporary().has(&DataKey::Session(user))
    }

    pub fn set_price_cache(env: Env, price: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // Price cache in temporary storage — cheap, OK to lose
        env.storage().temporary().set(&DataKey::PriceCache, &price);
        env.storage().temporary().extend_ttl(&DataKey::PriceCache, TEMP_THRESHOLD, TEMP_TTL);
    }

    pub fn get_price_cache(env: Env) -> i128 {
        env.storage().temporary()
            .get(&DataKey::PriceCache)
            .unwrap_or(0)
    }

    // ========================================
    // UTILITY — Check and remove data
    // ========================================

    pub fn has_balance(env: Env, user: Address) -> bool {
        env.storage().persistent().has(&DataKey::Balance(user))
    }

    pub fn remove_profile(env: Env, user: Address) {
        user.require_auth();
        env.storage().persistent().remove(&DataKey::Profile(user));
    }
}

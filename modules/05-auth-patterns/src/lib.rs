#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String};

// ============================================================
// This module demonstrates multiple auth patterns in one contract.
// In real projects, pick the pattern that fits your use case.
// ============================================================

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    User,
}

#[contracttype]
pub enum DataKey {
    Admin,                // Super admin address
    Role(Address),        // Per-address role
    Balance(Address),     // Token balance
    Profile(Address),     // User profile
    Paused,               // Circuit breaker
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Profile {
    pub owner: Address,
    pub name: String,
    pub bio: String,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AuthError {
    NotAdmin = 1,
    NotOwner = 2,
    NotAuthorized = 3,
    ContractPaused = 4,
    InsufficientBalance = 5,
}

#[contract]
pub struct AuthDemoContract;

#[contractimpl]
impl AuthDemoContract {
    // ========================================
    // PATTERN 1: Constructor sets admin
    // ========================================
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Role(admin), &Role::Admin);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    // ========================================
    // PATTERN 2: Admin-only function
    // ========================================
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), AuthError> {
        Self::require_not_paused(env.clone())?;

        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let balance: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
        Ok(())
    }

    // ========================================
    // PATTERN 3: User auth (sender signs)
    // ========================================
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), AuthError> {
        Self::require_not_paused(env.clone())?;
        from.require_auth();

        let from_bal: i128 = env.storage().persistent()
            .get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_bal < amount {
            return Err(AuthError::InsufficientBalance);
        }

        let to_bal: i128 = env.storage().persistent()
            .get(&DataKey::Balance(to.clone())).unwrap_or(0);

        env.storage().persistent().set(&DataKey::Balance(from), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_bal + amount));
        Ok(())
    }

    // ========================================
    // PATTERN 4: Owner-only access
    // ========================================
    pub fn create_profile(env: Env, caller: Address, name: String, bio: String) {
        caller.require_auth();
        let profile = Profile { owner: caller.clone(), name, bio };
        env.storage().persistent().set(&DataKey::Profile(caller), &profile);
    }

    pub fn update_profile(env: Env, caller: Address, name: String, bio: String) -> Result<(), AuthError> {
        caller.require_auth();

        let profile: Profile = env.storage().persistent()
            .get(&DataKey::Profile(caller.clone()))
            .ok_or(AuthError::NotOwner)?;

        if profile.owner != caller {
            return Err(AuthError::NotOwner);
        }

        let updated = Profile { owner: caller.clone(), name, bio };
        env.storage().persistent().set(&DataKey::Profile(caller), &updated);
        Ok(())
    }

    // ========================================
    // PATTERN 5: Role-based access
    // ========================================
    pub fn assign_role(env: Env, target: Address, role: Role) -> Result<(), AuthError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Role(target), &role);
        Ok(())
    }

    pub fn admin_action(env: Env, caller: Address) -> Result<(), AuthError> {
        caller.require_auth();
        let role: Role = env.storage().persistent()
            .get(&DataKey::Role(caller)).unwrap_or(Role::User);
        match role {
            Role::Admin => Ok(()),
            _ => Err(AuthError::NotAuthorized),
        }
    }

    pub fn moderator_action(env: Env, caller: Address) -> Result<(), AuthError> {
        caller.require_auth();
        let role: Role = env.storage().persistent()
            .get(&DataKey::Role(caller)).unwrap_or(Role::User);
        match role {
            Role::Admin | Role::Moderator => Ok(()),
            _ => Err(AuthError::NotAuthorized),
        }
    }

    // ========================================
    // PATTERN 6: Multi-party auth (swap)
    // ========================================
    pub fn swap(
        env: Env,
        party_a: Address,
        party_b: Address,
        amount_a: i128,
        amount_b: i128,
    ) -> Result<(), AuthError> {
        Self::require_not_paused(env.clone())?;

        // Both parties must authorize
        party_a.require_auth();
        party_b.require_auth();

        let bal_a: i128 = env.storage().persistent()
            .get(&DataKey::Balance(party_a.clone())).unwrap_or(0);
        let bal_b: i128 = env.storage().persistent()
            .get(&DataKey::Balance(party_b.clone())).unwrap_or(0);

        if bal_a < amount_a || bal_b < amount_b {
            return Err(AuthError::InsufficientBalance);
        }

        env.storage().persistent().set(&DataKey::Balance(party_a.clone()), &(bal_a - amount_a + amount_b));
        env.storage().persistent().set(&DataKey::Balance(party_b.clone()), &(bal_b - amount_b + amount_a));
        Ok(())
    }

    // ========================================
    // PATTERN 7: Circuit breaker (pause)
    // ========================================
    pub fn pause(env: Env) -> Result<(), AuthError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), AuthError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    // ========================================
    // HELPERS
    // ========================================
    fn require_not_paused(env: Env) -> Result<(), AuthError> {
        let paused: bool = env.storage().instance().get(&DataKey::Paused).unwrap_or(false);
        if paused {
            Err(AuthError::ContractPaused)
        } else {
            Ok(())
        }
    }

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn get_role(env: Env, addr: Address) -> Role {
        env.storage().persistent().get(&DataKey::Role(addr)).unwrap_or(Role::User)
    }
}

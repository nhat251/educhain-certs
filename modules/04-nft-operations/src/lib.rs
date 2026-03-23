#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String};

// ============================================================
// CONSTANTS
// ============================================================

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const NFT_TTL: u32 = 90 * DAY_IN_LEDGERS;
const NFT_THRESHOLD: u32 = 89 * DAY_IN_LEDGERS;

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    TokenCount,         // Total NFTs minted
    Owner(u64),         // token_id → owner Address
    Metadata(u64),      // token_id → NftMetadata
    Initialized,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub uri: String,
    pub created_at: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum NftError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotOwner = 3,
    NotAdmin = 4,
    TokenNotFound = 5,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct NftContract;

#[contractimpl]
impl NftContract {
    // --------------------------------------------------------
    // INITIALIZE
    // --------------------------------------------------------
    pub fn initialize(env: Env, admin: Address) -> Result<(), NftError> {
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(NftError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenCount, &0_u64);
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        Ok(())
    }

    // --------------------------------------------------------
    // MINT — Create a new NFT (admin only)
    // --------------------------------------------------------
    pub fn mint(
        env: Env,
        to: Address,
        name: String,
        description: String,
        uri: String,
    ) -> Result<u64, NftError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .ok_or(NftError::NotInitialized)?;
        admin.require_auth();

        // Auto-increment token ID
        let token_id: u64 = env.storage().instance()
            .get(&DataKey::TokenCount).unwrap_or(0) + 1;

        let metadata = NftMetadata {
            name,
            description,
            uri,
            created_at: env.ledger().timestamp(),
        };

        // Store owner and metadata
        env.storage().persistent().set(&DataKey::Owner(token_id), &to);
        env.storage().persistent().set(&DataKey::Metadata(token_id), &metadata);

        // Update count
        env.storage().instance().set(&DataKey::TokenCount, &token_id);

        // Extend TTLs
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        env.storage().persistent().extend_ttl(&DataKey::Owner(token_id), NFT_THRESHOLD, NFT_TTL);
        env.storage().persistent().extend_ttl(&DataKey::Metadata(token_id), NFT_THRESHOLD, NFT_TTL);

        Ok(token_id)
    }

    // --------------------------------------------------------
    // TRANSFER — Move NFT to another address (owner only)
    // --------------------------------------------------------
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) -> Result<(), NftError> {
        from.require_auth();

        let owner: Address = env.storage().persistent()
            .get(&DataKey::Owner(token_id))
            .ok_or(NftError::TokenNotFound)?;

        if owner != from {
            return Err(NftError::NotOwner);
        }

        env.storage().persistent().set(&DataKey::Owner(token_id), &to);
        env.storage().persistent().extend_ttl(&DataKey::Owner(token_id), NFT_THRESHOLD, NFT_TTL);

        Ok(())
    }

    // --------------------------------------------------------
    // BURN — Destroy an NFT (owner only)
    // --------------------------------------------------------
    pub fn burn(env: Env, caller: Address, token_id: u64) -> Result<(), NftError> {
        caller.require_auth();

        let owner: Address = env.storage().persistent()
            .get(&DataKey::Owner(token_id))
            .ok_or(NftError::TokenNotFound)?;

        if owner != caller {
            return Err(NftError::NotOwner);
        }

        env.storage().persistent().remove(&DataKey::Owner(token_id));
        env.storage().persistent().remove(&DataKey::Metadata(token_id));

        Ok(())
    }

    // --------------------------------------------------------
    // READ FUNCTIONS
    // --------------------------------------------------------

    pub fn owner_of(env: Env, token_id: u64) -> Result<Address, NftError> {
        env.storage().persistent()
            .get(&DataKey::Owner(token_id))
            .ok_or(NftError::TokenNotFound)
    }

    pub fn metadata(env: Env, token_id: u64) -> Result<NftMetadata, NftError> {
        env.storage().persistent()
            .get(&DataKey::Metadata(token_id))
            .ok_or(NftError::TokenNotFound)
    }

    pub fn total_minted(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::TokenCount)
            .unwrap_or(0)
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, NftContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(NftContract, ());
        let client = NftContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        (env, client, admin)
    }

    #[test]
    fn test_mint() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);

        let token_id = client.mint(
            &user,
            &String::from_str(&env, "Art #1"),
            &String::from_str(&env, "Beautiful art"),
            &String::from_str(&env, "https://example.com/1.json"),
        );

        assert_eq!(token_id, 1);
        assert_eq!(client.owner_of(&token_id), user);
        assert_eq!(client.total_minted(), 1);
    }

    #[test]
    fn test_transfer() {
        let (env, client, _admin) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        let token_id = client.mint(
            &alice,
            &String::from_str(&env, "Art #1"),
            &String::from_str(&env, "Art"),
            &String::from_str(&env, "https://example.com/1"),
        );

        client.transfer(&alice, &bob, &token_id);
        assert_eq!(client.owner_of(&token_id), bob);
    }

    #[test]
    fn test_burn() {
        let (env, client, _admin) = setup();
        let user = Address::generate(&env);

        let token_id = client.mint(
            &user,
            &String::from_str(&env, "Art #1"),
            &String::from_str(&env, "Art"),
            &String::from_str(&env, "https://example.com/1"),
        );

        client.burn(&user, &token_id);
        let result = client.try_owner_of(&token_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_wrong_owner() {
        let (env, client, _admin) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let charlie = Address::generate(&env);

        let token_id = client.mint(
            &alice,
            &String::from_str(&env, "Art #1"),
            &String::from_str(&env, "Art"),
            &String::from_str(&env, "https://example.com/1"),
        );

        // Bob tries to transfer Alice's NFT — should fail
        let result = client.try_transfer(&bob, &charlie, &token_id);
        assert!(result.is_err());
    }
}

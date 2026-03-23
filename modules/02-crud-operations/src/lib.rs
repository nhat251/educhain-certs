#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// ============================================================
// DATA TYPES
// ============================================================

/// Storage key enum — each variant maps to a different storage entry
#[contracttype]
pub enum DataKey {
    Record(u64),  // Individual record keyed by ID
    Counter,      // Auto-increment counter for IDs
}

/// A single record stored on-chain
#[contracttype]
#[derive(Clone, Debug)]
pub struct Record {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub content: String,
    pub created_at: u64,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct CrudContract;

#[contractimpl]
impl CrudContract {
    // --------------------------------------------------------
    // CREATE — Store a new record, return its ID
    // --------------------------------------------------------
    pub fn create(env: Env, caller: Address, title: String, content: String) -> u64 {
        caller.require_auth();

        // Auto-increment ID
        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0)
            + 1;

        let record = Record {
            id,
            owner: caller,
            title,
            content,
            created_at: env.ledger().timestamp(),
        };

        // Store the record in persistent storage
        env.storage()
            .persistent()
            .set(&DataKey::Record(id), &record);

        // Update the counter
        env.storage().instance().set(&DataKey::Counter, &id);

        // Extend TTL so data doesn't expire
        env.storage()
            .instance()
            .extend_ttl(17280, 17280 * 7); // threshold: 1 day, extend: 7 days
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Record(id), 17280, 17280 * 30); // 30 days

        id
    }

    // --------------------------------------------------------
    // READ — Get a record by ID (no auth needed — public read)
    // --------------------------------------------------------
    pub fn read(env: Env, id: u64) -> Record {
        env.storage()
            .persistent()
            .get(&DataKey::Record(id))
            .unwrap_or_else(|| panic!("Record {} not found", id))
    }

    // --------------------------------------------------------
    // UPDATE — Modify a record (only owner can update)
    // --------------------------------------------------------
    pub fn update(env: Env, caller: Address, id: u64, title: String, content: String) {
        caller.require_auth();

        let mut record: Record = Self::read(env.clone(), id);

        // Only the owner can update
        if record.owner != caller {
            panic!("Not the owner");
        }

        record.title = title;
        record.content = content;

        env.storage()
            .persistent()
            .set(&DataKey::Record(id), &record);

        // Extend TTL on update
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Record(id), 17280, 17280 * 30);
    }

    // --------------------------------------------------------
    // DELETE — Remove a record (only owner can delete)
    // --------------------------------------------------------
    pub fn delete(env: Env, caller: Address, id: u64) {
        caller.require_auth();

        let record: Record = Self::read(env.clone(), id);

        if record.owner != caller {
            panic!("Not the owner");
        }

        env.storage().persistent().remove(&DataKey::Record(id));
    }

    // --------------------------------------------------------
    // LIST — Get the total count of records created
    // --------------------------------------------------------
    pub fn count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0)
    }

    // --------------------------------------------------------
    // EXISTS — Check if a record exists
    // --------------------------------------------------------
    pub fn exists(env: Env, id: u64) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Record(id))
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_create_and_read() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrudContract, ());
        let client = CrudContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let title = String::from_str(&env, "Hello");
        let content = String::from_str(&env, "World");

        // Create
        let id = client.create(&user, &title, &content);
        assert_eq!(id, 1);

        // Read
        let record = client.read(&id);
        assert_eq!(record.id, 1);
        assert_eq!(record.owner, user);
        assert_eq!(record.title, title);
        assert_eq!(record.content, content);
    }

    #[test]
    fn test_update() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrudContract, ());
        let client = CrudContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let id = client.create(
            &user,
            &String::from_str(&env, "Old Title"),
            &String::from_str(&env, "Old Content"),
        );

        client.update(
            &user,
            &id,
            &String::from_str(&env, "New Title"),
            &String::from_str(&env, "New Content"),
        );

        let record = client.read(&id);
        assert_eq!(record.title, String::from_str(&env, "New Title"));
        assert_eq!(record.content, String::from_str(&env, "New Content"));
    }

    #[test]
    fn test_delete() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrudContract, ());
        let client = CrudContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let id = client.create(
            &user,
            &String::from_str(&env, "To Delete"),
            &String::from_str(&env, "Content"),
        );

        assert!(client.exists(&id));
        client.delete(&user, &id);
        assert!(!client.exists(&id));
    }

    #[test]
    fn test_count() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrudContract, ());
        let client = CrudContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        assert_eq!(client.count(), 0);

        client.create(&user, &String::from_str(&env, "A"), &String::from_str(&env, "1"));
        assert_eq!(client.count(), 1);

        client.create(&user, &String::from_str(&env, "B"), &String::from_str(&env, "2"));
        assert_eq!(client.count(), 2);
    }

    #[test]
    #[should_panic(expected = "Not the owner")]
    fn test_update_wrong_owner() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrudContract, ());
        let client = CrudContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let other = Address::generate(&env);

        let id = client.create(
            &owner,
            &String::from_str(&env, "Title"),
            &String::from_str(&env, "Content"),
        );

        // This should panic — other is not the owner
        client.update(
            &other,
            &id,
            &String::from_str(&env, "Hacked"),
            &String::from_str(&env, "Hacked"),
        );
    }
}

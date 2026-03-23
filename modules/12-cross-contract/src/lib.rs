#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

// ============================================================
// This module demonstrates a simple escrow contract that
// interacts with a token contract (cross-contract call).
//
// Flow:
// 1. Seller creates an escrow (locks tokens)
// 2. Buyer deposits payment
// 3. Seller releases tokens to buyer, gets payment
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    Escrow(u64),
    EscrowCount,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    pub id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub token: Address,         // Token contract address
    pub amount: i128,           // Amount of tokens in escrow
    pub payment_token: Address, // Payment token (e.g., USDC)
    pub price: i128,            // Price in payment token
    pub funded: bool,           // Has buyer paid?
    pub completed: bool,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::EscrowCount, &0_u64);
    }

    // --------------------------------------------------------
    // CREATE ESCROW — Seller locks tokens into the contract
    // --------------------------------------------------------
    pub fn create_escrow(
        env: Env,
        seller: Address,
        buyer: Address,
        token_addr: Address,
        amount: i128,
        payment_token_addr: Address,
        price: i128,
    ) -> u64 {
        seller.require_auth();

        // Cross-contract call: transfer seller's tokens to this contract
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(
            &seller,
            &env.current_contract_address(),
            &amount,
        );

        let id: u64 = env.storage().instance()
            .get(&DataKey::EscrowCount).unwrap_or(0) + 1;

        let escrow = Escrow {
            id,
            seller: seller.clone(),
            buyer: buyer.clone(),
            token: token_addr,
            amount,
            payment_token: payment_token_addr,
            price,
            funded: false,
            completed: false,
        };

        env.storage().persistent().set(&DataKey::Escrow(id), &escrow);
        env.storage().instance().set(&DataKey::EscrowCount, &id);

        id
    }

    // --------------------------------------------------------
    // FUND ESCROW — Buyer deposits payment
    // --------------------------------------------------------
    pub fn fund_escrow(env: Env, buyer: Address, escrow_id: u64) {
        buyer.require_auth();

        let mut escrow: Escrow = env.storage().persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("Escrow not found"));

        if escrow.buyer != buyer { panic!("Not the buyer"); }
        if escrow.funded { panic!("Already funded"); }

        // Cross-contract call: transfer buyer's payment to this contract
        let payment_client = token::Client::new(&env, &escrow.payment_token);
        payment_client.transfer(
            &buyer,
            &env.current_contract_address(),
            &escrow.price,
        );

        escrow.funded = true;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // --------------------------------------------------------
    // COMPLETE — Release tokens to buyer, payment to seller
    // --------------------------------------------------------
    pub fn complete(env: Env, escrow_id: u64) {
        let escrow: Escrow = env.storage().persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("Escrow not found"));

        if !escrow.funded { panic!("Not funded yet"); }
        if escrow.completed { panic!("Already completed"); }

        // Either party can trigger completion once funded
        // (In production, add more conditions)

        let contract_addr = env.current_contract_address();

        // Cross-contract: send tokens to buyer
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&contract_addr, &escrow.buyer, &escrow.amount);

        // Cross-contract: send payment to seller
        let payment_client = token::Client::new(&env, &escrow.payment_token);
        payment_client.transfer(&contract_addr, &escrow.seller, &escrow.price);

        let mut completed_escrow = escrow;
        completed_escrow.completed = true;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &completed_escrow);
    }

    // --------------------------------------------------------
    // READ
    // --------------------------------------------------------
    pub fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        env.storage().persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("Escrow not found"))
    }
}

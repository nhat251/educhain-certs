#![no_std]

use core::option::Option;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const CERT_TTL: u32 = 365 * DAY_IN_LEDGERS;
const CERT_THRESHOLD: u32 = 364 * DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextCertId,
    Cert(u64),
    WalletCerts(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    pub id: u64,
    pub student: Address,
    pub cert_hash: String,
    pub course_name: String,
    pub issuer_name: String,
    pub issued_at: u64,
    pub revoked: bool,
    pub revoked_reason: Option<String>,
    pub revoked_at: Option<u64>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CertError {
    NotAdmin = 1,
    CertNotFound = 2,
    AlreadyRevoked = 3,
    SoulboundNonTransferable = 4,
}

#[contract]
pub struct EduChainCerts;

#[contractimpl]
impl EduChainCerts {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextCertId, &0_u64);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn issue_cert(
        env: Env,
        issuer: Address,
        student: Address,
        cert_hash: String,
        course_name: String,
        issuer_name: String,
    ) -> Result<u64, CertError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if issuer != admin {
            return Err(CertError::NotAdmin);
        }
        issuer.require_auth();

        let cert_id: u64 = env.storage().instance().get(&DataKey::NextCertId).unwrap_or(0) + 1;
        let cert = Certificate {
            id: cert_id,
            student: student.clone(),
            cert_hash,
            course_name,
            issuer_name,
            issued_at: env.ledger().timestamp(),
            revoked: false,
            revoked_reason: Option::None,
            revoked_at: Option::None,
        };

        env.storage().persistent().set(&DataKey::Cert(cert_id), &cert);

        let mut ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::WalletCerts(student.clone()))
            .unwrap_or(Vec::new(&env));
        ids.push_back(cert_id);
        env.storage()
            .persistent()
            .set(&DataKey::WalletCerts(student.clone()), &ids);

        env.storage().instance().set(&DataKey::NextCertId, &cert_id);

        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Cert(cert_id), CERT_THRESHOLD, CERT_TTL);
        env.storage().persistent().extend_ttl(
            &DataKey::WalletCerts(student.clone()),
            CERT_THRESHOLD,
            CERT_TTL,
        );
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);

        env.events()
            .publish((symbol_short!("issued"), student, cert_id), true);

        Ok(cert_id)
    }

    pub fn verify_by_id(env: Env, cert_id: u64) -> Result<Certificate, CertError> {
        env.storage()
            .persistent()
            .get(&DataKey::Cert(cert_id))
            .ok_or(CertError::CertNotFound)
    }

    pub fn certs_of_wallet(env: Env, student: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::WalletCerts(student))
            .unwrap_or(Vec::new(&env))
    }

    pub fn revoke(
        env: Env,
        issuer: Address,
        cert_id: u64,
        reason: String,
    ) -> Result<(), CertError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if issuer != admin {
            return Err(CertError::NotAdmin);
        }
        issuer.require_auth();

        let mut cert: Certificate = env
            .storage()
            .persistent()
            .get(&DataKey::Cert(cert_id))
            .ok_or(CertError::CertNotFound)?;

        if cert.revoked {
            return Err(CertError::AlreadyRevoked);
        }

        cert.revoked = true;
        cert.revoked_reason = Option::Some(reason);
        cert.revoked_at = Option::Some(env.ledger().timestamp());

        env.storage().persistent().set(&DataKey::Cert(cert_id), &cert);
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Cert(cert_id), CERT_THRESHOLD, CERT_TTL);

        env.events()
            .publish((symbol_short!("revoked"), cert.student, cert_id), true);

        Ok(())
    }

    pub fn transfer(
        _env: Env,
        _from: Address,
        _to: Address,
        _cert_id: u64,
    ) -> Result<(), CertError> {
        Err(CertError::SoulboundNonTransferable)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, EduChainCertsClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let student = Address::generate(&env);

        let contract_id = env.register(EduChainCerts, EduChainCertsArgs::__constructor(&admin));
        let client = EduChainCertsClient::new(&env, &contract_id);

        (env, client, admin, student)
    }

    #[test]
    fn issue_and_verify_by_id() {
        let (env, client, admin, student) = setup();

        let cert_id = client.issue_cert(
            &admin,
            &student,
            &String::from_str(&env, "QmHash123"),
            &String::from_str(&env, "Soroban Bootcamp"),
            &String::from_str(&env, "EduChain University"),
        );

        assert_eq!(cert_id, 1);

        let cert = client.verify_by_id(&cert_id);
        assert_eq!(cert.student, student);
        assert_eq!(cert.revoked, false);
    }

    #[test]
    fn issue_fails_for_non_admin() {
        let (env, client, _admin, student) = setup();
        let attacker = Address::generate(&env);

        let result = client.try_issue_cert(
            &attacker,
            &student,
            &String::from_str(&env, "QmFake"),
            &String::from_str(&env, "Fake Course"),
            &String::from_str(&env, "Fake Issuer"),
        );

        assert_eq!(result, Err(Ok(CertError::NotAdmin)));
    }

    #[test]
    fn verify_by_wallet_returns_ids() {
        let (env, client, admin, student) = setup();

        let cert_id = client.issue_cert(
            &admin,
            &student,
            &String::from_str(&env, "QmHashABC"),
            &String::from_str(&env, "Blockchain 101"),
            &String::from_str(&env, "EduChain University"),
        );

        let ids = client.certs_of_wallet(&student);
        assert_eq!(ids.len(), 1);
        assert_eq!(ids.get(0).unwrap(), cert_id);
    }

    #[test]
    fn revoke_updates_certificate_status() {
        let (env, client, admin, student) = setup();

        let cert_id = client.issue_cert(
            &admin,
            &student,
            &String::from_str(&env, "QmHashREVOKE"),
            &String::from_str(&env, "Security Basics"),
            &String::from_str(&env, "EduChain University"),
        );

        client.revoke(&admin, &cert_id, &String::from_str(&env, "Academic misconduct"));

        let cert = client.verify_by_id(&cert_id);
        assert_eq!(cert.revoked, true);
        assert!(cert.revoked_reason.is_some());
    }

    #[test]
    fn transfer_is_always_blocked_for_soulbound() {
        let (env, client, admin, student) = setup();
        let other = Address::generate(&env);

        let cert_id = client.issue_cert(
            &admin,
            &student,
            &String::from_str(&env, "QmHashSoul"),
            &String::from_str(&env, "Web3 Fundamentals"),
            &String::from_str(&env, "EduChain University"),
        );

        let result = client.try_transfer(&student, &other, &cert_id);
        assert_eq!(result, Err(Ok(CertError::SoulboundNonTransferable)));
    }
}

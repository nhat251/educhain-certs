#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    pub id: u64,
    pub owner: Address,
    pub course_name: String,
    pub issuer: Address,
    pub issued_at: u64,
    pub revoked: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    NextCertificateId,
    Certificate(u64),
    OwnerCertificates(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CertError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    CertificateNotFound = 4,
    NotTransferable = 5,
}

#[contract]
pub struct EduChainsCertsContract;

#[contractimpl]
impl EduChainsCertsContract {
    // One-time setup: store the institution admin address.
    pub fn init(env: Env, admin: Address) -> Result<(), CertError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(CertError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextCertificateId, &1_u64);
        Ok(())
    }

    // Admin issues a new non-transferable certificate to a student wallet.
    pub fn issue_certificate(
        env: Env,
        caller: Address,
        owner: Address,
        course_name: String,
        issued_at: u64,
    ) -> Result<u64, CertError> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;

        let id = env
            .storage()
            .instance()
            .get::<DataKey, u64>(&DataKey::NextCertificateId)
            .ok_or(CertError::NotInitialized)?;

        let cert = Certificate {
            id,
            owner: owner.clone(),
            course_name,
            issuer: caller,
            issued_at,
            revoked: false,
        };

        env.storage().persistent().set(&DataKey::Certificate(id), &cert);

        let mut owner_ids = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<u64>>(&DataKey::OwnerCertificates(owner.clone()))
            .unwrap_or(Vec::new(&env));
        owner_ids.push_back(id);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerCertificates(owner), &owner_ids);

        env.storage()
            .instance()
            .set(&DataKey::NextCertificateId, &(id + 1));

        Ok(id)
    }

    // Admin can revoke a certificate (soft delete by status flag).
    pub fn revoke_certificate(env: Env, caller: Address, certificate_id: u64) -> Result<(), CertError> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;

        let mut cert: Certificate = env
            .storage()
            .persistent()
            .get(&DataKey::Certificate(certificate_id))
            .ok_or(CertError::CertificateNotFound)?;

        cert.revoked = true;
        env.storage()
            .persistent()
            .set(&DataKey::Certificate(certificate_id), &cert);

        Ok(())
    }

    // Anyone can verify whether a certificate exists and is not revoked.
    pub fn verify_certificate(env: Env, certificate_id: u64) -> bool {
        let cert: Option<Certificate> = env.storage().persistent().get(&DataKey::Certificate(certificate_id));
        match cert {
            Some(c) => !c.revoked,
            None => false,
        }
    }

    // Return full certificate records owned by a student wallet.
    pub fn get_certificates_by_owner(env: Env, owner: Address) -> Vec<Certificate> {
        let ids = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<u64>>(&DataKey::OwnerCertificates(owner))
            .unwrap_or(Vec::new(&env));

        let mut certs = Vec::new(&env);
        for id in ids.iter() {
            let cert: Certificate = env
                .storage()
                .persistent()
                .get(&DataKey::Certificate(id))
                .unwrap();
            certs.push_back(cert);
        }

        certs
    }

    // Helper reader for frontends/explorers that need full certificate details by id.
    pub fn get_certificate(env: Env, certificate_id: u64) -> Result<Certificate, CertError> {
        env.storage()
            .persistent()
            .get(&DataKey::Certificate(certificate_id))
            .ok_or(CertError::CertificateNotFound)
    }

    // Soulbound guarantee: transfer is permanently blocked.
    pub fn transfer_certificate(
        env: Env,
        _caller: Address,
        _to: Address,
        _certificate_id: u64,
    ) -> Result<(), CertError> {
        let _ = env;
        Err(CertError::NotTransferable)
    }

    fn require_admin(env: &Env, caller: &Address) -> Result<(), CertError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(CertError::NotInitialized)?;

        if &admin != caller {
            return Err(CertError::NotAdmin);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup() -> (
        Env,
        EduChainsCertsContractClient<'static>,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(EduChainsCertsContract, ());
        let client = EduChainsCertsContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let student = Address::generate(&env);
        let stranger = Address::generate(&env);

        client.init(&admin);

        (env, client, admin, student, stranger)
    }

    #[test]
    fn issuing_a_certificate_works() {
        let (env, client, admin, student, _) = setup();

        let cert_id = client.issue_certificate(
            &admin,
            &student,
            &String::from_str(&env, "Soroban Bootcamp"),
            &1_710_000_000_u64,
        );

        assert_eq!(cert_id, 1);

        let cert = client.get_certificate(&cert_id);
        assert_eq!(cert.id, 1);
        assert_eq!(cert.owner, student.clone());
        assert_eq!(cert.course_name, String::from_str(&env, "Soroban Bootcamp"));
        assert_eq!(cert.issuer, admin.clone());
        assert_eq!(cert.issued_at, 1_710_000_000_u64);
        assert_eq!(cert.revoked, false);

        let by_owner = client.get_certificates_by_owner(&student);
        assert_eq!(by_owner.len(), 1);
        assert_eq!(by_owner.get(0).unwrap().id, cert_id);
    }

    #[test]
    fn revoking_a_certificate_works() {
        let (env, client, admin, student, _) = setup();

        let cert_id = client.issue_certificate(
            &admin,
            &student,
            &String::from_str(&env, "Rust 101"),
            &1_710_000_001_u64,
        );

        assert_eq!(client.verify_certificate(&cert_id), true);

        client.revoke_certificate(&admin, &cert_id);

        assert_eq!(client.verify_certificate(&cert_id), false);
        assert_eq!(client.get_certificate(&cert_id).revoked, true);
    }

    #[test]
    fn verifying_certificate_validity_works() {
        let (env, client, admin, student, _) = setup();

        let valid_id = client.issue_certificate(
            &admin,
            &student,
            &String::from_str(&env, "Blockchain Fundamentals"),
            &1_710_000_002_u64,
        );

        assert_eq!(client.verify_certificate(&valid_id), true);
        assert_eq!(client.verify_certificate(&999_u64), false);
    }

    #[test]
    fn preventing_unauthorized_actions_works() {
        let (env, client, admin, student, stranger) = setup();

        let issue_result = client.try_issue_certificate(
            &stranger,
            &student,
            &String::from_str(&env, "Unauthorized Course"),
            &1_710_000_003_u64,
        );
        assert_eq!(issue_result, Err(Ok(CertError::NotAdmin)));

        let cert_id = client.issue_certificate(
            &admin,
            &student,
            &String::from_str(&env, "Authorized Course"),
            &1_710_000_004_u64,
        );

        let revoke_result = client.try_revoke_certificate(&stranger, &cert_id);
        assert_eq!(revoke_result, Err(Ok(CertError::NotAdmin)));

        assert_eq!(client.verify_certificate(&cert_id), true);
    }

    #[test]
    fn transfer_is_blocked_for_soulbound_behavior() {
        let (env, client, admin, student, stranger) = setup();

        let cert_id = client.issue_certificate(
            &admin,
            &student,
            &String::from_str(&env, "Web3 Identity"),
            &1_710_000_005_u64,
        );

        let transfer_result = client.try_transfer_certificate(&stranger, &student, &cert_id);
        assert_eq!(transfer_result, Err(Ok(CertError::NotTransferable)));

        let cert = client.get_certificate(&cert_id);
        assert_eq!(cert.owner, student);
    }
}

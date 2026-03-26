<img width="1910" height="995" alt="image" src="https://github.com/user-attachments/assets/d7092648-c630-402e-ac09-be6d44158992" />


# Edu

Edu is a Soroban smart contract project for issuing verifiable on-chain certificates on Stellar. The contract models certificates as soulbound records: institutions can issue and revoke them, anyone can verify them, and transfer is permanently blocked.

## Problem

Academic certificates are easy to forge, hard to verify across institutions, and often depend on slow manual checks.

## Solution

Edu stores certificate records on Soroban so an institution can issue a certificate to a wallet address, revoke it when needed, and let third parties verify authenticity directly on-chain.

## Why Stellar

- Low fees make certificate issuance and verification affordable.
- Soroban enables simple, auditable credential logic on-chain.
- Wallet-based ownership gives each learner a portable digital identity anchor.

## Current MVP Scope

- One-time contract initialization with an admin
- Admin-only certificate issuance
- Admin-only certificate revocation
- Public certificate verification
- Query all certificates by owner wallet
- Query full certificate details by ID
- Permanent non-transferability for soulbound behavior

## Smart Contract

Contract location: `contracts/educhains-certs/src/lib.rs`

Certificate fields:

- `id`
- `owner`
- `course_name`
- `issuer`
- `issued_at`
- `revoked`

Storage model:

- `Admin` stores institution admin
- `NextCertificateId` stores the next incremental ID
- `Certificate(id)` stores certificate data
- `OwnerCertificates(address)` stores all certificate IDs for a wallet

Main functions:

- `init(admin)` initializes the contract once
- `issue_certificate(caller, owner, course_name, issued_at)` issues a certificate
- `revoke_certificate(caller, certificate_id)` revokes an issued certificate
- `verify_certificate(certificate_id)` returns validity as `bool`
- `get_certificates_by_owner(owner)` returns full certificate list for a wallet
- `get_certificate(certificate_id)` returns full certificate detail
- `transfer_certificate(...)` always returns `NotTransferable`

## Project Structure

```text
edu/
├── contracts/
│   └── educhains-certs/    # Main certificate contract
│       ├── src/lib.rs
│       └── Cargo.toml
├── modules/                # Soroban learning modules
├── examples/               # Soroban reference examples
├── scaffold/               # Starter scaffold docs
├── skills/                 # AI workflow prompts/guides
└── STUDENT-GUIDE.md
```

## Quick Start

### Prerequisites

- Rust
- `wasm32-unknown-unknown` target
- Stellar CLI

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
```

### Build

```bash
cd contracts/educhains-certs
stellar contract build
```

### Test

```bash
cd contracts/educhains-certs
cargo test
```

## Local Test Coverage

The contract includes unit tests for:

- issuing a certificate
- revoking a certificate
- verifying valid and missing certificates
- blocking unauthorized issue and revoke actions
- enforcing soulbound non-transferability

## Example Flow

1. Institution initializes the contract with an admin wallet.
2. Admin issues a certificate to a student wallet.
3. Anyone can verify the certificate by ID.
4. The institution can revoke the certificate if needed.
5. The student can query all certificates tied to their wallet.

## Example Usage

```bash
# Initialize
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ADMIN> \
  --network testnet \
  -- init \
  --admin <ADMIN_ADDRESS>

# Issue a certificate
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ADMIN> \
  --network testnet \
  -- issue_certificate \
  --caller <ADMIN_ADDRESS> \
  --owner <STUDENT_ADDRESS> \
  --course_name "Soroban Bootcamp" \
  --issued_at 1710000000

# Verify a certificate
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <ANY_ACCOUNT> \
  --network testnet \
  -- verify_certificate \
  --certificate_id 1
```

## Limitations

- The MVP tracks certificates as records, not token-standard NFTs.
- No frontend is included in this repo yet.
- There are no metadata links, institution profiles, or batch issuance tools yet.
- Revocation is a soft status flag rather than record deletion.

## Roadmap

- Add frontend for institutions and students
- Support certificate metadata hashes or IPFS links
- Add issuer profiles and institution branding
- Add event emission for indexing and analytics
- Support QR-friendly verification pages

## Tech Stack

- Rust
- Soroban SDK v22
- Stellar Testnet compatible

## License

Educational project for Soroban learning and prototyping.

# CLAUDE.md — Project Instructions for Claude Code

## Project Overview

This is a learning hub repository for Stellar/Soroban smart contract development.
It targets students with zero to intermediate blockchain knowledge.

## Repository Structure

- `scaffold/` — Guide for official Stellar scaffold (installed via CLI)
- `examples/soroban-examples/` — Official Soroban examples cloned from stellar/soroban-examples v23.0.0
- `modules/` — Bite-sized code modules, best practices, error guides
- `skills/` — AI skill files for accelerated development with Claude Code

## Tech Stack

- **Smart Contracts**: Rust → WebAssembly (Soroban SDK v22)
- **Frontend**: TypeScript/JavaScript with @stellar/stellar-sdk
- **Wallet**: Freighter browser extension
- **Networks**: Testnet (development/default), Futurenet (experimental), Mainnet (production)
- **CLI**: Stellar CLI v25+

## Key Commands

```bash
# Build a contract
stellar contract build

# Run tests
cargo test

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<contract>.wasm \
  --source-account <identity> \
  --network testnet

# Invoke a contract function
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <identity> \
  --network testnet \
  -- <function_name> --arg1 value1

# Generate a funded test identity
stellar keys generate <name> --network testnet --fund
```

## Soroban Contract Conventions

- All contracts use `#![no_std]`
- Use `#[contract]` and `#[contractimpl]` macros
- Storage types: Instance (config), Persistent (user data), Temporary (ephemeral)
- Always use `require_auth()` for state-changing user operations
- Max WASM size: 64KB — use `opt-level = "z"` and LTO
- Symbol keys: max 32 chars, `symbol_short!()` for ≤9 chars

## When Helping Students

- Assume zero blockchain knowledge unless stated otherwise
- Always explain WHY, not just HOW
- Provide complete, runnable code — not fragments
- Always include test code alongside contract code
- Default to Testnet for deployment examples
- Reference the relevant `modules/` folder for deeper explanation

# Skills — AI-Accelerated Development

> These skill files turn Claude Code into a Stellar/Soroban expert copilot.
> Load them into your project and develop at god-speed.

## What Are Skill Files?

Skill files (`.md`) contain structured knowledge that AI tools like Claude Code can use
to write better code faster. They're like giving the AI a cheat sheet of patterns,
conventions, and best practices specific to your tech stack.

## Available Skills

| Skill File | Purpose | Use When |
|-----------|---------|----------|
| [soroban-contract.md](soroban-contract.md) | Write Soroban smart contracts | Building any smart contract |
| [soroban-deploy.md](soroban-deploy.md) | Deploy and manage contracts | Deploying to any network |
| [frontend-dapp.md](frontend-dapp.md) | Build dApp frontends | Creating UI for your contract |
| [testing.md](testing.md) | Test Soroban contracts | Writing and running tests |
| [full-stack-dapp.md](full-stack-dapp.md) | End-to-end dApp development | Building a complete project |

## How to Use

### With Claude Code

These files are automatically available when you work in this repo.
Claude Code reads the CLAUDE.md and skill files to understand your project context.

### With Other AI Tools

Copy the content of any skill file and paste it as context/system prompt:

```
# In ChatGPT / Claude web
"Here is my coding reference. Use these patterns when helping me build Soroban contracts:"
[paste skill file content]
```

### With v0.dev / Cursor

Add the frontend skill file as a custom instruction to generate Stellar-connected UIs.

## Tips for Maximum Speed

1. **Start with the right skill**: Pick the skill that matches your current task
2. **Be specific in prompts**: "Write a token contract with mint/burn/transfer using the patterns from skills"
3. **Iterate fast**: Use skills for the initial scaffold, then customize
4. **Always test**: Skills give you fast code, but always verify with `cargo test`
5. **Combine skills**: Use `soroban-contract.md` + `testing.md` together for TDD workflow

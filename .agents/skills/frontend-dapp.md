# SKILL: Stellar dApp Frontend Development

> Use this reference when building frontend UIs for Stellar/Soroban dApps.

## Tech Stack

- **SDK**: `@stellar/stellar-sdk` (JavaScript/TypeScript)
- **Wallet**: Freighter (`@stellar/freighter-api`)
- **Multi-wallet**: Stellar Wallets Kit (`@creit.tech/stellar-wallets-kit`)
- **RPC**: Soroban RPC for contract interaction
- **Horizon**: REST API for account/transaction queries

## Setup

```bash
npm install @stellar/stellar-sdk @stellar/freighter-api
```

## Network Configuration

```javascript
// networks.js
export const NETWORKS = {
  testnet: {
    rpcUrl: "https://soroban-testnet.stellar.org",
    passphrase: "Test SDF Network ; September 2015",
    explorerUrl: "https://stellar.expert/explorer/testnet",
    friendbotUrl: "https://friendbot.stellar.org",
  },
  futurenet: {
    rpcUrl: "https://rpc-futurenet.stellar.org",
    passphrase: "Test SDF Future Network ; October 2022",
    explorerUrl: "https://stellar.expert/explorer/futurenet",
    friendbotUrl: "https://friendbot-futurenet.stellar.org",
  },
  mainnet: {
    rpcUrl: "https://mainnet.sorobanrpc.com",
    passphrase: "Public Global Stellar Network ; September 2015",
    explorerUrl: "https://stellar.expert/explorer/public",
  },
};

export const NETWORK = NETWORKS.testnet; // Default to testnet for development
```

## Wallet Connection

### Freighter (simplest)
```javascript
import {
  isConnected,
  getAddress,
  signTransaction,
  setAllowed,
} from "@stellar/freighter-api";

async function connectWallet() {
  const connected = await isConnected();
  if (!connected) {
    alert("Please install Freighter wallet extension");
    return null;
  }

  // Request permission
  await setAllowed();

  // Get public key
  const { address } = await getAddress();
  console.log("Connected:", address);
  return address;
}
```

### Stellar Wallets Kit (multi-wallet)
```javascript
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  FREIGHTER_ID,
} from "@creit.tech/stellar-wallets-kit";

const kit = new StellarWalletsKit({
  network: WalletNetwork.TESTNET,
  selectedWalletId: FREIGHTER_ID,
  modules: allowAllModules(),
});

// Open wallet selector modal
await kit.openModal({ onWalletSelected: async (option) => {
  kit.setWallet(option.id);
  const { address } = await kit.getAddress();
  console.log("Connected:", address);
}});
```

## Contract Interaction

### Initialize Client
```javascript
import * as StellarSdk from "@stellar/stellar-sdk";

const CONTRACT_ID = "CABC123..."; // Your deployed contract ID

function getServer() {
  return new StellarSdk.SorobanRpc.Server(NETWORK.rpcUrl);
}

function getContract() {
  return new StellarSdk.Contract(CONTRACT_ID);
}
```

### Call a Read-Only Function (no signature needed)
```javascript
async function readValue(key) {
  const server = getServer();
  const contract = getContract();

  // Build transaction (any valid account works for simulation)
  const account = await server.getAccount(publicKey);
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: NETWORK.passphrase,
  })
    .addOperation(contract.call("get_value", StellarSdk.nativeToScVal(key, { type: "symbol" })))
    .setTimeout(30)
    .build();

  // Simulate (doesn't submit — free)
  const result = await server.simulateTransaction(tx);
  return StellarSdk.scValToNative(result.result.retval);
}
```

### Call a State-Changing Function (needs signature)
```javascript
async function callContract(funcName, ...args) {
  const server = getServer();
  const contract = getContract();
  const publicKey = await connectWallet();

  // 1. Build transaction
  const account = await server.getAccount(publicKey);
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: NETWORK.passphrase,
  })
    .addOperation(contract.call(funcName, ...args))
    .setTimeout(30)
    .build();

  // 2. Simulate (prepare footprint + fees)
  const prepared = await server.prepareTransaction(tx);

  // 3. Sign with Freighter
  const { signedTxXdr } = await signTransaction(prepared.toXDR(), {
    networkPassphrase: NETWORK.passphrase,
  });

  // 4. Submit
  const signedTx = StellarSdk.TransactionBuilder.fromXDR(
    signedTxXdr,
    NETWORK.passphrase
  );
  const response = await server.sendTransaction(signedTx);

  // 5. Wait for confirmation
  if (response.status === "PENDING") {
    let result;
    do {
      await new Promise((r) => setTimeout(r, 1000));
      result = await server.getTransaction(response.hash);
    } while (result.status === "NOT_FOUND");
    return result;
  }

  return response;
}
```

## Value Conversion (JS ↔ Soroban)

```javascript
import { nativeToScVal, scValToNative, Address } from "@stellar/stellar-sdk";

// JavaScript → Soroban ScVal
nativeToScVal("hello", { type: "string" })       // String
nativeToScVal(42, { type: "i128" })               // i128
nativeToScVal(100, { type: "u32" })               // u32
nativeToScVal("test", { type: "symbol" })         // Symbol
nativeToScVal(true, { type: "bool" })             // Bool
new Address(publicKey).toScVal()                   // Address

// Soroban ScVal → JavaScript
scValToNative(scVal)  // auto-converts to JS type

// Struct
nativeToScVal({
  name: "Hello",
  amount: BigInt(1000),
}, { type: { name: "string", amount: "i128" } })
```

## Complete HTML Template

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>My Stellar dApp</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: system-ui, sans-serif; background: #0a0a0a; color: #fff; }
    .container { max-width: 600px; margin: 0 auto; padding: 2rem; }
    .card { background: #1a1a2e; border-radius: 12px; padding: 1.5rem; margin: 1rem 0; }
    button { background: #6366f1; color: white; border: none; padding: 0.75rem 1.5rem;
             border-radius: 8px; cursor: pointer; font-size: 1rem; }
    button:hover { background: #4f46e5; }
    button:disabled { opacity: 0.5; cursor: not-allowed; }
    input { width: 100%; padding: 0.75rem; border: 1px solid #333; border-radius: 8px;
            background: #0a0a0a; color: #fff; margin: 0.5rem 0; font-size: 1rem; }
    .status { padding: 0.5rem; border-radius: 4px; margin: 0.5rem 0; font-size: 0.875rem; }
    .success { background: #065f46; }
    .error { background: #7f1d1d; }
    .address { font-family: monospace; font-size: 0.875rem; word-break: break-all; }
    h1 { margin-bottom: 1rem; }
    h2 { margin-bottom: 0.5rem; font-size: 1.25rem; }
    label { display: block; margin-top: 0.5rem; font-size: 0.875rem; color: #999; }
  </style>
</head>
<body>
  <div class="container">
    <h1>My Stellar dApp</h1>

    <!-- Wallet Connection -->
    <div class="card">
      <h2>Wallet</h2>
      <button id="connectBtn" onclick="handleConnect()">Connect Wallet</button>
      <p id="walletAddress" class="address" style="margin-top: 0.5rem;"></p>
    </div>

    <!-- Main Action -->
    <div class="card">
      <h2>Your Action</h2>
      <label>Input</label>
      <input type="text" id="inputField" placeholder="Enter value..." />
      <button onclick="handleAction()" style="margin-top: 0.5rem;">Submit</button>
    </div>

    <!-- Result -->
    <div class="card">
      <h2>Result</h2>
      <div id="result"></div>
    </div>
  </div>

  <script type="module" src="app.js"></script>
</body>
</html>
```

## React Component Pattern (with Scaffold Stellar)

```tsx
import { useState } from "react";
import { useSorobanReact } from "@soroban-react/core";

function ContractInteraction() {
  const { address, connect, sendTransaction } = useSorobanReact();
  const [result, setResult] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleSubmit(value: string) {
    if (!address) return alert("Connect wallet first");
    setLoading(true);
    try {
      const tx = await sendTransaction(/* build tx */);
      setResult(`Success: ${tx.hash}`);
    } catch (err) {
      setResult(`Error: ${err.message}`);
    }
    setLoading(false);
  }

  return (
    <div>
      {!address ? (
        <button onClick={connect}>Connect Wallet</button>
      ) : (
        <p>Connected: {address.slice(0, 8)}...{address.slice(-4)}</p>
      )}
      <button onClick={() => handleSubmit("value")} disabled={loading}>
        {loading ? "Processing..." : "Submit"}
      </button>
      {result && <p>{result}</p>}
    </div>
  );
}
```

## Error Handling Pattern

```javascript
async function safeContractCall(funcName, ...args) {
  const statusEl = document.getElementById("result");

  try {
    statusEl.innerHTML = '<div class="status">Processing...</div>';

    const result = await callContract(funcName, ...args);

    if (result.status === "SUCCESS") {
      const returnVal = scValToNative(result.returnValue);
      statusEl.innerHTML = `<div class="status success">
        Success! TX: <a href="${NETWORK.explorerUrl}/tx/${result.hash}" target="_blank">${result.hash.slice(0, 12)}...</a>
      </div>`;
      return returnVal;
    } else {
      statusEl.innerHTML = `<div class="status error">Failed: ${result.status}</div>`;
    }
  } catch (err) {
    statusEl.innerHTML = `<div class="status error">Error: ${err.message}</div>`;
    console.error(err);
  }
}
```

## AI Prompt for Generating UI

When using v0.dev, Claude, or ChatGPT to generate your UI, use this template:

```
Create a clean, modern web UI for a [PROJECT NAME] dapp on Stellar blockchain.

The app should have:
- A header with project name and "Connect Wallet" button
- Main section with [DESCRIBE YOUR FEATURE]
- A status panel showing wallet address and network
- Result area showing transaction hash with link to Stellar Expert
- Dark theme, minimal design, system fonts

Use plain HTML, CSS, and vanilla JavaScript.
Do NOT include blockchain logic — just the UI layout and event listeners.
I will wire up the Stellar SDK separately.
```

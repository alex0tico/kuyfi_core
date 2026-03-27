```diff
+    __ __  __  __  __  __  ____  ____
+   / // / / / / /  \ \/ / / __/ /  _/
+  / ,<   / /_/ /    \  / / _/  _/ /  
+ /_/|_|  \____/     /_/ /_/   /___/  
+                                     
+ CORE SECURITY MODULE                  
```

# Kuyfi Core: Active Cyber-Security Shield for Web3

Kuyfi Core is a proactive, decentralized security module designed to protect smart contracts and SME liquidity pools on the Stellar (Soroban) network. It acts as an on-chain "Circuit Breaker," combining strict access controls, mathematical safety, and event-driven architecture to mitigate Web3 attack vectors.

## Architecture & Threat Model

This protocol is built with a DevSecOps mindset. The smart contract architecture strictly follows the Read-Calculate-Save pattern to prevent state manipulation.

Mitigated threats include:
* **Spoofing & Unauthorized Access:** Enforced through Soroban's native `require_auth()` signature verification.
* **Integer Overflow/Underflow:** Mitigated by strictly utilizing Rust's checked arithmetic (`checked_add`, `checked_sub`).
* **Value Injection:** Logic constraints to prevent negative amount attacks.
* **Liquidity Draining:** Pre-calculation validations ensuring withdrawal requests never exceed current balances.

## How It Works

Kuyfi Core exposes two public functions — `deposit` and `withdraw` — that manage per-address balances in on-chain persistent storage. Every call follows a strict **Read-Calculate-Save** pipeline:

1. **Authenticate** — The caller's address is verified via `require_auth()`.
2. **Validate** — The amount is rejected if it is zero or negative (`PoolError::NegativeAmount`).
3. **Read** — The current balance is loaded from persistent storage (defaults to `0` for new addresses).
4. **Calculate** — The new balance is computed using checked arithmetic (`checked_add` / `checked_sub`), returning `PoolError::MathOverflow` on overflow or `PoolError::InsufficientFunds` if a withdrawal exceeds the balance.
5. **Save** — The updated balance is written back to storage.
6. **Emit** — A `deposit` or `withdraw` event is published for off-chain consumers.

### Error Reference

| Error | Code | Trigger |
|---|---|---|
| `NotAuthorized` | 1 | Authentication failure |
| `InsufficientFunds` | 2 | Withdrawal exceeds balance |
| `NegativeAmount` | 3 | Amount is zero or negative |
| `MathOverflow` | 4 | Arithmetic overflow/underflow |

## Security Audit (SAST)

Kuyfi Core has undergone Static Application Security Testing (SAST) using CoinFabrik's `cargo-scout-audit`.

**Latest Audit Results:**
* Critical Vulnerabilities: 0
* Medium Vulnerabilities: 0
* Minor Vulnerabilities: 0
* Status: Passed and production-ready.

## Event-Driven Integration

The contract emits cryptographic events (`deposit` and `withdraw`). This allows off-chain heuristic monitors and NIDS (Network Intrusion Detection Systems) to listen to the blockchain in real-time and trigger automated security responses.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021+)
- The `wasm32-unknown-unknown` target
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

```bash
# Add the WebAssembly target (if not already installed)
rustup target add wasm32-unknown-unknown

# Install the Soroban CLI
cargo install --locked soroban-cli
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled `.wasm` binary will be at:
```text
target/wasm32-unknown-unknown/release/kuyfi_core.wasm
```

### Deploy (Testnet)

```bash
# Configure the Soroban CLI for testnet
soroban network add testnet \
  --rpc-url [https://soroban-testnet.stellar.org:443](https://soroban-testnet.stellar.org:443) \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate a test identity and fund it
soroban keys generate alice
soroban keys fund alice --network testnet

# Deploy the contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kuyfi_core.wasm \
  --source alice \
  --network testnet
```

The deploy command returns a **Contract ID** — save it for the next step.

### Interact

```bash
# Deposit 1000 units
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- deposit \
  --from alice \
  --amount 1000

# Withdraw 500 units
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- withdraw \
  --to alice \
  --amount 500
```

Replace `<CONTRACT_ID>` with the ID returned during deployment.
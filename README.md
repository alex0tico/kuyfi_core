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

## Security Audit (SAST)

Kuyfi Core has undergone Static Application Security Testing (SAST) using CoinFabrik's cargo-scout-audit.

**Latest Audit Results:**
* Critical Vulnerabilities: 0
* Medium Vulnerabilities: 0
* Minor Vulnerabilities: 0
* Status: Passed and production-ready.

## Event-Driven Integration

The contract emits cryptographic events (`deposit` and `withdraw`). This allows off-chain heuristic monitors and NIDS (Network Intrusion Detection Systems) to listen to the blockchain in real-time and trigger automated security responses.

## Build Instructions

To compile the smart contract for the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

## How to Use (Local Sandbox Simulation)

You can interact with Küyfi Core locally to test its security constraints and circuit-breaker logic using the [Soroban CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup).

**1. Simulate a Secure Deposit:**
Triggers the `deposit` function, simulating a user funding the protection pool.
```bash
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/kuyfi_core.wasm \
  --id 1 \
  -- \
  deposit \
  --from <USER_ADDRESS> \
  --amount 500

  soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/kuyfi_core.wasm \
  --id 1 \
  -- \
  withdraw \
  --to <USER_ADDRESS> \
  --amount 200

  soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/kuyfi_core.wasm \
  --id 1 \
  -- \
  withdraw \
  --to <ATTACKER_ADDRESS> \
  --amount 9999999
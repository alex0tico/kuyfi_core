# Küyfi Core: Active Cyber-Security Shield for Web3

Küyfi Core is a proactive, decentralized security module designed to protect smart contracts and SME liquidity pools on the Stellar (Soroban) network. It acts as an on-chain "Circuit Breaker," combining strict access controls, mathematical safety, and event-driven architecture to mitigate Web3 attack vectors.

## Architecture & Threat Model

This protocol was built with a DevSecOps mindset from day one. Our smart contract architecture strictly follows the **Read-Calculate-Save** pattern to prevent state manipulation. 

We have successfully modeled and mitigated the following critical threats:
* **Spoofing & Unauthorized Access:** Enforced through Soroban's native `require_auth()` signature verification.
* **Integer Overflow/Underflow:** Mitigated by strictly utilizing Rust's checked arithmetic (`checked_add`, `checked_sub`).
* **Value Injection:** Logic constraints to prevent negative amount attacks.
* **Liquidity Draining:** Pre-calculation validations to ensure withdrawal requests never exceed current balances.

## Security Audit (SAST)

Küyfi Core has undergone Static Application Security Testing (SAST) using CoinFabrik's **cargo-scout-audit**. 

**Latest Audit Results:**
* **Critical Vulnerabilities:** 0
* **Medium Vulnerabilities:** 0
* **Minor Vulnerabilities:** 0
* **Status:** Passed and production-ready.

## 📡 Event-Driven Integration

The contract emits cryptographic events (`deposit` and `withdraw`). This is specifically designed to allow off-chain heuristic monitors and NIDS (Network Intrusion Detection Systems) to listen to the blockchain in real-time and trigger automated security responses.

## Build Instructions

To compile the smart contract for the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
# 💠 PROVION

> **The Autonomous Meritocracy Protocol.**  
> A self-sustaining, cryptographically enforced economy for open work.  
> **Pay for code. Verified by math. Settled by consensus.**

[![License](https://img.shields.io/badge/License-Apache_2.0-green.svg)](./LICENSE)
[![Stack](https://img.shields.io/badge/Stack-Arbitrum_Orbit_%7C_Stylus-blue)](https://arbitrum.io/stylus)
[![ZK](https://img.shields.io/badge/ZK-RISC_Zero-orange)](https://risczero.com)
[![Build Status](https://img.shields.io/badge/build-passing-success)]()

---

## 📡 Abstract

**PROVION** is a Layer 3 App-Chain built to solve the "Oracle Problem" of digital labor. It eliminates managerial overhead by replacing trust with **Zero-Knowledge Proofs** and **Economic Consensus**.

The protocol allows organizations to operate as autonomous code:
1.  **Backers** fund issues with yield-bearing stablecoins.
2.  **Contributors** solve issues via standard Git workflows.
3.  **The Protocol** verifies the work via ZK-proofs of cryptographic signatures.
4.  **Payment** is instant, deterministic, and gasless.

---

## 🏗 Architecture

PROVION operates as a sovereign chain on the **Arbitrum Orbit** stack, utilizing **Stylus (Rust)** for execution efficiency and **RISC Zero** for off-chain verification.

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Network** | Arbitrum Orbit (L3) | Dedicated throughput, gasless transactions via Paymaster. |
| **Execution** | Stylus (Rust) | High-performance logic (Yield Escrows, Reputation Math). |
| **Verification** | RISC Zero (zkVM) | Verifies Git Commit Signatures (Ed25519/GPG) off-chain. |
| **Data Availability** | Celestia | Immutable storage for contribution logs. |

---

## ⚡ Key Mechanisms

### 1. Yield-Bearing Escrows (YBE)
Bounties are funded in USDC. Idle funds are automatically routed to low-risk yield strategies (e.g., Aave).
*   **Self-Sustaining:** Generated yield pays for L3 Gas and ZK-Proof generation.
*   **No Friction:** Contributors and Reviewers pay **$0 gas**.

### 2. Git-Native ZK Verification
We do not trust GitHub's API, TLS, or Webhooks. We verify the mathematics of the commit.
```rust
// Simplified Logic
fn verify(commit_hash: B256, signature: Bytes, maintainer_key: PubKey) -> bool {
    // Verified inside zkVM (RISC Zero)
    ecrecover(commit_hash, signature) == maintainer_key
}

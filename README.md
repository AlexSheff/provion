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
1. **Backers** fund issues with yield-bearing stablecoins.
2. **Contributors** solve issues via standard Git workflows.
3. **The Protocol** verifies the work via ZK-proofs of cryptographic signatures.
4. **Payment** is instant, deterministic, and gasless.

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
- **Self-Sustaining:** Generated yield pays for L3 Gas and ZK-Proof generation.
- **No Friction:** Contributors and Reviewers pay **$0 gas**.

### 2. Git-Native ZK Verification
We do not trust GitHub's API, TLS, or Webhooks. We verify the mathematics of the commit.

```rust
// Simplified Logic inside zkVM
fn verify(commit_hash: B256, signature: Bytes, maintainer_key: PubKey) -> bool {
    // Verifies Ed25519/GPG signature off-chain
    ecrecover(commit_hash, signature) == maintainer_key
}
```

### 3. The "Fisherman" Defense
- **Staking:** Reviewers stake `$PVN` tokens to vote on PRs.
- **Slashing:** Any external observer (running a "Fisherman Node") can challenge a payout within the Dispute Window.
- **Reward:** Successful challengers receive 50% of the slashed reviewer stake.

---

## 📂 Repository Structure

```bash
provion/
├── contracts/          # Smart Contracts (Arbitrum Stylus / Rust)
│   ├── src/lib.rs      # Core Protocol Logic (Bounty, Registry)
│   └── Cargo.toml      # Stylus Configuration
├── prover/             # ZK Circuits (RISC Zero)
│   ├── methods/guest   # The Logic running inside zkVM (Signature Verify)
│   └── host/           # Proof generator agent
├── ui/                 # Frontend (Next.js + Tailwind)
├── integration-tests/  # Hardhat & JS Scripts
└── scripts/            # Deployment & Node configuration
```

---

## 🚀 Quick Start

### Prerequisites
- Rust & Cargo (stable toolchain)
- Docker (For local Orbit node)
- `rzup` (RISC Zero toolchain)
- `cargo-stylus` CLI

### 1. Installation
```bash
# Install Stylus CLI
cargo install --force cargo-stylus

# Clone Repo
git clone https://github.com/AlexSheff/provion.git
cd provion
```

### 2. Run Local DevNet
```bash
# Clone and start Arbitrum Nitro testnode
git clone https://github.com/OffchainLabs/nitro-testnode
./nitro-testnode/test-node.bash --init
```

### 3. Build & Deploy Contracts
```bash
cd contracts
# Verify WASM compatibility
cargo stylus check
# Deploy to local node
cargo stylus deploy --endpoint=http://localhost:8547 --private-key=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

---

## 🗺 Roadmap

- **Phase 0**: Architecture Spec & Data Structures  
- **Phase 1**: Core Contracts (Stylus) & Local DevNet  
- **Phase 2**: RISC Zero Integration (Git Signature Verification)  
- **Phase 3**: Yield-Bearing Escrow Integration  
- **Phase 4**: Audited Testnet & "Fisherman" Bounties  
- **Phase 5**: Mainnet Genesis  

---

## 🛡️ Security Model

PROVION operates under an Adversarial Threat Model.

- **No Admin Keys**: Protocol parameters are governed by Reputation-Weighted DAO.
- **Explicit Trust Boundaries**: The protocol recognizes only:
  - Valid ECDSA/Ed25519 Signatures.
  - Valid ZK-STARK receipts.
  - Consensus Weight > Threshold.
- **Anti-Collusion**: Reviewer stakes are slashable via permissionless fraud proofs.

---

## 🤝 Contributing

We follow a strict **"Eat Your Own Dogfood"** policy.  
To contribute to PROVION, you must solve a bounty on the PROVION Protocol itself.

1. Find an open Issue.
2. Submit a PR.
3. Wait for the ZK-Proof verification.
4. Get paid automatically.

---

## 📄 License

This project is licensed under the **Apache 2.0** — see the [LICENSE](./LICENSE) file for details.

> **Disclaimer**: This code is currently in Alpha. It has not yet undergone a formal security audit. Use at your own risk.

<div align="center">
<sub>© 2026 PROVION Foundation. Open Source. Unstoppable.</sub>
</div>
```

---


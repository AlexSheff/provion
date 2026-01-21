# 💠 PROVION

The Autonomous Meritocracy Protocol.
A self-sustaining, cryptographically enforced economy for open work.
Pay for code. Verified by math. Settled by consensus.

## 📡 Abstract
PROVION is a Layer 3 App-Chain built to solve the "Oracle Problem" of digital labor. It eliminates managerial overhead by replacing trust with Zero-Knowledge Proofs and Economic Consensus.
The protocol allows organizations to operate as autonomous code:

- Backers fund issues with yield-bearing stablecoins.
- Contributors solve issues via standard Git workflows.
- The Protocol verifies the work via ZK-proofs of cryptographic signatures.
- Payment is instant, deterministic, and gasless.

## 🏗 Architecture
PROVION operates as a sovereign chain on the Arbitrum Orbit stack, utilizing Stylus (Rust) for execution efficiency and RISC Zero for off-chain verification.

| Component | Technology | Role |
|-----------|------------|------|
| Network | Arbitrum Orbit (L3) | Dedicated throughput, gasless transactions via Paymaster. |
| Execution | Stylus (Rust) | High-performance logic (Yield Escrows, Reputation Math). |
| Verification | RISC Zero (zkVM) | Verifies Git Commit Signatures (Ed25519/GPG) off-chain. |
| Data Availability | Celestia | Immutable storage for contribution logs. |

## ⚡ Key Mechanisms

### 1. Yield-Bearing Escrows (YBE)
Bounties are funded in USDC. Idle funds are automatically routed to low-risk yield strategies (e.g., Aave).
- **Self-Sustaining**: Generated yield pays for L3 Gas and ZK-Proof generation.
- **No Friction**: Contributors and Reviewers pay $0 gas.

### 2. Git-Native ZK Verification
We do not trust GitHub's API, TLS, or Webhooks. We verify the mathematics of the commit.

```rust
// Simplified Logic
fn verify(commit_hash: B256, signature: Bytes, maintainer_key: PubKey) -> bool {
    // Verified inside zkVM (RISC Zero)
    ecrecover(commit_hash, signature) == maintainer_key
}
```

### 3. The "Fisherman" Defense
- **Staking**: Reviewers stake $PVN tokens to vote on PRs.
- **Slashing**: Any external observer (running a "Fisherman Node") can challenge a payout within the Dispute Window.
- **Reward**: Successful challengers receive 50% of the slashed reviewer stake.

## 🚀 Quick Start

### Prerequisites
- Rust & Cargo (stable toolchain)
- Docker (For local Orbit node)
- Rzup (RISC Zero toolchain)

### 1. Installation

```bash
# Install Stylus CLI
cargo install --force cargo-stylus

# Clone Repo
git clone https://github.com/provion-network/provion-core.git
cd provion-core
```

### 2. Run Local DevNet

```bash
# Start a local Arbitrum Nitro node
./scripts/start-local-node.sh
```

### 3. Deploy Contracts

```bash
cd contracts
cargo stylus check
cargo stylus deploy --endpoint=http://localhost:8547 --private-key=0x...
```

## 📂 Repository Structure

```
provion-core/
├── contracts/          # Smart Contracts (Rust/Stylus)
│   ├── src/lib.rs      # Core Protocol Logic (Bounty, Registry)
│   └── Cargo.toml      # Stylus Configuration
├── prover/             # ZK Circuits (RISC Zero)
│   ├── methods/guest   # The Logic running inside zkVM
│   └── host/           # Proof generator agent
├── ui/                 # Frontend (Next.js + Tailwind)
└── scripts/            # Deployment & Integration Tests
```

## 🛡️ Security Model
PROVION operates under an Adversarial Threat Model.
- **No Admin Keys**: Protocol parameters are governed by Reputation-Weighted DAO.
- **Explicit Trust Boundaries**: The protocol recognizes only:
    - Valid ECDSA/Ed25519 Signatures.
    - Valid ZK-STARK receipts.
    - Consensus Weight > Threshold.
- **Anti-Collusion**: Reviewer stakes are slashable via permissionless fraud proofs.

## 🤝 Contributing
We follow a strict "Eat Your Own Dogfood" policy.
To contribute to PROVION, you must solve a bounty on the PROVION Protocol itself.
1. Find an open Issue on PROVION Bounty Board.
2. Submit a PR.
3. Wait for the ZK-Proof verification.
4. Get paid automatically.

See CONTRIBUTING.md for setup details.

## 📄 License
This project is licensed under the Apache 2.0 - see the LICENSE file for details.

Disclaimer: This code is currently in Alpha. It has not yet undergone a formal security audit. Use at your own risk.

© 2026 PROVION Foundation. Open Source. Unstoppable.

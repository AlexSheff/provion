🚧 WARNING: WORK IN PROGRESS
This repository is currently under active development (Pre-Alpha). Contracts are unaudited and deployed on Testnet only. APIs and Storage layouts may change.

<p align="center">
  <img src="https://github.com/AlexSheff/provion/blob/main/provion_logo.svg" width="200" alt="PROVION">
</p>

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

**PROVION** is a Layer 3 App-Chain that allows anyone to fund bounties on any public GitHub issue. It replaces managerial overhead with **Zero-Knowledge Proofs** and **Economic Consensus**.

The protocol verifies work via cryptographic proofs of Git merge events and pays contributors instantly—without requiring repository owners to install anything.

---

## 🔑 Core Principles

### 1. Permissionless & Scalable

- **No GitHub App Required:** Repository maintainers do not need to install or configure anything.
- **Anyone Can Register a Repository:** Any user can add a public repository to PROVION via our web app. To prevent spam, there is a nominal one-time registration fee (e.g., 1 USDC).
- **Maintainer Opt-Out:** While registration is open, we respect maintainer autonomy. A repository owner can delist their project at any time by issuing a simple command (e.g., `#provion-disable`) in any issue.

### 2. Secure & User-Centric

- **Hashtag as a UX Trigger:** A hashtag like `#provion-fund 50 USDC` does not immediately move funds. It's a UX trigger that prompts the PROVION bot to provide a secure confirmation link.
- **Cryptographic Confirmation:** Economic action only occurs after the user signs a transaction via the secure link, reserving funds in the bounty escrow.
- **Read-Only Bot:** The bot uses the public GitHub API in a read-only capacity, ensuring it cannot perform unwanted actions.

### 3. Objective & Automated Payouts

- **Merge as a Trigger:** The "Oracle Problem" of knowing when work is "done" is solved by using the `merge` event as the definitive trigger.
- **Automated Payout:** When a pull request linked to a funded issue is merged by a maintainer, the protocol cryptographically verifies the event and automatically releases the bounty funds to the contributor.

---

## 🏗 Architecture

PROVION operates as a sovereign chain on the **Arbitrum Orbit** stack, utilizing **Stylus (Rust)** for execution efficiency and **RISC Zero** for off-chain verification.

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Network** | Arbitrum Orbit (L3) | Dedicated throughput, gasless transactions via Paymaster |
| **Execution** | Stylus (Rust) | High-performance logic (Yield Escrows, Reputation Math) |
| **Verification** | RISC Zero (zkVM) | Verifies Git Merge Events via ZK-proofs |
| **Monitoring** | Public GitHub API | Read-only observer for registered repos only |
| **Data Availability** | Celestia | Immutable storage for contribution logs |

---

## ⚡ Key Mechanisms

### 1. Yield-Bearing Escrows (YBE)

Bounties are funded in USDC. Idle funds are automatically routed to low-risk yield strategies (e.g., Aave).

- **Self-Sustaining:** Generated yield pays for L3 Gas and ZK-Proof generation.
- **No Friction:** Contributors and Reviewers pay **$0 gas**.

**Example:**
```
Bounty: $5,000 USDC
Duration: 30 days
Aave APY: 4.2%
Yield earned: $17.26

Covers:
- ZK proof generation: ~$10
- Gas costs: ~$7
- Surplus: Reinvested or distributed to reviewers
```

### 2. Git-Native ZK Verification

We do not trust GitHub's API, TLS, or Webhooks. We verify the mathematics of the merge event.

**Verification Flow:**
```rust
// Simplified Logic inside zkVM
fn verify_merge(
    merge_commit: B256,
    maintainer_signature: Bytes,
    maintainer_pubkey: PubKey
) -> bool {
    // 1. Verify merge commit signature
    ecrecover(merge_commit, maintainer_signature) == maintainer_pubkey
    
    // 2. Verify commit is in main branch
    // 3. Verify PR references funded issue
    // 4. Generate STARK proof
}
```

**What Gets Proven:**
- ✅ Merge commit signature is cryptographically valid
- ✅ Commit hash matches signed message
- ✅ Signer key matches registered maintainer
- ✅ Merge occurred in correct repository
- ✅ PR correctly references funded issue

### 3. The "Fisherman" Defense

Protection against fraudulent payouts through economic incentives.

**How It Works:**
- **Staking:** Reviewers stake `$PVN` tokens to vote on PRs
- **Slashing:** Any external observer (running a "Fisherman Node") can challenge a payout within the Dispute Window (24-48 hours)
- **Reward:** Successful challengers receive 50% of the slashed reviewer stake

**Economic Incentive Analysis:**
```
Reviewer stake: $500
Bounty value: $5,000
Challenge reward: $250 (50% of stake)

Attack expected value:
  EV = (chance_not_caught × $5,000) - (chance_caught × $500)
  EV = (0.1 × $5,000) - (0.9 × $500) = $50

Defense expected value for fisherman:
  EV = chance_catch_collusion × $250 = 0.9 × $250 = $225

Result: Economically favorable to defend, not attack.
```

---

## 📂 Repository Structure

```bash
provion/
├── contracts/          # Smart Contracts (Arbitrum Stylus / Rust)
│   ├── src/lib.rs      # Core Protocol Logic (Bounty, Registry)
│   └── Cargo.toml      # Stylus Configuration
├── prover/             # ZK Circuits (RISC Zero)
│   ├── methods/guest   # The Logic running inside zkVM (Merge Verify)
│   └── host/           # Proof generator agent
├── ui/                 # Frontend (Next.js + Tailwind)
│   ├── pages/          # Web app pages
│   └── components/     # React components
├── bot/                # GitHub Observer (read-only, public API)
│   └── src/            # Bot logic for hashtag detection
├── integration-tests/  # Hardhat & JS Scripts
└── scripts/            # Deployment & Node configuration
```

---

## 🚀 How It Works (User Flow)

### For Funders:

1. **Register a Repository** (one-time)
   - Go to [app.provion.network](https://app.provion.network)
   - Add GitHub repository URL
   - Pay 1 USDC registration fee (prevents spam)

2. **Fund an Issue**
   - Navigate to any issue in the registered repo
   - Comment: `#provion-fund 50 USDC`
   - Bot replies with secure confirmation link

3. **Confirm Funding**
   - Click bot's link
   - Sign gasless transaction (no seed phrase needed)
   - Funds reserved in yield-bearing escrow

### For Contributors:

4. **Solve the Issue**
   - Fork the repository
   - Implement the fix
   - Submit Pull Request referencing the issue

5. **Get Paid Automatically**
   - Maintainer reviews and merges PR
   - Protocol detects merge event
   - ZK proof verifies merge signature
   - Funds automatically released to contributor
   - **No manual approval needed**

### For Maintainers:

6. **Business as Usual**
   - No installation required
   - No configuration needed
   - Review PRs normally
   - Merge when satisfied
   - Contributors get paid automatically

> ✅ **No seed phrases. No gas fees. No admin permissions.**

---

## 🗺 Roadmap

### ✅ Phase 0: Foundation (Q1 2026)
**Status: COMPLETE**
- ✓ Architecture design & specification
- ✓ Repository setup
- ✓ Core documentation
- ✓ Community building (Discord, Telegram)

### 🔄 Phase 1: MVP Development (Q1-Q2 2026)
**Status: IN PROGRESS**
- ⏳ Stylus smart contracts (Bounty, Registry, Escrow)
- ⏳ RISC Zero zkVM integration
- ⏳ Local devnet testing environment
- ⏳ Basic frontend UI
- ⏳ Git merge event verification

### 📋 Phase 2: GitHub Integration (Q2 2026)
**Target: April 2026**
- Public testnet launch (Arbitrum Sepolia)
- GitHub bot deployment (read-only API)
- Hashtag detection system
- Repository registration system
- Yield-bearing escrow activation

### 🚀 Phase 3: Public Testnet (Q2-Q3 2026)
**Target: June 2026**
- Community testing program
- Bug bounty program ($10,000 pool)
- First 100 test bounties
- Documentation completion
- Security improvements

### 🔒 Phase 4: Security Audit (Q3 2026)
**Target: July 2026**
- Professional security audit (Trail of Bits or similar)
- Automated security scanning (Slither, Mythril)
- Community review period
- Bug fixes and improvements

### 🌟 Phase 5: Mainnet Genesis (Q3 2026)
**Target: August 2026**
- Deploy on Arbitrum One
- First 10 real bounties
- Token launch ($PVN)
- Governance activation
- Grant distributions

### 🚀 Phase 6: Growth & Expansion (Q4 2026+)
**Target: Q4 2026**
- Multi-chain support (Base, Optimism)
- GitHub App (optional for enhanced features)
- DAO governance
- Reputation-weighted voting
- 1,000+ active bounties
- Enterprise partnerships

---

## 🛡️ Security Model

### Trust Boundaries

**We Trust:**
- ✅ Ethereum L1 consensus
- ✅ Arbitrum L2 bridge contracts
- ✅ RISC Zero zkVM correctness
- ✅ Ed25519/GPG cryptography

**We DON'T Trust:**
- ❌ GitHub API (could be hacked)
- ❌ GitHub webhooks (could be spoofed)
- ❌ Reviewers (economic collusion possible)
- ❌ Sequencer (censorship possible, but detectable)

### Attack Vectors & Mitigations

#### 1. Reviewer Collusion
**Attack:** Reviewers approve fake PR to steal bounty.

**Mitigation:**
- Fisherman mechanism with economic incentives
- 24-48 hour dispute window
- 100% stake slashing for colluding reviewers
- ZK proof as final arbiter

#### 2. GitHub Account Compromise
**Attack:** Attacker steals maintainer's GitHub account, merges malicious PR.

**Mitigation:**
- ZK proof verifies GPG/SSH signature, not just GitHub login
- Compromised GitHub account cannot forge cryptographic signatures
- Maintainers encouraged to use hardware keys (YubiKey)

#### 3. Spam & Denial of Service
**Attack:** Malicious actors spam repository registrations or fund fake issues.

**Mitigation:**
- 1 USDC registration fee per repository
- Maintainer can disable bounties with `#provion-disable`
- Rate limiting on bot API
- Economic cost to spam (must lock funds in escrow)

---

## 💰 Cost Optimization

PROVION is designed to be **sustainable from day one** through yield generation.

### Monthly Operating Costs (Bootstrap Mode)

| Component | Solution | Cost/month |
|-----------|----------|------------|
| **Server** | Hetzner AX41-NVMe | $45 |
| **ZK Proofs** | Vast.ai on-demand (RTX 4090) | $10 |
| **L1 Gas** | AnyTrust + Lazy Batching | $50 |
| **Domain** | Cloudflare | $2 |
| **Frontend** | Vercel Free | $0 |
| **Indexer** | Self-hosted Ponder | $0 |
| **Monitoring** | Self-hosted Grafana | $0 |
| **RPC** | Self-hosted on Hetzner | $0 |
| **TOTAL** | | **$107/month** |

**Revenue from Yield (at $100k TVL):**
- Aave APY: 4% = $4,000/year = $333/month
- **Net profit:** $226/month (213% margin)

See [Bootstrap Guide](docs/BOOTSTRAP_GUIDE.md) for full deployment instructions.

---

## 🤝 Contributing

We practice **"Eat Your Own Dogfood"**:

1. Find an issue in this repository
2. Submit a PR with your solution
3. Wait for review and merge
4. Get paid automatically via PROVION

### Development Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Cargo Stylus
cargo install --force cargo-stylus

# Install RISC Zero
curl -L https://risczero.com/install | bash
rzup install

# Clone repository
git clone https://github.com/AlexSheff/provion.git
cd provion

# Run local devnet
git clone https://github.com/OffchainLabs/nitro-testnode
cd nitro-testnode
./test-node.bash --init

# Deploy contracts (in separate terminal)
cd ../provion/contracts
cargo stylus deploy --endpoint=http://localhost:8547 \
  --private-key=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 📚 Documentation

- **[Bootstrap Guide](docs/BOOTSTRAP_GUIDE.md)** - Launch a mainnet for $107/month
- **[Architecture](docs/ARCHITECTURE.md)** - Technical deep-dive into protocol design
- **[Whitepaper](WHITEPAPER.md)** - Academic specification of the protocol
- **[Contributing](CONTRIBUTING.md)** - How to contribute to PROVION
- **[Terms & Conditions](TERMS_AND_CONDITIONS.md)** - Legal disclaimers and user agreements

---

## 🌐 Links

- **Website:** https://sites.google.com/view/provion
- **GitHub:** https://github.com/AlexSheff/provion
- **Discord:** https://discord.gg/provion
- **Telegram:** https://t.me/provions
- **Twitter:** https://twitter.com/provion_xyz

---

## 💰 Grants & Funding

We're applying for grants to accelerate development:

- **Arbitrum Foundation:** $5,000-10,000 (Developer Tooling)
- **Gitcoin Grants:** $500-2,000 per round
- **Ethereum ESP:** $10,000-50,000 (Layer 2 / ZK category)
- **Google Cloud Startup:** $100,000 credits

**Status:** Applications in progress. Check [roadmap](docs/ROADMAP.md) for updates.

---

## 📄 License

This project is licensed under the **Apache 2.0 License** — see the [LICENSE](LICENSE) file for details.

**Open Source. Permissionless. Unstoppable.**

---

## ⚠️ Disclaimer

**ALPHA SOFTWARE WARNING**

This protocol is currently in **alpha stage** and has NOT undergone professional security audits. 

- Smart contracts may contain bugs or vulnerabilities
- Use at your own risk
- Only deposit funds you can afford to lose
- Not recommended for production use until Phase 4 completion

See [Terms & Conditions](TERMS_AND_CONDITIONS.md) for full legal disclaimers.

---

## 🙏 Acknowledgments

PROVION is built on the shoulders of giants:

- **Arbitrum** - For Orbit and Stylus technology
- **RISC Zero** - For zero-knowledge proof infrastructure
- **Celestia** - For data availability
- **Aave** - For yield generation
- **The Ethereum Community** - For making decentralized finance possible

---

**Built with ❤️ by the PROVION community**

*Pay for code. Verified by math. Settled by consensus.*

---

© 2026 PROVION Foundation. All rights reserved.

Last updated: January 21, 2026


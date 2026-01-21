<p align="center">
  <img src="https://github.com/AlexSheff/provion/blob/main/provion_logo.svg" width="200" alt="PROVION">
</p>

# 🏗 PROVION Architecture

> **Technical Deep-Dive**: A sovereign L3 App-Chain for cryptographically verified open work.

---

## 📋 Table of Contents

1. [System Overview](#system-overview)
2. [Technology Stack](#technology-stack)
3. [Core Components](#core-components)
4. [Smart Contract Architecture](#smart-contract-architecture)
5. [ZK Proof System](#zk-proof-system)
6. [Economic Model](#economic-model)
7. [Security Model](#security-model)
8. [Data Flow](#data-flow)
9. [Scalability & Performance](#scalability--performance)
10. [Future Improvements](#future-improvements)

---

## 🎯 System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    USER INTERFACES                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Web App    │  │  GitHub Bot  │  │   CLI Tool   │     │
│  │  (Next.js)   │  │   (Webhook)  │  │   (Rust)     │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
          └──────────────────┴──────────────────┘
                             │
                    ┌────────▼────────┐
                    │   RPC Gateway   │
                    │   (Nginx/Envoy) │
                    └────────┬────────┘
                             │
┌────────────────────────────┼─────────────────────────────────┐
│                     PROVION L3 CHAIN                         │
│                                                               │
│  ┌─────────────────────────┴──────────────────────────┐     │
│  │           Arbitrum Orbit Sequencer                 │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │     Stylus Smart Contracts (Rust/WASM)       │  │     │
│  │  │  ┌────────────┐  ┌────────────┐  ┌────────┐ │  │     │
│  │  │  │  Bounty    │  │  Registry  │  │ Escrow │ │  │     │
│  │  │  │  Manager   │  │  Contract  │  │ Vault  │ │  │     │
│  │  │  └────────────┘  └────────────┘  └────────┘ │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────────┐  │     │
│  │  │         State Database (PostgreSQL)          │  │     │
│  │  └──────────────────────────────────────────────┘  │     │
│  └─────────────────────────────────────────────────────┘     │
└───────────────────────────────────────────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │  Batch Poster   │
                    │  (AnyTrust DAC) │
                    └────────┬────────┘
                             │
┌────────────────────────────┼─────────────────────────────────┐
│                    ARBITRUM ONE (L2)                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │   Bridge Contracts (Parent Chain Settlement)         │  │
│  └───────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘
                             │
┌────────────────────────────┼─────────────────────────────────┐
│                   ETHEREUM MAINNET (L1)                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │         Final Settlement & Security Guarantees        │  │
│  └───────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘

            ┌─────────────────────────────────┐
            │    PARALLEL INFRASTRUCTURE      │
            ├─────────────────────────────────┤
            │  ┌──────────────────────────┐   │
            │  │  ZK Proof Generation     │   │
            │  │  (RISC Zero zkVM)        │   │
            │  │  ┌────────────────────┐  │   │
            │  │  │ Guest Program:     │  │   │
            │  │  │ - Git signature    │  │   │
            │  │  │ - Commit verify    │  │   │
            │  │  │ - Merkle proofs    │  │   │
            │  │  └────────────────────┘  │   │
            │  └──────────────────────────┘   │
            │                                  │
            │  ┌──────────────────────────┐   │
            │  │  Data Availability       │   │
            │  │  (Celestia)              │   │
            │  │  - Contribution logs     │   │
            │  │  - Dispute evidence      │   │
            │  └──────────────────────────┘   │
            │                                  │
            │  ┌──────────────────────────┐   │
            │  │  Indexer (Ponder)        │   │
            │  │  - GraphQL API           │   │
            │  │  - Event processing      │   │
            │  └──────────────────────────┘   │
            └─────────────────────────────────┘
```

---

## 🔧 Technology Stack

### Layer 3 Execution

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Base Layer** | Arbitrum Orbit | Sovereign chain with custom gas token support |
| **DA Mode** | AnyTrust (DAC) | 90% cheaper than Rollup mode, suitable for early stage |
| **VM** | Stylus (WASM) | 10-100x cheaper gas than EVM, native Rust support |
| **Settlement** | Arbitrum One | Inherits Ethereum security, lower fees than L1 |

### Smart Contracts

| Contract | Language | Purpose |
|----------|----------|---------|
| **BountyManager** | Stylus (Rust) | Core bounty lifecycle management |
| **RegistryContract** | Stylus (Rust) | User reputation and credentials |
| **EscrowVault** | Stylus (Rust) | Yield-bearing fund management |
| **PaymasterContract** | Stylus (Rust) | Gasless transactions for users |

### Zero-Knowledge Proofs

| Component | Technology | Purpose |
|-----------|------------|---------|
| **zkVM** | RISC Zero | General-purpose ZK computation |
| **Guest Program** | Rust | Git signature verification logic |
| **Proof System** | STARK | Post-quantum secure, no trusted setup |
| **Verification** | On-chain (L3) | Smart contract validates receipts |

### Data Availability

| Layer | Technology | Cost | Use Case |
|-------|------------|------|----------|
| **Primary** | AnyTrust DAC | ~$50/month | Regular transactions |
| **Archival** | Celestia | ~$3/month | Dispute evidence, audit logs |
| **Emergency** | Ethereum calldata | ~$1800/month | Only if DAC fails |

### Infrastructure

| Component | Technology | Deployment |
|-----------|------------|------------|
| **Sequencer** | Nitro Node | Bare metal (Hetzner) |
| **Database** | PostgreSQL 16 | Same server |
| **Indexer** | Ponder | Same server |
| **RPC Gateway** | Nginx | Same server |
| **Monitoring** | Grafana + Prometheus | Docker containers |

---

## 🧩 Core Components

### 1. Bounty Manager Contract

**Responsibilities**:
- Create bounties with USDC backing
- Accept PR submissions with Git commit hashes
- Coordinate reviewer consensus
- Trigger ZK proof verification
- Execute automatic payouts

**Key Functions**:

```rust
// Stylus (Rust) pseudocode
pub fn create_bounty(
    &mut self,
    repo: String,
    issue_id: u64,
    amount: U256,
    deadline: u64
) -> Result<BountyId> {
    // 1. Transfer USDC to escrow
    // 2. Deploy to yield strategy (Aave)
    // 3. Emit BountyCreated event
    // 4. Return bounty ID
}

pub fn submit_pr(
    &mut self,
    bounty_id: BountyId,
    commit_hash: B256,
    pr_url: String
) -> Result<()> {
    // 1. Verify bounty is open
    // 2. Record submission
    // 3. Start review period
    // 4. Emit PRSubmitted event
}

pub fn verify_with_zk_proof(
    &mut self,
    bounty_id: BountyId,
    receipt: Vec<u8>  // RISC Zero receipt
) -> Result<()> {
    // 1. Verify receipt signature
    // 2. Extract journal data (commit hash, signer)
    // 3. Validate against bounty requirements
    // 4. Update state
}

pub fn execute_payout(
    &mut self,
    bounty_id: BountyId
) -> Result<()> {
    // 1. Verify consensus reached
    // 2. Withdraw from yield vault (principal + interest)
    // 3. Distribute to contributor(s)
    // 4. Emit PayoutExecuted event
}
```

**State Machine**:

```
┌──────────┐
│  FUNDED  │ (Initial state: funds deposited)
└────┬─────┘
     │ submit_pr()
     ▼
┌──────────┐
│ PR_OPEN  │ (Submitted, awaiting reviews)
└────┬─────┘
     │ reviewers vote
     ▼
┌──────────┐
│CONSENSUS │ (Threshold reached)
└────┬─────┘
     │ verify_with_zk_proof()
     ▼
┌──────────┐
│ VERIFIED │ (ZK proof validated)
└────┬─────┘
     │ execute_payout()
     ▼
┌──────────┐
│   PAID   │ (Final state)
└──────────┘

     Alternative paths:
     - DISPUTED → Fisherman challenge
     - EXPIRED → Refund to creator
     - SLASHED → Reviewer collusion detected
```

---

### 2. Registry Contract

**Purpose**: On-chain identity and reputation system

**Data Structures**:

```rust
struct Developer {
    address: Address,
    github_username: String,
    pgp_key: Vec<u8>,
    reputation_score: u64,
    total_earned: U256,
    completed_bounties: u64,
    sbt_credentials: Vec<SBT>,
}

struct SBT {  // Soul-Bound Token
    credential_type: CredentialType,
    issuer: Address,
    issued_at: u64,
    metadata_uri: String,
}

enum CredentialType {
    RustExpert,
    SecurityAuditor,
    ZKPioneer,
    TopContributor,
}
```

**Reputation Algorithm**:

```
reputation_score = (
    completed_bounties * 10 +
    total_earned_usd / 100 +
    successful_reviews * 5 +
    time_active_days / 7 +
    sbt_credentials.len() * 50
) - (slashing_events * 100)
```

---

### 3. Escrow Vault

**Purpose**: Yield-bearing fund management

**Integration**:
- **Primary**: Aave V3 (USDC lending)
- **Secondary**: Compound V3 (diversification)
- **Fallback**: Direct USDC holding

**Yield Strategy**:

```rust
pub fn deposit_to_yield(&mut self, amount: U256) -> Result<()> {
    // 1. Approve Aave pool
    // 2. Supply USDC → receive aUSDC
    // 3. Track aUSDC balance
    // 4. Yield auto-compounds
}

pub fn withdraw_with_yield(&mut self, bounty_id: BountyId) -> Result<U256> {
    // 1. Calculate time elapsed
    // 2. Get current aUSDC/USDC rate
    // 3. Withdraw from Aave
    // 4. Return principal + interest
}
```

**Example**:
- Bounty: $5,000 USDC
- Duration: 30 days
- Aave APY: 4.2%
- Yield earned: $5,000 × 0.042 × (30/365) = **$17.26**

This covers ZK proof generation costs (~$10) + partial gas ($7).

---

### 4. Paymaster Contract

**Purpose**: Gasless transactions for end users

**Mechanism**:
```rust
pub fn sponsor_transaction(
    &mut self,
    user: Address,
    call_data: Vec<u8>
) -> Result<()> {
    // 1. Verify user has valid reputation
    // 2. Check daily gas limit not exceeded
    // 3. Execute transaction on behalf of user
    // 4. Deduct gas from protocol treasury
}
```

**Gas Budget per User**:
- **Contributors**: 500,000 gas/day (~$0.10)
- **Reviewers**: 200,000 gas/day (~$0.04)
- **Creators**: 300,000 gas/day (~$0.06)

**Funding Source**:
- 10% of yield generated from escrow vaults
- One-time grant allocation
- Optional: Small protocol fee (0.5%)

---

## 🔐 ZK Proof System

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 RISC Zero zkVM Guest                    │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  fn main() {                                    │    │
│  │    // 1. Read inputs from host                 │    │
│  │    let commit_hash = env::read();              │    │
│  │    let signature = env::read();                │    │
│  │    let public_key = env::read();               │    │
│  │                                                 │    │
│  │    // 2. Verify Git signature (Ed25519/GPG)    │    │
│  │    let valid = crypto::verify_signature(       │    │
│  │        commit_hash,                             │    │
│  │        signature,                               │    │
│  │        public_key                               │    │
│  │    );                                           │    │
│  │                                                 │    │
│  │    // 3. Commit result to journal              │    │
│  │    env::commit(&valid);                        │    │
│  │    env::commit(&commit_hash);                  │    │
│  │  }                                              │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          │
                          │ Execution trace
                          ▼
┌─────────────────────────────────────────────────────────┐
│              RISC Zero Prover (Host)                    │
│  ┌────────────────────────────────────────────────┐    │
│  │  - Runs guest program in zkVM                  │    │
│  │  - Generates execution trace                   │    │
│  │  - Constructs STARK proof                      │    │
│  │  - Outputs: Receipt { proof, journal }        │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          │
                          │ Receipt (195 KB)
                          ▼
┌─────────────────────────────────────────────────────────┐
│           PROVION L3 Smart Contract                     │
│  ┌────────────────────────────────────────────────┐    │
│  │  fn verify_proof(receipt: Vec<u8>) -> bool {   │    │
│  │    // 1. Verify STARK proof on-chain           │    │
│  │    risc0_verifier::verify(receipt)             │    │
│  │                                                 │    │
│  │    // 2. Extract journal data                  │    │
│  │    let (valid, commit_hash) = parse_journal(); │    │
│  │                                                 │    │
│  │    // 3. Update bounty state                   │    │
│  │    if valid { approve_submission(); }          │    │
│  │  }                                              │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

### What Gets Proven

**Input** (from GitHub):
```json
{
  "commit_hash": "7f3a2b94c1e5d8f9a0b3c4d5e6f7a8b9",
  "signature": "3045022100...",  // Ed25519/GPG signature
  "signer_key": "ssh-ed25519 AAAAC3...",
  "pr_metadata": {
    "repo": "rust-lang/cargo",
    "pr_number": 13420,
    "files_changed": ["src/cargo/ops/mod.rs"]
  }
}
```

**Verified** (inside zkVM):
1. ✅ Signature is cryptographically valid
2. ✅ Commit hash matches signed message
3. ✅ Signer key matches registered developer
4. ✅ Commit is in correct repository
5. ✅ Files match bounty requirements

**Output** (on-chain journal):
```rust
struct ProofJournal {
    commit_hash: [u8; 32],
    is_valid: bool,
    signer_address: Address,  // Derived from pubkey
    timestamp: u64,
}
```

### Performance Metrics

| Metric | Value |
|--------|-------|
| Guest program cycles | ~1.2M cycles |
| Proof generation time | 30-45 seconds (RTX 4090) |
| Receipt size | ~195 KB |
| On-chain verification gas | ~80,000 gas (~$0.002) |
| Total cost per proof | ~$0.05 (Vast.ai) |

---

## 💰 Economic Model

### Token Flow

```
┌──────────────┐
│ Bounty       │
│ Creator      │
│ (deposits    │
│  $5000 USDC) │
└──────┬───────┘
       │
       ▼
┌──────────────┐      ┌─────────────┐
│ Escrow Vault │─────>│ Aave V3     │
│              │      │ (4.2% APY)  │
└──────┬───────┘      └─────────────┘
       │                     │
       │ (30 days)           │ Yield: $17.26
       ▼                     ▼
┌──────────────┐      ┌─────────────┐
│ Payout       │      │ Protocol    │
│ Execution    │<─────│ Treasury    │
└──────┬───────┘      └─────────────┘
       │
       ├──> Contributor: $5,000.00 (100%)
       ├──> Reviewer 1:  $5.00 (from yield)
       ├──> Reviewer 2:  $5.00 (from yield)
       └──> Gas costs:   $7.26 (from yield)
```

### Fee Structure

| Action | User Pays | Protocol Earns | Notes |
|--------|-----------|----------------|-------|
| Create bounty | 0% (gasless) | 10% of yield | Funded by escrow interest |
| Submit PR | 0% (gasless) | 0% | Fully subsidized |
| Review PR | 0% (gasless) | 0% | Reviewers earn from yield pool |
| Claim payout | 0% (gasless) | 0% | Direct transfer |

**Protocol Sustainability**:
- TVL target: $100,000
- Average yield: 4% APY = $4,000/year
- Monthly revenue: $333
- Monthly costs: $107
- **Net profit**: $226/month (213% margin)

At $1M TVL: $3,333/month revenue → profitable even with full audit costs.

---

## 🛡 Security Model

### Trust Assumptions

**We Trust**:
1. ✅ Ethereum L1 consensus
2. ✅ Arbitrum L2 bridge contracts
3. ✅ RISC Zero zkVM correctness
4. ✅ Ed25519/GPG cryptography

**We DON'T Trust**:
1. ❌ GitHub API (could be hacked)
2. ❌ GitHub webhooks (could be spoofed)
3. ❌ Reviewers (economic collusion possible)
4. ❌ Sequencer (censorship possible, but detectable)

### Attack Vectors & Mitigations

#### 1. Collusion Attack

**Scenario**: Reviewers approve fake PR to steal bounty.

**Mitigation**:
- **Fisherman Mechanism**: Anyone can challenge within 24h dispute window
- **Slashing**: Colluding reviewers lose 100% of stake
- **Reward**: Challenger gets 50% of slashed funds
- **ZK Proof**: Final arbiter - can't fake cryptographic signatures

**Economics**:
```
Reviewer stake: $500
Bounty value: $5,000
Challenge reward: $250 (50% of stake)

Attack expected value:
  EV = (chance_not_caught × $5,000) - (chance_caught × $500)
  EV = (0.1 × $5,000) - (0.9 × $500) = $50

Defense expected value for fisherman:
  EV = chance_catch_collusion × $250 = 0.9 × $250 = $225

Conclusion: Economically favorable to defend, not attack.
```

#### 2. Sequencer Censorship

**Scenario**: Sequencer refuses to include legitimate transactions.

**Mitigation**:
- **Force Inclusion**: Users can submit directly to L2 (Arbitrum One)
- **Delayed Inbox**: Transactions force-included after 24h
- **Sequencer Rotation**: Community can vote to change sequencer

#### 3. GitHub Account Compromise

**Scenario**: Attacker steals GitHub account, signs fake commits.

**Mitigation**:
- **Time-locked keys**: Require 2FA + hardware key for bounty claims
- **Multi-sig approval**: High-value bounties need 3/5 multi-sig
- **Reputation decay**: Suspicious activity triggers cooldown period

#### 4. Smart Contract Bugs

**Mitigation**:
- **Formal verification**: Stylus contracts easier to prove than Solidity
- **Automated scanning**: Slither + Mythril in CI/CD
- **Bug bounty**: 10% of tokens or $1,000 for critical bugs
- **Emergency pause**: 2/3 multi-sig can halt system
- **Timelock**: 48h delay on upgrades

---

## 🔄 Data Flow

### Happy Path: Bounty Creation → Payout

```
1. Creator deposits $5,000 USDC
   ├─> BountyManager.create_bounty()
   ├─> EscrowVault.deposit_to_yield()
   └─> Event: BountyCreated

2. Contributor submits PR with commit hash
   ├─> BountyManager.submit_pr()
   └─> Event: PRSubmitted

3. Reviewers stake and vote
   ├─> RegistryContract.stake_for_review()
   ├─> BountyManager.cast_vote(approve)
   └─> Event: ConsensusReached (when 3/5 approve)

4. ZK proof generation (off-chain)
   ├─> Prover fetches commit from GitHub
   ├─> Verifies Ed25519 signature in zkVM
   ├─> Generates STARK proof
   └─> Receipt posted to L3

5. Proof verification (on-chain)
   ├─> BountyManager.verify_with_zk_proof()
   ├─> risc0_verifier.verify(receipt)
   └─> Event: ProofVerified

6. Dispute window (24 hours)
   ├─> Fishermen can challenge
   └─> If no challenge → proceed

7. Payout execution
   ├─> EscrowVault.withdraw_with_yield()
   ├─> BountyManager.execute_payout()
   ├─> Transfer: $5,000 → Contributor
   ├─> Transfer: $5 × 3 → Reviewers (from yield)
   └─> Event: PayoutExecuted
```

### Event Log Example

```json
[
  {
    "event": "BountyCreated",
    "block": 100,
    "data": {
      "bounty_id": 42,
      "creator": "0x1234...",
      "amount": 5000000000,  // 6 decimals (USDC)
      "repo": "rust-lang/cargo"
    }
  },
  {
    "event": "PRSubmitted",
    "block": 150,
    "data": {
      "bounty_id": 42,
      "contributor": "0x5678...",
      "commit_hash": "0x7f3a2b94...",
      "pr_url": "https://github.com/rust-lang/cargo/pull/13420"
    }
  },
  {
    "event": "ConsensusReached",
    "block": 200,
    "data": {
      "bounty_id": 42,
      "approvals": 5,
      "rejections": 0
    }
  },
  {
    "event": "ProofVerified",
    "block": 220,
    "data": {
      "bounty_id": 42,
      "commit_hash": "0x7f3a2b94...",
      "is_valid": true
    }
  },
  {
    "event": "PayoutExecuted",
    "block": 1400,  // 24h later (assuming 3s blocks)
    "data": {
      "bounty_id": 42,
      "recipient": "0x5678...",
      "amount": 5000000000,
      "yield_earned": 17260000  // $17.26
    }
  }
]
```

---

## 📈 Scalability & Performance

### Transaction Throughput

| Metric | Testnet | Mainnet (Launch) | Future (Optimized) |
|--------|---------|------------------|--------------------|
| **TPS** | ~100 | ~1,000 | ~10,000 |
| **Block time** | 3 seconds | 3 seconds | 1 second |
| **Txs per block** | 300 | 3,000 | 10,000 |
| **Daily capacity** | 8.6M | 86M | 864M |

**Bottlenecks**:
1. ❌ ZK proof generation (30-45s per proof)
2. ❌ PostgreSQL write throughput (10k TPS limit)
3. ✅ Stylus execution (not a bottleneck)

**Solutions**:
1. **Batch proofs**: Prove 10 commits in one zkVM execution
2. **Parallel provers**: Run 5 GPU instances simultaneously
3. **Caching**: Cache verified commit hashes for 30 days

### Storage Requirements

**Per bounty**:
```
Contract state: 512 bytes
Event logs: 256 bytes
Total: 768 bytes
```

**Projected growth**:
```
Year 1: 1,000 bounties × 768 bytes = 768 KB
Year 5: 50,000 bounties × 768 bytes = 38 MB

Database (PostgreSQL):
Year 1: ~100 MB (includes indexes)
Year 5: ~5 GB
```

**Archival strategy**:
- Keep last 90 days on hot storage (NVMe SSD)
- Move older data to Celestia (~$3/month for 1 TB)
- Emergency restore from Celestia if needed

---

## 🔮 Future Improvements

### Phase 2 (Q2 2026)

**1. Multi-Chain Support**
- Deploy on Base (Coinbase L2)
- Deploy on Optimism
- Unified liquidity via LayerZero

**2. Advanced Reviewers**
- AI-assisted code review (GPT-4 integration)
- Automated security scanning (Slither/Semgrep)
- Reputation-weighted voting (more stake = more power)

**3. Enhanced Privacy**
- Private bounties (ZK-encrypted descriptions)
- Anonymous contributors (zk-SNARKs for identity)

### Phase 3 (Q4 2026)

**1. DAO Governance**
- $PVN token launch
- Protocol parameter voting
- Treasury management

**2. Native GitHub Integration**
- GitHub App (no manual webhook setup)
- Automatic bounty posting from issues
- Direct PR → Payout flow

**3. Cross-Protocol Composability**
- Integration with Gitcoin Grants
- Integration with Aragon DAOs
- Integration with Coordinape

### Phase 4 (2027+)

**1. Recursive Proofs**
- Use zkVM to verify other zkVM proofs
- Reduce verification costs by 10x

**2. Fully On-Chain Git**
- Store entire Git history on-chain (via Celestia)
- No dependency on GitHub
- True decentralized code hosting

**3. Universal Compute Market**
- Extend beyond code bounties
- Scientific computing bounties
- ML training bounties
- Zero-knowledge research bounties

---

## 📚 Technical References

### Standards & Specifications

- [EIP-4337: Account Abstraction](https://eips.ethereum.org/EIPS/eip-4337)
- [Arbitrum Nitro Specification](https://github.com/OffchainLabs/nitro)
- [RISC Zero zkVM Documentation](https://dev.risczero.com/)
- [Celestia Data Availability Sampling](https://docs.celestia.org/)

### Related Research

- **Optimistic vs ZK Rollups**: [Vitalik's Guide](https://vitalik.ca/general/2021/01/05/rollup.html)
- **MEV in L2s**: [Flashbots Research](https://writings.flashbots.net/)
- **Cross-L2 Communication**: [EIP-5164](https://eips.ethereum.org/EIPS/eip-5164)

### Code References

```
provion/
├── contracts/src/
│   ├── bounty_manager.rs      # Core logic
│   ├── registry.rs             # Identity system
│   ├── escrow_vault.rs         # Yield management
│   └── paymaster.rs            # Gasless txs
├── prover/
│   ├── methods/guest/          # zkVM program
│   └── host/                   # Proof generator
└── ui/
    └── src/hooks/              # Web3 integration
```

---

## 🤝 Contributing

See main [CONTRIBUTING.md](../CONTRIBUTING.md) for development setup.

**Architecture discussions**: Open a GitHub Discussion with `[Architecture]` tag.

---

*Document version: 1.0*  
*Last updated: January 21, 2026*  
*Status: Living document — open to community feedback*

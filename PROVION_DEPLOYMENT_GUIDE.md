<p align="center">
  <img src="https://github.com/AlexSheff/provion/blob/main/provion_logo.svg" width="200" alt="PROVION">
</p>

# 🚀 PROVION: Bootstrap Strategy — Lean & Mean Edition

## 💡 Philosophy: From $283k/year to $1,284/year

**Core Principle**: Indie Hacker approach instead of Enterprise burn rate.  
**Goal**: Working mainnet for the cost of a Netflix subscription.

---

## 📋 Table of Contents

1. [Extreme Cost Optimization](#extreme-cost-optimization)
2. [Lean Stack Architecture](#lean-stack-architecture)
3. [Step-by-Step Bootstrap Guide](#step-by-step-bootstrap-guide)
4. [Grants Strategy: $0 Out of Pocket](#grants-strategy-0-out-of-pocket)
5. [Production Checklist](#production-checklist)

---

## 💰 Extreme Cost Optimization

### Comparison: Enterprise vs Bootstrap

| Component | ❌ Enterprise | ✅ Bootstrap | Savings |
|-----------|---------------|--------------|---------|
| **Servers** | AWS c6i.2xlarge × 2 | Hetzner AX41 × 1 | $500 → $45 |
| **ZK Proofs** | Boundless SaaS | Vast.ai on-demand | $1,500 → $10 |
| **L1 Gas** | Standard Rollup | AnyTrust + Lazy Batching | $1,800 → $50 |
| **Frontend** | Vercel Pro | Vercel Free | $20 → $0 |
| **RPC** | Alchemy Growth | Self-hosted | $200 → $0 |
| **Monitoring** | Grafana Cloud | Self-hosted Grafana | $50 → $0 |
| **Audit** | Trail of Bits | Community + Slither | $60k → $0* |
| **Legal** | Swiss Foundation | Disclaimer only | $30k → $0* |
| **Team** | DevOps engineer | Founders (sweat equity) | $3k → $0 |
| **TOTAL/month** | **$15,073** | **$107** | **-99.3%** |
| **TOTAL/year** | **$283k** | **$1,284** | **$281,716** |

*Deferred until product-market fit

---

## 🏗 Lean Stack Architecture

```
┌─────────────────────────────────────────┐
│   PROVION L3 (Arbitrum Orbit)          │
│   Mode: AnyTrust (DAC)                 │
│   Batching: Lazy (24h or 100KB)       │
└─────────────────────────────────────────┘
              ↓ minimal settlement
┌─────────────────────────────────────────┐
│   Arbitrum One (L2)                     │
│   Gas: ~$50/month                       │
└─────────────────────────────────────────┘

   Infrastructure (all on one server):
┌─────────────────────────────────────────┐
│   Hetzner AX41-NVMe (€40/month)        │
│   ├── Nitro Node (Sequencer)           │
│   ├── PostgreSQL (State DB)            │
│   ├── Ponder (Indexer)                 │
│   ├── Nginx (RPC Gateway)              │
│   └── Grafana + Prometheus             │
└─────────────────────────────────────────┘

   ZK Proofs (on-demand):
┌─────────────────────────────────────────┐
│   Vast.ai / RunPod                     │
│   GPU: RTX 4090 @ $0.35/hour           │
│   Usage: ~30 hours/month                │
│   Cost: $10/month                       │
└─────────────────────────────────────────┘
```

---

## 🛠 Step-by-Step Bootstrap Guide

### Stage 1: Environment Setup (Day 1)

#### 1.1. Install Tools

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Cargo Stylus
cargo install --force cargo-stylus

# RISC Zero
curl -L https://risczero.com/install | bash
rzup install

# Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Node.js for frontend
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
```

#### 1.2. Clone Repository

```bash
git clone https://github.com/AlexSheff/provion.git
cd provion
```

---

### Stage 2: Rent Bare Metal Server (Day 1)

#### 2.1. Hetzner Server Auction

1. Register at https://www.hetzner.com/sb
2. Select server:
   - **Model**: AX41-NVMe or AX51-NVMe
   - **Specs**: 6-8 cores, 64GB RAM, 2×512GB NVMe
   - **Price**: €39-49/month (~$45-55)
   - **Location**: Falkenstein (Germany) or Helsinki (Finland)

#### 2.2. Hetzner Alternatives

| Provider | Model | Specs | Price/month |
|----------|-------|-------|-------------|
| **OVH** | Rise-1 | Ryzen 5600X, 32GB, 2×512GB NVMe | €45 |
| **Contabo** | VPS XL | 10 vCPU, 60GB RAM, 1.6TB SSD | €30 |
| **Scaleway** | GP1-L | 8 vCPU, 32GB RAM, 600GB NVMe | €50 |

**Recommendation**: Start with Hetzner AX41 — best price/reliability balance.

#### 2.3. Initial Server Setup

```bash
# SSH to server
ssh root@YOUR_SERVER_IP

# Update system
apt update && apt upgrade -y

# Install essentials
apt install -y curl git build-essential ufw fail2ban

# Configure firewall
ufw allow 22/tcp   # SSH
ufw allow 80/tcp   # HTTP
ufw allow 443/tcp  # HTTPS
ufw allow 8547/tcp # L3 RPC
ufw enable

# Create swap (important for ZK proofs)
fallocate -l 32G /swapfile
chmod 600 /swapfile
mkswap /swapfile
swapon /swapfile
echo '/swapfile none swap sw 0 0' | tee -a /etc/fstab
```

---

### Stage 3: Local Development on DevNet (Week 1-2)

#### 3.1. Launch Local Arbitrum Nitro

```bash
# Clone testnode
git clone https://github.com/OffchainLabs/nitro-testnode
cd nitro-testnode

# Start
./test-node.bash --init

# Verify
curl http://localhost:8547 -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

#### 3.2. Compile and Deploy Stylus Contracts

```bash
cd ../provion/contracts

# Check WASM
cargo stylus check

# Deploy to local devnet
cargo stylus deploy \
  --endpoint=http://localhost:8547 \
  --private-key=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

# Save contract address!
# Output: Deployed at: 0x...
```

#### 3.3. ZK Prover in Dev Mode (no GPU)

```bash
cd ../prover

# Dev mode (no real proof generation)
RISC0_DEV_MODE=1 cargo run --release --bin host

# Should execute in 1-2 seconds
# Output: ✅ Mock proof generated
```

#### 3.4. Launch UI

```bash
cd ../ui

# Install dependencies
yarn install

# Configure .env.local
cat > .env.local << EOF
NEXT_PUBLIC_CHAIN_ID=412346
NEXT_PUBLIC_RPC_URL=http://localhost:8547
NEXT_PUBLIC_CONTRACT_ADDRESS=0x... # address from step 3.2
EOF

# Start dev server
yarn dev

# UI available at http://localhost:3000
```

**Verification**: Create test bounty via UI, ensure transactions go through.

---

### Stage 4: Deploy to Testnet (Week 3-4)

#### 4.1. Get Testnet ETH

```bash
# Arbitrum Sepolia Faucet
# https://faucet.arbitrum.io/

# Need: 0.5 testnet ETH for Orbit chain deployment
# + 0.1 testnet ETH for contract deployment
```

#### 4.2. Deploy Orbit Chain (AnyTrust)

1. Go to https://orbit.arbitrum.io/
2. Connect Wallet
3. Configuration:
   - **Chain Type**: AnyTrust (cheaper!)
   - **Base Chain**: Arbitrum Sepolia
   - **Data Availability**: AnyTrust DAC
   - **Gas Token**: ETH (default)
4. Deploy (spend ~0.4 testnet ETH)
5. Download `nodeConfig.json` and `orbitSetupScriptConfig.json`

#### 4.3. Launch L3 Node on Hetzner Server

```bash
# On your Hetzner server
ssh root@YOUR_SERVER_IP

# Clone Orbit setup script
git clone https://github.com/OffchainLabs/orbit-setup-script
cd orbit-setup-script

# Copy configs
scp nodeConfig.json root@YOUR_SERVER_IP:~/orbit-setup-script/config/
scp orbitSetupScriptConfig.json root@YOUR_SERVER_IP:~/orbit-setup-script/config/

# Install dependencies
apt install -y nodejs npm
npm install -g yarn
yarn install

# Launch L3 Sequencer
yarn run setup

# Verify
curl http://localhost:8449 -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

#### 4.4. Configure systemd for Auto-start

```bash
cat > /etc/systemd/system/provion-sequencer.service << EOF
[Unit]
Description=PROVION L3 Sequencer
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/orbit-setup-script
ExecStart=/usr/bin/yarn start
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable provion-sequencer
systemctl start provion-sequencer
systemctl status provion-sequencer
```

#### 4.5. Deploy PROVION Contracts on L3

```bash
# On local machine
cd provion/contracts

cargo stylus deploy \
  --endpoint=http://YOUR_SERVER_IP:8449 \
  --private-key=$YOUR_DEPLOYER_KEY

# Save contract addresses
```

---

### Stage 5: Setup ZK Proofs on Vast.ai (Week 4)

#### 5.1. Register on Vast.ai

1. https://vast.ai/ → Sign Up
2. Top-up balance: $10 (enough for a month)

#### 5.2. Create Script for On-Demand Proofs

```bash
cd provion/prover

# Create wrapper for remote generation
cat > scripts/vast_proof.sh << 'EOF'
#!/bin/bash
# Launches Docker container on Vast.ai for proof generation

COMMIT_HASH=$1
SIGNATURE=$2

vastai create instance \
  --image risczero/risczero:latest \
  --disk 50 \
  --gpu "RTX_4090" \
  --env COMMIT_HASH=$COMMIT_HASH \
  --env SIGNATURE=$SIGNATURE \
  --on-demand \
  bash -c "cd /app && cargo run --release --bin host"

# After completion instance auto-deletes
EOF

chmod +x scripts/vast_proof.sh
```

#### 5.3. Alternative: Gaming PC at Home

If you have a gaming PC with RTX 3080/4090:

```bash
# On home PC install Cloudflare Tunnel
wget https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
sudo dpkg -i cloudflared-linux-amd64.deb

# Authenticate
cloudflared tunnel login

# Create tunnel
cloudflared tunnel create provion-prover

# Start
cloudflared tunnel --url http://localhost:3000 run provion-prover
```

Now your home GPU is accessible via Cloudflare.  
**Cost**: $0 (only electricity ~$15/month)

---

### Stage 6: Optimize Gas Costs (Week 5)

#### 6.1. Enable AnyTrust Mode

In `nodeConfig.json`:

```json
{
  "chain": {
    "info-json": "[{\"chain-id\":412346,\"parent-chain-id\":421614,\"parent-chain-is-arbitrum\":true,\"chain-name\":\"PROVION\",\"chain-config\":{\"homesteadBlock\":0,\"daoForkBlock\":null,\"daoForkSupport\":true,\"eip150Block\":0,\"eip150Hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"eip155Block\":0,\"eip158Block\":0,\"byzantiumBlock\":0,\"constantinopleBlock\":0,\"petersburgBlock\":0,\"istanbulBlock\":0,\"muirGlacierBlock\":0,\"berlinBlock\":0,\"londonBlock\":0,\"clique\":{\"period\":0,\"epoch\":0},\"arbitrum\":{\"EnableArbOS\":true,\"AllowDebugPrecompiles\":false,\"DataAvailabilityCommittee\":true,\"InitialArbOSVersion\":11,\"InitialChainOwner\":\"0x...\",\"GenesisBlockNum\":0}},\"rollup\":{\"bridge\":\"0x...\",\"inbox\":\"0x...\",\"sequencer-inbox\":\"0x...\",\"rollup\":\"0x...\",\"validator-utils\":\"0x...\",\"validator-wallet-creator\":\"0x...\",\"deployed-at\":123456}}]",
    "name": "provion"
  }
}
```

**Key parameter**: `"DataAvailabilityCommittee": true`

#### 6.2. Configure Lazy Batching

Add to sequencer config:

```json
{
  "node": {
    "batch-poster": {
      "enable": true,
      "max-interval": "24h",        // Batch once per day
      "max-size": 102400,           // Or when reaching 100 KB
      "compression-level": 11       // Maximum compression
    }
  }
}
```

**Result**: Gas costs drop from $1,800/month to $50-100/month.

---

### Stage 7: Frontend on Vercel Free (Week 5)

#### 7.1. Deploy UI to Vercel

```bash
cd provion/ui

# Install Vercel CLI
npm i -g vercel

# Deploy
vercel --prod

# Vercel automatically:
# - Creates production URL (provion.vercel.app)
# - Sets up CI/CD with GitHub
# - Issues SSL certificate
```

#### 7.2. Custom Domain (optional)

```bash
# Buy domain on Namecheap/Cloudflare: $2-10/year
# In Vercel: Settings → Domains → Add domain

# DNS setup:
# CNAME: @ → cname.vercel-dns.com
# CNAME: www → cname.vercel-dns.com
```

**Cost**: $0 (Vercel Free Plan up to 100GB bandwidth)

---

### Stage 8: Self-hosted Indexer (Week 6)

Instead of The Graph (expensive), use Ponder (free):

```bash
# On Hetzner server
cd /opt
git clone https://github.com/ponder-sh/ponder
cd ponder

# Install
npm install

# Configure ponder.config.ts
cat > ponder.config.ts << 'EOF'
import { createConfig } from "@ponder/core";
import { http } from "viem";

export default createConfig({
  networks: {
    provionL3: {
      chainId: 412346,
      transport: http("http://localhost:8449")
    }
  },
  contracts: {
    BountyRegistry: {
      network: "provionL3",
      abi: "./abis/BountyRegistry.json",
      address: "0x...",  // Your contract
      startBlock: 0
    }
  }
});
EOF

# Launch
npm run dev

# Ponder GraphQL available at http://localhost:42069
```

Configure systemd similar to sequencer for auto-start.

---

## 💸 Grants Strategy: $0 Out of Pocket

### Grant 1: Arbitrum Foundation ($5,000-10,000)

**Application**:

1. https://arbitrumfoundation.io/grants
2. Select category: "Developer Tooling" or "Public Goods"
3. Fill form:
   - **Project**: PROVION - Autonomous Meritocracy Protocol
   - **Description**: Cryptographically verified open work on Arbitrum Orbit + Stylus
   - **Requested**: $8,000
   - **Breakdown**:
     - Server hosting (12 months): $600
     - Domain & CDN: $50
     - Gas costs for settlement: $1,200
     - Community incentives: $6,150
4. Attach:
   - GitHub repo
   - Whitepaper
   - Working testnet demo
   - Architecture diagram

**Approval probability**: High (~70%) if you have:
- Real working product
- Using Stylus/Orbit
- Active GitHub commits

**Timeline**: 4-8 weeks for review.

---

### Grant 2: Gitcoin Grants Round ($500-2,000)

**Preparation**:

1. https://gitcoin.co/ → Create Project
2. Category: "Open Source" or "dApps & Apps"
3. Project description + links
4. Add Milestones:
   - ✅ Testnet launch
   - ✅ Smart contracts open-sourced
   - 🔄 Mainnet deployment (in progress)
   - 📋 First 100 bounties funded

**Promotion**:
- Tweet with video demo
- Post in /r/ethereum and /r/arbitrum
- Post in Arbitrum Discord #showcase

**Expected result**: $500-2,000 depending on round.

---

### Grant 3: Google Cloud Startup Credits (Up to $100,000)

**Requirements**:
- Incorporation (can do Delaware C-Corp via Stripe Atlas for $500)
- Public GitHub repo
- Working MVP

**Application**:
1. https://cloud.google.com/startup
2. Fill application
3. If approved — get $100,000 credits for 2 years

**Use case**: If you get credits, can move back to GCP/AWS for free.

---

### Grant 4: Ethereum Foundation ($10,000-50,000)

**Categories**:
- Layer 2 Community and Adoption
- Zero Knowledge Proofs

**Requirements**:
- Detailed technical proposal
- Team background
- Open-source commitment

**Application**: https://esp.ethereum.foundation/applicants

**Timeline**: 2-4 months.

---

## 📊 Final Bootstrap Budget

| Item | Solution | Cost/month |
|------|----------|------------|
| **Server** | Hetzner AX41-NVMe | $45 |
| **ZK Proofs** | Vast.ai on-demand (RTX 4090) | $10 |
| **L1 Gas** | AnyTrust + Lazy Batching | $50 |
| **Domain** | Cloudflare | $2 |
| **Frontend** | Vercel Free | $0 |
| **Indexer** | Self-hosted Ponder | $0 |
| **Monitoring** | Self-hosted Grafana | $0 |
| **RPC** | Self-hosted on Hetzner | $0 |
| **Team** | Founders (sweat equity) | $0 |
| **TOTAL** | | **$107/month** |

### Annual cost: $1,284

### With grants: **$0** (fully covered)

**Possible grants**:
- Arbitrum Foundation: $8,000 ✅
- Gitcoin: $1,000 ✅
- Google Cloud: $100,000 (credits) ✅

**Total runway**: 5+ years without additional investment.

---

## ✅ Production Checklist

### Security (DIY approach)

- [ ] **Automated scanning**:
  ```bash
  # Add to GitHub Actions
  - name: Slither
    run: slither contracts/
  
  - name: Mythril
    run: myth analyze contracts/src/lib.rs
  ```

- [ ] **Community Bug Bounty**:
  - Create `SECURITY.md` file in repo
  - Offer: "Critical bug = 10% of tokens or $1,000"
  - Post-factum payment (only if bug is real)

- [ ] **Multi-sig for admin functions**:
  - Safe (Gnosis Safe) on your L3
  - 2/3 threshold
  - Founders + trusted advisor

- [ ] **Timelock on upgrades**:
  - 48 hours delay
  - Allows community to notice malicious changes

- [ ] **Emergency Pause**:
  - Only for critical bugs
  - Requires 2/3 multi-sig

### Monitoring

```bash
# On Hetzner server
docker run -d \
  --name=grafana \
  -p 3001:3000 \
  grafana/grafana

docker run -d \
  --name=prometheus \
  -p 9090:9090 \
  prom/prometheus

# Setup alerts in Grafana:
# - Sequencer down > 5 min → Email
# - Gas balance < 0.1 ETH → Email
# - L2 batch posting fail → Critical
```

### Backups

```bash
# Daily state snapshot
crontab -e

# Add:
0 2 * * * tar -czf /backup/provion-$(date +\%Y\%m\%d).tar.gz /root/orbit-setup-script/data
0 3 * * * rclone copy /backup/ remote:provion-backups/
```

**Rclone**: Free backup to Google Drive / Backblaze B2 (10GB free).

### Disaster Recovery

**RTO** (Recovery Time): < 2 hours  
**RPO** (Recovery Point): < 1 hour

**Plan**:
1. Keep second Hetzner server in "cold standby" (not paid until started)
2. When primary fails:
   - Restore latest snapshot
   - Start on backup server
   - Switch DNS (Cloudflare)

**Cost**: $0 (backup server only paid when activated)

---

## 🎯 Roadmap with Bootstrap Budget

### Month 1-2: MVP Development ($0)
- [ ] Local devnet
- [ ] Stylus contracts
- [ ] ZK prover in dev mode
- [ ] Basic UI

### Month 3-4: Testnet Launch ($200)
- [ ] Hetzner server (2 months × $45)
- [ ] Arbitrum Sepolia deployment
- [ ] Public testnet
- [ ] Documentation

### Month 5-6: Grant Applications ($100)
- [ ] Arbitrum Foundation grant → $8,000 ✅
- [ ] Gitcoin round participation → $1,000 ✅
- [ ] Community building
- [ ] First external contributors

### Month 7-8: Mainnet Preparation ($214)
- [ ] Security hardening
- [ ] Audit automation (Slither/Mythril)
- [ ] Stress testing
- [ ] Bug bounty program

### Month 9: Mainnet Genesis ($0)
- [ ] Deploy on Arbitrum One (covered by grant)
- [ ] First 10 bounties (from grant funds)
- [ ] Marketing push
- [ ] Token launch (optional)

### Month 10-12: Growth ($0)
- [ ] Scale to 100+ bounties
- [ ] Integrate with GitHub Apps
- [ ] Expand to other chains (Base, Optimism)

---

## 📦 Quick Start Commands

### Full setup in 1 hour:

```bash
# 1. Clone
git clone https://github.com/AlexSheff/provion.git
cd provion

# 2. Local devnet
git clone https://github.com/OffchainLabs/nitro-testnode
cd nitro-testnode
./test-node.bash --init &

# 3. Contracts
cd ../provion/contracts
cargo stylus deploy --endpoint=http://localhost:8547 --private-key=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

# 4. UI
cd ../ui
yarn install
yarn dev

# Done! http://localhost:3000
```

---

## 🚨 Important Warnings

### ⚠️ What NOT to do at start:

1. **DON'T buy AWS/GCP** — burn rate killer
2. **DON'T pay for full audit** until TVL > $100k
3. **DON'T register Foundation** before product-market fit
4. **DON'T hire full-time devops** — automate
5. **DON'T use Alchemy/Infura** — self-host RPC

### ✅ What to do instead:

1. **Bare metal** (Hetzner/OVH) instead of cloud
2. **Community security** + automated scanners
3. **Disclaimer: "Use at your own risk"**
4. **Systemd + Grafana** for monitoring
5. **Self-host everything** possible

---

## 📞 Useful Links

### Infrastructure
- **Hetzner Server Auction**: https://www.hetzner.com/sb
- **Vast.ai GPU Marketplace**: https://vast.ai/
- **Vercel Free Tier**: https://vercel.com/

### Grants
- **Arbitrum Grants**: https://arbitrumfoundation.io/grants
- **Gitcoin**: https://gitcoin.co/
- **Ethereum ESP**: https://esp.ethereum.foundation/

### Documentation
- **Arbitrum Orbit**: https://docs.arbitrum.io/launch-orbit-chain/orbit-quickstart
- **Stylus Docs**: https://docs.arbitrum.io/stylus/stylus-quickstart
- **RISC Zero**: https://dev.risczero.com/

### Community
- **Arbitrum Discord**: https://discord.gg/arbitrum
- **RISC Zero Discord**: https://discord.gg/risczero
- **Telegram**: https://t.me/arbitrum_orbit

---

## 🎓 FAQ

**Q: What if Hetzner blocks crypto node?**  
A: Use OVH (France) or Contabo (Germany) — they're crypto-friendly. Hetzner doesn't block Layer 2/3 nodes yet.

**Q: What if 64GB RAM isn't enough?**  
A: Add 32GB swap. Swap is critical for ZK proofs.

**Q: Can I run without GPU?**  
A: Yes, in Dev Mode (`RISC0_DEV_MODE=1`) proofs aren't really generated. For production need GPU or Boundless/Vast.ai.

**Q: How much ETH needed for mainnet?**  
A: ~1.2 ETH ($3,000) to deploy Orbit chain. But this is covered by grant.

**Q: How long do grants take?**  
A: Arbitrum Foundation: 4-8 weeks. Gitcoin: right after round. Google Cloud: 2-4 weeks.

**Q: What if no grant?**  
A: Can launch testnet for $0 and attract community funding via Gitcoin/Juicebox.

---

## 🏁 Summary: 3 Launch Paths

| Scenario | Timeline | Cost | Risk |
|----------|----------|------|------|
| **Hobby** (testnet only) | 1-2 months | $0 | Zero |
| **Bootstrap** (lean mainnet) | 3-6 months | $1,284/year (or $0 with grants) | Low |
| **VC-backed** (enterprise) | 6-12 months | $283k/year | Medium |

### Recommended Strategy:

1. **Month 1-2**: Local devnet ($0)
2. **Month 3-4**: Testnet on Hetzner ($100)
3. **Month 5-6**: Grant applications ($0, potential $10k+)
4. **Month 7+**: Mainnet from grants ($0 out of pocket)

**Total out-of-pocket cost**: $100-300 until grants arrive.

**Expected grants**: $8,000-10,000 in first 6 months.

**Runway**: 5+ years without fundraising.

---

**Philosophy**: *"Build in public. Ship fast. Optimize for longevity, not vanity metrics."*

---

*Document current as of: January 21, 2026*  
*Strategy: Indie Hacker / Bootstrap*  
*Burn rate: -99.3% vs Enterprise*

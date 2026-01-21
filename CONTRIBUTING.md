# 🤝 Contributing to PROVION

Thank you for your interest in contributing to PROVION! This document provides guidelines and instructions for contributing to the project.

---

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [How to Contribute](#how-to-contribute)
5. [Coding Standards](#coding-standards)
6. [Testing Guidelines](#testing-guidelines)
7. [Pull Request Process](#pull-request-process)
8. [Community](#community)

---

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. We pledge to make participation in our project a harassment-free experience for everyone, regardless of:

- Age, body size, disability, ethnicity, gender identity and expression
- Level of experience, education, socio-economic status
- Nationality, personal appearance, race, religion
- Sexual identity and orientation

### Our Standards

**Positive behavior includes:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable behavior includes:**
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the project team at [security@provion.xyz](mailto:security@provion.xyz). All complaints will be reviewed and investigated promptly and fairly.

---

## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust** (stable toolchain): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js** 18+: `nvm install 18`
- **Docker**: For running local test nodes
- **Git**: For version control

### Repository Structure

```
provion/
├── contracts/          # Stylus smart contracts (Rust)
├── prover/            # ZK proof generation (RISC Zero)
├── ui/                # Frontend (Next.js + React)
├── integration-tests/ # End-to-end tests
├── docs/              # Documentation
└── scripts/           # Deployment and utility scripts
```

---

## 💻 Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/provion.git
cd provion

# Add upstream remote
git remote add upstream https://github.com/AlexSheff/provion.git
```

### 2. Install Dependencies

#### Contracts (Stylus)

```bash
cd contracts

# Install Cargo Stylus
cargo install --force cargo-stylus

# Add WASM target
rustup target add wasm32-unknown-unknown

# Verify setup
cargo stylus check
```

#### Prover (RISC Zero)

```bash
cd prover

# Install RISC Zero toolchain
curl -L https://risczero.com/install | bash
rzup install

# Verify installation
cargo risczero --version
```

#### Frontend (UI)

```bash
cd ui

# Install dependencies
yarn install

# Start dev server
yarn dev
```

### 3. Run Local Development Environment

#### Start Local Arbitrum Nitro Node

```bash
# Clone testnode
git clone https://github.com/OffchainLabs/nitro-testnode
cd nitro-testnode

# Initialize and start
./test-node.bash --init

# Node will be available at:
# - L1: http://localhost:8545
# - L2: http://localhost:8547
```

#### Deploy Contracts Locally

```bash
cd provion/contracts

# Deploy to local node
cargo stylus deploy \
  --endpoint=http://localhost:8547 \
  --private-key=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

# Save the contract address for testing
```

#### Run Tests

```bash
# Contract tests
cd contracts
cargo test

# Integration tests
cd ../integration-tests
yarn test

# Frontend tests
cd ../ui
yarn test
```

---

## 🔧 How to Contribute

### Types of Contributions

We welcome the following types of contributions:

1. **Bug Reports**: Found a bug? Open an issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots (if applicable)
   - Environment details (OS, browser, etc.)

2. **Feature Requests**: Have an idea? Open a discussion with:
   - Use case and motivation
   - Proposed implementation (if any)
   - Alternatives considered

3. **Code Contributions**:
   - Bug fixes
   - New features
   - Performance improvements
   - Documentation updates

4. **Documentation**:
   - Fix typos or clarify existing docs
   - Add examples or tutorials
   - Translate documentation

5. **Testing**:
   - Add test coverage
   - Improve test infrastructure
   - Report edge cases

### Finding Work

**Good first issues**: Look for issues tagged with `good first issue` or `help wanted`

**Priority areas**:
- [ ] Increase test coverage (currently ~60%, target 80%)
- [ ] Improve ZK proof generation performance
- [ ] Add support for GitHub App integration
- [ ] Implement multi-chain deployment scripts
- [ ] Write more comprehensive documentation

---

## 📝 Coding Standards

### Rust (Contracts & Prover)

**Style Guide**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

**Formatting**:
```bash
# Format code before committing
cargo fmt

# Check linting
cargo clippy -- -D warnings
```

**Best Practices**:
```rust
// ✅ Good: Clear naming, documented
/// Deposits USDC into yield-bearing vault
pub fn deposit_to_yield(&mut self, amount: U256) -> Result<(), Error> {
    require!(amount > 0, Error::InvalidAmount);
    // Implementation...
    Ok(())
}

// ❌ Bad: Unclear naming, no docs
pub fn dep(&mut self, amt: U256) {
    // Implementation...
}
```

**Error Handling**:
```rust
// ✅ Use custom error types
#[derive(Debug)]
pub enum BountyError {
    InvalidAmount,
    BountyNotFound,
    UnauthorizedAccess,
    InsufficientBalance,
}

// ❌ Don't use string errors
Err("something went wrong")
```

### TypeScript/JavaScript (Frontend)

**Style Guide**: Follow [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)

**Formatting**:
```bash
# Format with Prettier
yarn format

# Lint with ESLint
yarn lint
```

**Best Practices**:
```typescript
// ✅ Good: Type-safe, clear
interface BountyData {
  id: number;
  amount: bigint;
  status: BountyStatus;
}

async function fetchBounty(id: number): Promise<BountyData> {
  const response = await fetch(`/api/bounties/${id}`);
  if (!response.ok) throw new Error('Bounty not found');
  return response.json();
}

// ❌ Bad: No types, unclear
async function get(x) {
  const r = await fetch('/api/' + x);
  return r.json();
}
```

### Git Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format:
<type>(<scope>): <subject>

# Types:
feat:     New feature
fix:      Bug fix
docs:     Documentation changes
style:    Code style changes (formatting, etc.)
refactor: Code refactoring
perf:     Performance improvements
test:     Adding or updating tests
chore:    Maintenance tasks

# Examples:
feat(contracts): add multi-sig support for high-value bounties
fix(prover): resolve memory leak in proof generation
docs(readme): update deployment instructions
test(contracts): add unit tests for escrow vault
```

**Good commit messages**:
```
✅ feat(contracts): implement yield-bearing escrow vault

Integrates with Aave V3 to generate yield on deposited USDC.
Yield is used to cover gas costs and reviewer incentives.

Closes #42
```

**Bad commit messages**:
```
❌ fixed stuff
❌ update
❌ wip
```

---

## 🧪 Testing Guidelines

### Test Coverage Requirements

All PRs must maintain or improve test coverage:

- **Contracts**: Minimum 80% coverage
- **Prover**: Minimum 70% coverage
- **Frontend**: Minimum 60% coverage

### Writing Tests

#### Contract Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bounty_success() {
        let mut contract = BountyManager::new();
        let result = contract.create_bounty(
            "rust-lang/cargo".to_string(),
            42,
            U256::from(5000),
            1234567890,
        );
        
        assert!(result.is_ok());
        let bounty_id = result.unwrap();
        assert_eq!(contract.get_bounty(bounty_id).amount, U256::from(5000));
    }

    #[test]
    #[should_panic(expected = "InvalidAmount")]
    fn test_create_bounty_zero_amount() {
        let mut contract = BountyManager::new();
        contract.create_bounty(
            "test/repo".to_string(),
            1,
            U256::from(0),  // Invalid: zero amount
            1234567890,
        ).unwrap();
    }
}
```

#### Integration Tests (TypeScript)

```typescript
import { expect } from 'chai';
import { deployContract, createBounty } from './helpers';

describe('Bounty Lifecycle', () => {
  it('should create, submit, and payout bounty', async () => {
    // Setup
    const contract = await deployContract();
    const creator = await getSigners()[0];
    
    // Create bounty
    const tx = await createBounty(contract, {
      repo: 'test/repo',
      amount: ethers.parseUnits('5000', 6), // USDC has 6 decimals
    });
    await tx.wait();
    
    // Verify
    const bounty = await contract.getBounty(0);
    expect(bounty.amount).to.equal(ethers.parseUnits('5000', 6));
    expect(bounty.status).to.equal(BountyStatus.FUNDED);
  });
});
```

#### Frontend Tests (React Testing Library)

```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { BountyCard } from './BountyCard';

describe('BountyCard', () => {
  it('displays bounty details correctly', () => {
    const bounty = {
      id: 1,
      repo: 'rust-lang/cargo',
      amount: 5000,
      status: 'open',
    };
    
    render(<BountyCard bounty={bounty} />);
    
    expect(screen.getByText('rust-lang/cargo')).toBeInTheDocument();
    expect(screen.getByText('$5,000')).toBeInTheDocument();
    expect(screen.getByText('Open')).toBeInTheDocument();
  });
  
  it('calls onClaim when claim button clicked', () => {
    const onClaim = jest.fn();
    render(<BountyCard bounty={mockBounty} onClaim={onClaim} />);
    
    fireEvent.click(screen.getByText('Claim Bounty'));
    
    expect(onClaim).toHaveBeenCalledWith(mockBounty.id);
  });
});
```

### Running Tests

```bash
# Run all tests
./scripts/test-all.sh

# Run specific test suite
cd contracts && cargo test
cd prover && cargo test
cd ui && yarn test

# Run with coverage
cd contracts && cargo tarpaulin --out Html
cd ui && yarn test --coverage
```

---

## 🔀 Pull Request Process

### Before Submitting

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run tests locally**:
   ```bash
   cargo test
   yarn test
   ```

3. **Run linters**:
   ```bash
   cargo fmt
   cargo clippy
   yarn lint
   ```

4. **Update documentation** if needed

5. **Add tests** for new features

### Submitting a PR

1. **Create a feature branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Make your changes** following coding standards

3. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat(scope): description"
   ```

4. **Push to your fork**:
   ```bash
   git push origin feat/your-feature-name
   ```

5. **Open a Pull Request** on GitHub with:
   - Clear title following conventional commits
   - Description of what changed and why
   - Link to related issues (`Closes #123`)
   - Screenshots (if UI changes)
   - Checklist completed (see below)

### PR Template

```markdown
## Description
Brief description of changes

## Motivation and Context
Why is this change needed? What problem does it solve?

## How Has This Been Tested?
Describe the tests you ran

## Types of changes
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update

## Checklist
- [ ] My code follows the code style of this project
- [ ] I have updated the documentation accordingly
- [ ] I have added tests to cover my changes
- [ ] All new and existing tests passed
- [ ] My changes generate no new warnings
- [ ] I have checked my code and corrected any misspellings
```

### Review Process

1. **Automated checks** must pass:
   - CI/CD pipeline (GitHub Actions)
   - Test coverage requirements
   - Linting and formatting

2. **Code review** by maintainers:
   - At least 1 approval required
   - Address all comments before merging

3. **Merge**:
   - Squash and merge for feature branches
   - Merge commit for release branches

---

## 🐛 Reporting Bugs

### Security Vulnerabilities

**DO NOT** open public issues for security vulnerabilities.

Instead, email [security@provion.xyz](mailto:security@provion.xyz) with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

**Bug Bounty**: Critical vulnerabilities may be eligible for rewards (see [SECURITY.md](SECURITY.md)).

### Regular Bugs

Open an issue on GitHub with the **Bug Report** template:

```markdown
## Bug Description
Clear and concise description of the bug

## To Reproduce
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. Scroll down to '...'
4. See error

## Expected Behavior
What you expected to happen

## Actual Behavior
What actually happened

## Screenshots
If applicable, add screenshots

## Environment
- OS: [e.g. Ubuntu 22.04]
- Browser: [e.g. Chrome 120]
- Contract version: [e.g. v0.1.0]
- Node version: [e.g. 18.17.0]

## Additional Context
Any other context about the problem
```

---

## 💡 Feature Requests

Open a **GitHub Discussion** (not an issue) with:

1. **Problem Statement**: What problem are you trying to solve?
2. **Proposed Solution**: How would you solve it?
3. **Alternatives**: What other solutions did you consider?
4. **Additional Context**: Mockups, examples, etc.

**Example**:
```markdown
## Problem
Currently, reviewers must manually check GitHub for new PRs to review.

## Proposed Solution
Add email notifications when a new PR is submitted for a bounty the reviewer is eligible for.

## Alternatives
- Telegram bot notifications
- Discord webhook integration
- RSS feed

## Mockups
[Attach screenshot of email template]
```

---

## 🌍 Community

### Communication Channels

- **GitHub Discussions**: Design discussions, feature requests
- **GitHub Issues**: Bug reports, specific tasks
- **Discord**: [discord.gg/provion](https://discord.gg/fnZcbKpTWj) (real-time chat)
- **Twitter**: [none](none) (announcements)
- **Telegram**: [t.me/provion](https://t.me/provions) (community chat)

### Weekly Development Calls

**When**: Every Monday at 4 PM UTC  
**Where**: Discord voice channel  
**Agenda**: Posted in #dev-calls 24h before

Everyone is welcome to join and listen. If you want to present, DM a maintainer.

### Maintainers

Current maintainers:
- [@AlexSheff](https://github.com/AlexSheff) - Project Lead
- (More maintainers to be added as project grows)

### Recognition

Contributors are recognized in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md) - Hall of fame
- Monthly highlights in Discord
- Special role in Discord server
- Early access to token airdrops (if applicable)

---

## 🎓 Learning Resources

### For New Contributors

**Arbitrum Orbit**:
- [Orbit Documentation](https://docs.arbitrum.io/launch-orbit-chain/orbit-quickstart)
- [Stylus Quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart)

**RISC Zero**:
- [zkVM Developer Guide](https://dev.risczero.com/)
- [Example Projects](https://github.com/risc0/risc0/tree/main/examples)

**Web3 Development**:
- [wagmi Documentation](https://wagmi.sh/)
- [viem Documentation](https://viem.sh/)

**Rust**:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Recommended Tools

- **VS Code Extensions**:
  - rust-analyzer
  - Error Lens
  - GitLens
  - Prettier
  - ESLint

- **CLI Tools**:
  - `cargo-watch`: Auto-recompile on changes
  - `cargo-tarpaulin`: Code coverage
  - `foundry`: Ethereum development toolkit

---

## 📄 License

By contributing to PROVION, you agree that your contributions will be licensed under the [Apache 2.0 License](LICENSE).

---

## 🙏 Thank You

Your contributions, no matter how small, are valuable to PROVION. Whether it's fixing a typo, adding a test, or implementing a major feature - thank you for helping build the future of decentralized work!

**Questions?** Don't hesitate to ask in Discord or open a discussion on GitHub.

---

*This document is a living guide. If you have suggestions for improvements, please submit a PR!*

*Last updated: January 21, 2026*

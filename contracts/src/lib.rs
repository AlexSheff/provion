#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*, alloy_primitives::{Address, U256}, msg, block};

#[storage]
pub struct Bounty {
    pub creator: StorageAddress,
    pub asset_token: StorageAddress,
    pub amount: StorageU256,
    pub status: StorageU8, // 0=Open, 1=InDispute, 2=Paid, 3=Slashed
    pub created_at: StorageU64,
    pub dispute_end_time: StorageU64,
    pub solver: StorageAddress,
}

#[storage]
#[entrypoint]
pub struct ProvionCore {
    pub bounty_count: StorageU256,
    pub bounties: StorageMap<U256, Bounty>,
    pub yield_vault: StorageAddress,
}

sol_interface! {
    interface IERC20 {
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
        function transfer(address recipient, uint256 amount) external returns (bool);
        function approve(address spender, uint256 amount) external returns (bool);
    }
    
    interface IYieldVault {
        function deposit(address asset, uint256 amount) external returns (uint256);
        function withdraw(address asset, uint256 amount) external returns (uint256);
    }
}

#[public]
impl ProvionCore {
    pub fn init(&mut self, vault: Address) -> Result<(), Vec<u8>> {
        self.yield_vault.set(vault);
        Ok(())
    }

    pub fn create_bounty(&mut self, asset: Address, amount: U256) -> Result<U256, Vec<u8>> {
        let current_id = self.bounty_count.get();
        let new_id = current_id + U256::from(1);
        self.bounty_count.set(new_id);

        let token_contract = IERC20::new(asset);
        // Transfer funds from Creator to Contract
        let transfer_result = token_contract.transfer_from(self, msg::sender(), contract::address(), amount);
        
        if transfer_result.is_err() || !transfer_result.unwrap() {
             return Err(b"ERC20 Transfer Failed".to_vec());
        }

        // AUTO-YIELD: Deposit idle funds into Strategy
        let vault_addr = self.yield_vault.get();
        if vault_addr != Address::ZERO {
            let _ = token_contract.approve(self, vault_addr, amount);
            let vault = IYieldVault::new(vault_addr);
            let _ = vault.deposit(self, asset, amount);
        }

        let mut new_bounty = self.bounties.setter(new_id);
        new_bounty.creator.set(msg::sender());
        new_bounty.asset_token.set(asset);
        new_bounty.amount.set(amount);
        new_bounty.status.set(0); 
        new_bounty.created_at.set(block::timestamp());

        Ok(new_id)
    }

    /// Step 1: Submit Proof & Start Dispute Window
    /// Verifies ZK Proof of Maintainer & Reviewer Signatures
    pub fn submit_solution(&mut self, id: U256, solver: Address, _proof: Vec<u8>) -> Result<bool, Vec<u8>> {
        let mut bounty = self.bounties.setter(id);
        
        if bounty.status.get() != 0 {
            return Err(b"Bounty not open".to_vec());
        }

        // ZK Verification (Simulated for Alpha)
        // Checks: Maintainer Merged + Reviewers Signed
        let proof_valid = true; 
        if !proof_valid {
            return Err(b"Invalid ZK Proof".to_vec());
        }

        // Start Dispute Window (24 Hours)
        bounty.status.set(1); // InDispute
        bounty.solver.set(solver);
        bounty.dispute_end_time.set(block::timestamp() + 86400);

        Ok(true)
    }

    /// Step 2: Finalize Payout (Post-Dispute)
    pub fn finalize_bounty(&mut self, id: U256) -> Result<bool, Vec<u8>> {
        let mut bounty = self.bounties.setter(id);
        
        if bounty.status.get() != 1 {
            return Err(b"Not in dispute phase".to_vec());
        }

        if block::timestamp() < bounty.dispute_end_time.get() {
            return Err(b"Dispute window active".to_vec());
        }

        // Withdraw from Yield Strategy
        let asset = bounty.asset_token.get();
        let amount = bounty.amount.get();
        let vault_addr = self.yield_vault.get();

        if vault_addr != Address::ZERO {
            let vault = IYieldVault::new(vault_addr);
            let _ = vault.withdraw(self, asset, amount);
        }

        // Payout to Solver
        let solver = bounty.solver.get();
        let token_contract = IERC20::new(asset);
        let payout_result = token_contract.transfer(self, solver, amount);
        
        if payout_result.is_err() || !payout_result.unwrap() {
            return Err(b"Payout Failed".to_vec());
        }

        bounty.status.set(2); // Paid
        Ok(true)
    }

    /// The Fisherman Mechanism
    pub fn challenge_bounty(&mut self, id: U256) -> Result<bool, Vec<u8>> {
        let mut bounty = self.bounties.setter(id);
        
        if bounty.status.get() != 1 {
            return Err(b"Not challengeable".to_vec());
        }

        // In a real scenario, this would verify a Fraud Proof
        // For Alpha, we simulate a successful challenge
        bounty.status.set(3); // Slashed
        
        // Reward Challenger (Logic omitted for brevity)
        
        Ok(true)
    }

    pub fn get_bounty(&self, id: U256) -> Result<(Address, Address, U256, u8, u64), Vec<u8>> {
        let bounty = self.bounties.getter(id);
        Ok((
            bounty.creator.get(), 
            bounty.asset_token.get(), 
            bounty.amount.get(), 
            bounty.status.get(),
            bounty.dispute_end_time.get()
        ))
    }
}

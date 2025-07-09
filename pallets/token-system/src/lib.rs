//! # Token System Pallet (Standalone Version)
//!
//! A comprehensive token system for energy trading ecosystems featuring:
//! - GRID token: Utility/governance token with staking capabilities  
//! - WATT token: Fiat-pegged stablecoin for energy trading
//! - Governance system with proposal creation and voting
//! - Staking rewards and price stability mechanisms

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Token balance type
pub type Balance = u64;

/// Token ID type
pub type TokenId = u32;

/// Proposal ID type
pub type ProposalId = u32;

/// Account ID type
pub type AccountId = String;

/// Block number type
pub type BlockNumber = u64;

/// Token types in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// GRID token - Utility/governance token
    Grid,
    /// WATT token - Fiat-pegged stablecoin
    Watt,
}

/// Token information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Token type
    pub token_type: TokenType,
    /// Total supply
    pub total_supply: Balance,
    /// Current price (in basis points, 1 USD = 10000)
    pub price: u32,
    /// Whether the token is active
    pub is_active: bool,
}

/// Staking information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeInfo {
    /// Staked amount
    pub amount: Balance,
    /// Staking start block
    pub start_block: BlockNumber,
    /// Last reward claim block
    pub last_reward_block: BlockNumber,
    /// Unclaimed rewards
    pub unclaimed_rewards: Balance,
}

/// Governance proposal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID
    pub id: ProposalId,
    /// Proposer account
    pub proposer: AccountId,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Voting deadline
    pub voting_deadline: BlockNumber,
    /// Votes in favor
    pub votes_for: Balance,
    /// Votes against
    pub votes_against: Balance,
    /// Proposal status
    pub status: ProposalStatus,
}

/// Vote information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vote {
    /// Voter account
    pub voter: AccountId,
    /// Proposal ID
    pub proposal_id: ProposalId,
    /// Support for the proposal
    pub support: bool,
    /// Voting power used
    pub voting_power: Balance,
}

/// Proposal status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    /// Proposal is active and accepting votes
    Active,
    /// Proposal passed
    Passed,
    /// Proposal failed
    Failed,
    /// Proposal was executed
    Executed,
}

/// Events emitted by the token system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    /// GRID tokens were minted
    GridTokensMinted { account: AccountId, amount: Balance },
    /// WATT tokens were minted
    WattTokensMinted { account: AccountId, amount: Balance },
    /// GRID tokens were burned
    GridTokensBurned { account: AccountId, amount: Balance },
    /// WATT tokens were burned
    WattTokensBurned { account: AccountId, amount: Balance },
    /// Tokens were staked
    TokensStaked { account: AccountId, amount: Balance },
    /// Tokens were unstaked
    TokensUnstaked { account: AccountId, amount: Balance },
    /// Staking rewards were claimed
    RewardsClaimed { account: AccountId, amount: Balance },
    /// Governance proposal was created
    ProposalCreated { proposal_id: ProposalId, proposer: AccountId },
    /// Vote was cast on a proposal
    VoteCast { proposal_id: ProposalId, voter: AccountId, vote: bool },
    /// Proposal was finalized
    ProposalFinalized { proposal_id: ProposalId, passed: bool },
    /// WATT token price was updated
    WattPriceUpdated { new_price: u32 },
    /// Tokens were transferred
    TokensTransferred { token_type: TokenType, from: AccountId, to: AccountId, amount: Balance },
}

/// Errors that can occur in the token system
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TokenSystemError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Token type not found")]
    TokenNotFound,
    #[error("Minimum stake amount not met")]
    MinimumStakeNotMet,
    #[error("Account is not staking")]
    NotStaking,
    #[error("Proposal not found")]
    ProposalNotFound,
    #[error("Voting period has ended")]
    VotingPeriodEnded,
    #[error("Already voted on this proposal")]
    AlreadyVoted,
    #[error("Cannot vote on own proposal")]
    CannotVoteOnOwnProposal,
    #[error("Proposal is not active")]
    ProposalNotActive,
    #[error("Maximum number of proposals reached")]
    TooManyProposals,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("Price stability threshold exceeded")]
    PriceStabilityThresholdExceeded,
    #[error("Unauthorized operation")]
    Unauthorized,
}

/// Token system configuration
#[derive(Debug, Clone)]
pub struct TokenSystemConfig {
    /// Maximum number of governance proposals
    pub max_proposals: u32,
    /// Maximum number of stakers
    pub max_stakers: u32,
    /// Minimum stake amount for governance participation
    pub min_stake_amount: Balance,
    /// Annual staking reward rate (as percentage)
    pub staking_reward_rate: u8,
    /// WATT token price stability threshold (in basis points)
    pub price_stability_threshold: u32,
    /// Blocks per year (for reward calculation)
    pub blocks_per_year: u64,
}

impl Default for TokenSystemConfig {
    fn default() -> Self {
        Self {
            max_proposals: 100,
            max_stakers: 10000,
            min_stake_amount: 1000,
            staking_reward_rate: 8,
            price_stability_threshold: 500, // 5%
            blocks_per_year: 5_256_000, // Assuming 6-second block time
        }
    }
}

/// Main token system implementation
#[derive(Debug, Clone)]
pub struct TokenSystem {
    /// Configuration
    config: TokenSystemConfig,
    /// Current block number
    current_block: BlockNumber,
    /// GRID token balances
    grid_balances: HashMap<AccountId, Balance>,
    /// WATT token balances
    watt_balances: HashMap<AccountId, Balance>,
    /// Token information
    token_info: HashMap<TokenType, TokenInfo>,
    /// Staking information
    stakes: HashMap<AccountId, StakeInfo>,
    /// Governance proposals
    proposals: HashMap<ProposalId, Proposal>,
    /// Votes on proposals
    votes: HashMap<(ProposalId, AccountId), bool>,
    /// Next proposal ID
    next_proposal_id: ProposalId,
    /// Total staked tokens
    total_staked: Balance,
    /// Event log
    events: Vec<Event>,
}

impl TokenSystem {
    /// Create a new token system with default configuration
    pub fn new() -> Self {
        Self::with_config(TokenSystemConfig::default())
    }

    /// Create a new token system with custom configuration
    pub fn with_config(config: TokenSystemConfig) -> Self {
        Self {
            config,
            current_block: 0,
            grid_balances: HashMap::new(),
            watt_balances: HashMap::new(),
            token_info: HashMap::new(),
            stakes: HashMap::new(),
            proposals: HashMap::new(),
            votes: HashMap::new(),
            next_proposal_id: 0,
            total_staked: 0,
            events: Vec::new(),
        }
    }

    /// Initialize the token system
    pub fn initialize_tokens(&mut self, grid_supply: Balance, watt_supply: Balance) -> Result<(), TokenSystemError> {
        // Initialize GRID token
        let grid_info = TokenInfo {
            token_type: TokenType::Grid,
            total_supply: grid_supply,
            price: 10000, // 1 USD = 10000 basis points
            is_active: true,
        };
        self.token_info.insert(TokenType::Grid, grid_info);

        // Initialize WATT token
        let watt_info = TokenInfo {
            token_type: TokenType::Watt,
            total_supply: watt_supply,
            price: 10000, // 1 USD = 10000 basis points
            is_active: true,
        };
        self.token_info.insert(TokenType::Watt, watt_info);

        Ok(())
    }

    /// Mint GRID tokens to an account
    pub fn mint_grid_tokens(&mut self, to: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let current_balance = self.grid_balances.get(to).unwrap_or(&0);
        self.grid_balances.insert(to.clone(), current_balance + amount);

        // Update total supply
        if let Some(token_info) = self.token_info.get_mut(&TokenType::Grid) {
            token_info.total_supply = token_info.total_supply.saturating_add(amount);
        }

        self.emit_event(Event::GridTokensMinted { account: to.clone(), amount });
        Ok(())
    }

    /// Mint WATT tokens to an account
    pub fn mint_watt_tokens(&mut self, to: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let current_balance = self.watt_balances.get(to).unwrap_or(&0);
        self.watt_balances.insert(to.clone(), current_balance + amount);

        // Update total supply
        if let Some(token_info) = self.token_info.get_mut(&TokenType::Watt) {
            token_info.total_supply = token_info.total_supply.saturating_add(amount);
        }

        self.emit_event(Event::WattTokensMinted { account: to.clone(), amount });
        Ok(())
    }

    /// Burn GRID tokens from an account
    pub fn burn_grid_tokens(&mut self, from: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let current_balance = self.grid_balances.get(from).unwrap_or(&0);
        if *current_balance < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        self.grid_balances.insert(from.clone(), current_balance - amount);

        // Update total supply
        if let Some(token_info) = self.token_info.get_mut(&TokenType::Grid) {
            token_info.total_supply = token_info.total_supply.saturating_sub(amount);
        }

        self.emit_event(Event::GridTokensBurned { account: from.clone(), amount });
        Ok(())
    }

    /// Burn WATT tokens from an account
    pub fn burn_watt_tokens(&mut self, from: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let current_balance = self.watt_balances.get(from).unwrap_or(&0);
        if *current_balance < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        self.watt_balances.insert(from.clone(), current_balance - amount);

        // Update total supply
        if let Some(token_info) = self.token_info.get_mut(&TokenType::Watt) {
            token_info.total_supply = token_info.total_supply.saturating_sub(amount);
        }

        self.emit_event(Event::WattTokensBurned { account: from.clone(), amount });
        Ok(())
    }

    /// Transfer GRID tokens between accounts
    pub fn transfer_grid_tokens(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let from_balance = self.grid_balances.get(from).unwrap_or(&0);
        if *from_balance < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        let to_balance = self.grid_balances.get(to).unwrap_or(&0);
        self.grid_balances.insert(from.clone(), from_balance - amount);
        self.grid_balances.insert(to.clone(), to_balance + amount);

        self.emit_event(Event::TokensTransferred { 
            token_type: TokenType::Grid, 
            from: from.clone(), 
            to: to.clone(), 
            amount 
        });
        Ok(())
    }

    /// Transfer WATT tokens between accounts
    pub fn transfer_watt_tokens(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let from_balance = self.watt_balances.get(from).unwrap_or(&0);
        if *from_balance < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        let to_balance = self.watt_balances.get(to).unwrap_or(&0);
        self.watt_balances.insert(from.clone(), from_balance - amount);
        self.watt_balances.insert(to.clone(), to_balance + amount);

        self.emit_event(Event::TokensTransferred { 
            token_type: TokenType::Watt, 
            from: from.clone(), 
            to: to.clone(), 
            amount 
        });
        Ok(())
    }

    /// Stake GRID tokens for governance participation
    pub fn stake_tokens(&mut self, who: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        if amount < self.config.min_stake_amount {
            return Err(TokenSystemError::MinimumStakeNotMet);
        }

        let current_balance = self.grid_balances.get(who).unwrap_or(&0);
        if *current_balance < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        // Update or create stake info
        let stake_info = self.stakes.entry(who.clone()).or_insert(StakeInfo {
            amount: 0,
            start_block: self.current_block,
            last_reward_block: self.current_block,
            unclaimed_rewards: 0,
        });

        // Claim pending rewards before increasing stake
        let pending_rewards = self.calculate_pending_rewards(who, stake_info);
        stake_info.unclaimed_rewards = stake_info.unclaimed_rewards.saturating_add(pending_rewards);
        stake_info.amount = stake_info.amount.saturating_add(amount);
        stake_info.last_reward_block = self.current_block;

        // Transfer tokens from balance to staking
        self.grid_balances.insert(who.clone(), current_balance - amount);
        self.total_staked = self.total_staked.saturating_add(amount);

        self.emit_event(Event::TokensStaked { account: who.clone(), amount });
        Ok(())
    }

    /// Unstake GRID tokens
    pub fn unstake_tokens(&mut self, who: &AccountId, amount: Balance) -> Result<(), TokenSystemError> {
        let stake_info = self.stakes.get_mut(who).ok_or(TokenSystemError::NotStaking)?;
        if stake_info.amount < amount {
            return Err(TokenSystemError::InsufficientBalance);
        }

        // Calculate and add pending rewards
        let pending_rewards = self.calculate_pending_rewards(who, stake_info);
        stake_info.unclaimed_rewards = stake_info.unclaimed_rewards.saturating_add(pending_rewards);

        // Update stake
        stake_info.amount = stake_info.amount.saturating_sub(amount);
        stake_info.last_reward_block = self.current_block;

        if stake_info.amount == 0 {
            self.stakes.remove(who);
        }

        // Return tokens to balance
        let current_balance = self.grid_balances.get(who).unwrap_or(&0);
        self.grid_balances.insert(who.clone(), current_balance + amount);
        self.total_staked = self.total_staked.saturating_sub(amount);

        self.emit_event(Event::TokensUnstaked { account: who.clone(), amount });
        Ok(())
    }

    /// Claim staking rewards
    pub fn claim_rewards(&mut self, who: &AccountId) -> Result<(), TokenSystemError> {
        let stake_info = self.stakes.get_mut(who).ok_or(TokenSystemError::NotStaking)?;
        
        // Calculate total rewards (pending + unclaimed)
        let pending_rewards = self.calculate_pending_rewards(who, stake_info);
        let total_rewards = stake_info.unclaimed_rewards.saturating_add(pending_rewards);

        if total_rewards == 0 {
            return Err(TokenSystemError::InsufficientBalance);
        }

        // Update stake info
        stake_info.unclaimed_rewards = 0;
        stake_info.last_reward_block = self.current_block;

        // Mint reward tokens
        let current_balance = self.grid_balances.get(who).unwrap_or(&0);
        self.grid_balances.insert(who.clone(), current_balance + total_rewards);

        self.emit_event(Event::RewardsClaimed { account: who.clone(), amount: total_rewards });
        Ok(())
    }

    /// Create a governance proposal
    pub fn create_proposal(&mut self, who: &AccountId, title: String, description: String, voting_period: BlockNumber) -> Result<(), TokenSystemError> {
        // Ensure the proposer has staked tokens
        if !self.stakes.contains_key(who) {
            return Err(TokenSystemError::NotStaking);
        }

        let proposal_id = self.next_proposal_id;
        let voting_deadline = self.current_block + voting_period;

        let proposal = Proposal {
            id: proposal_id,
            proposer: who.clone(),
            title,
            description,
            voting_deadline,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
        };

        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;

        self.emit_event(Event::ProposalCreated { proposal_id, proposer: who.clone() });
        Ok(())
    }

    /// Vote on a governance proposal
    pub fn vote_on_proposal(&mut self, who: &AccountId, proposal_id: ProposalId, vote: bool) -> Result<(), TokenSystemError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(TokenSystemError::ProposalNotFound)?;
        
        // Check if proposal is active
        if proposal.status != ProposalStatus::Active {
            return Err(TokenSystemError::ProposalNotActive);
        }
        
        // Check if voting period has ended
        if self.current_block > proposal.voting_deadline {
            return Err(TokenSystemError::VotingPeriodEnded);
        }

        // Check if already voted
        if self.votes.contains_key(&(proposal_id, who.clone())) {
            return Err(TokenSystemError::AlreadyVoted);
        }

        // Cannot vote on own proposal
        if proposal.proposer == *who {
            return Err(TokenSystemError::CannotVoteOnOwnProposal);
        }

        // Get voting weight (staked amount)
        let stake_info = self.stakes.get(who).ok_or(TokenSystemError::NotStaking)?;
        let voting_weight = stake_info.amount;

        // Record vote
        self.votes.insert((proposal_id, who.clone()), vote);

        // Update proposal vote counts
        if vote {
            proposal.votes_for = proposal.votes_for.saturating_add(voting_weight);
        } else {
            proposal.votes_against = proposal.votes_against.saturating_add(voting_weight);
        }

        self.emit_event(Event::VoteCast { proposal_id, voter: who.clone(), vote });
        Ok(())
    }

    /// Finalize a governance proposal
    pub fn finalize_proposal(&mut self, proposal_id: ProposalId) -> Result<(), TokenSystemError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(TokenSystemError::ProposalNotFound)?;
        
        // Check if proposal is active
        if proposal.status != ProposalStatus::Active {
            return Err(TokenSystemError::ProposalNotActive);
        }
        
        // Check if voting period has ended
        if self.current_block <= proposal.voting_deadline {
            return Err(TokenSystemError::VotingPeriodEnded);
        }

        // Determine if proposal passed
        let passed = proposal.votes_for > proposal.votes_against;
        proposal.status = if passed { ProposalStatus::Passed } else { ProposalStatus::Failed };

        self.emit_event(Event::ProposalFinalized { proposal_id, passed });
        Ok(())
    }

    /// Update WATT token price
    pub fn update_watt_price(&mut self, new_price: u32) -> Result<(), TokenSystemError> {
        if let Some(token_info) = self.token_info.get_mut(&TokenType::Watt) {
            let old_price = token_info.price;
            token_info.price = new_price;
            
            // Check price stability
            let price_change = if new_price > old_price {
                new_price - old_price
            } else {
                old_price - new_price
            };
            
            let price_change_percentage = (price_change as f64 / old_price as f64) * 10000.0;
            
            // If price change exceeds threshold, trigger stability mechanism
            if price_change_percentage > self.config.price_stability_threshold as f64 {
                // In a real implementation, this would trigger mint/burn operations
                // to maintain price stability
            }
        }

        self.emit_event(Event::WattPriceUpdated { new_price });
        Ok(())
    }

    /// Calculate pending rewards for a staker
    fn calculate_pending_rewards(&self, _who: &AccountId, stake_info: &StakeInfo) -> Balance {
        let blocks_since_last_reward = self.current_block.saturating_sub(stake_info.last_reward_block);
        
        // Calculate rewards based on annual rate
        let annual_rate = self.config.staking_reward_rate as u64;
        
        let rewards = stake_info.amount
            .saturating_mul(annual_rate)
            .saturating_mul(blocks_since_last_reward)
            .saturating_div(self.config.blocks_per_year)
            .saturating_div(100); // Convert percentage to decimal
        
        rewards
    }

    /// Get account's GRID token balance
    pub fn get_grid_balance(&self, who: &AccountId) -> Balance {
        *self.grid_balances.get(who).unwrap_or(&0)
    }

    /// Get account's WATT token balance
    pub fn get_watt_balance(&self, who: &AccountId) -> Balance {
        *self.watt_balances.get(who).unwrap_or(&0)
    }

    /// Get account's staking information
    pub fn get_stake_info(&self, who: &AccountId) -> Option<StakeInfo> {
        self.stakes.get(who).cloned()
    }

    /// Get token information
    pub fn get_token_info(&self, token_type: &TokenType) -> Option<TokenInfo> {
        self.token_info.get(token_type).cloned()
    }

    /// Get proposal information
    pub fn get_proposal(&self, proposal_id: ProposalId) -> Option<Proposal> {
        self.proposals.get(&proposal_id).cloned()
    }

    /// Check if account has voted on a proposal
    pub fn has_voted(&self, proposal_id: ProposalId, who: &AccountId) -> bool {
        self.votes.contains_key(&(proposal_id, who.clone()))
    }

    /// Get total staked amount
    pub fn total_staked(&self) -> Balance {
        self.total_staked
    }

    /// Advance to the next block
    pub fn advance_block(&mut self) {
        self.current_block += 1;
    }

    /// Get current block number
    pub fn current_block(&self) -> BlockNumber {
        self.current_block
    }

    /// Get all events
    pub fn get_events(&self) -> &[Event] {
        &self.events
    }

    /// Clear events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Emit an event
    fn emit_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl Default for TokenSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_token_system() -> TokenSystem {
        let mut token_system = TokenSystem::new();
        token_system.initialize_tokens(1_000_000, 1_000_000).unwrap();
        token_system
    }

    #[test]
    fn test_token_initialization() {
        let token_system = setup_token_system();
        
        let grid_info = token_system.get_token_info(&TokenType::Grid).unwrap();
        assert_eq!(grid_info.total_supply, 1_000_000);
        assert_eq!(grid_info.price, 10000);
        assert_eq!(grid_info.is_active, true);

        let watt_info = token_system.get_token_info(&TokenType::Watt).unwrap();
        assert_eq!(watt_info.total_supply, 1_000_000);
        assert_eq!(watt_info.price, 10000);
        assert_eq!(watt_info.is_active, true);
    }

    #[test]
    fn test_token_minting_and_burning() {
        let mut token_system = setup_token_system();
        
        // Mint GRID tokens
        token_system.mint_grid_tokens(&"alice".to_string(), 1000).unwrap();
        assert_eq!(token_system.get_grid_balance(&"alice".to_string()), 1000);

        // Burn GRID tokens
        token_system.burn_grid_tokens(&"alice".to_string(), 500).unwrap();
        assert_eq!(token_system.get_grid_balance(&"alice".to_string()), 500);

        // Try to burn more than available
        assert!(token_system.burn_grid_tokens(&"alice".to_string(), 1000).is_err());
    }

    #[test]
    fn test_token_transfers() {
        let mut token_system = setup_token_system();
        
        // Mint tokens to alice
        token_system.mint_grid_tokens(&"alice".to_string(), 1000).unwrap();
        
        // Transfer tokens to bob
        token_system.transfer_grid_tokens(&"alice".to_string(), &"bob".to_string(), 300).unwrap();
        
        assert_eq!(token_system.get_grid_balance(&"alice".to_string()), 700);
        assert_eq!(token_system.get_grid_balance(&"bob".to_string()), 300);
    }

    #[test]
    fn test_staking() {
        let mut token_system = setup_token_system();
        
        // Mint tokens and stake
        token_system.mint_grid_tokens(&"alice".to_string(), 5000).unwrap();
        token_system.stake_tokens(&"alice".to_string(), 2000).unwrap();
        
        let stake_info = token_system.get_stake_info(&"alice".to_string()).unwrap();
        assert_eq!(stake_info.amount, 2000);
        assert_eq!(token_system.get_grid_balance(&"alice".to_string()), 3000);
        assert_eq!(token_system.total_staked(), 2000);
    }

    #[test]
    fn test_staking_rewards() {
        let mut token_system = setup_token_system();
        
        // Mint tokens and stake
        token_system.mint_grid_tokens(&"alice".to_string(), 5000).unwrap();
        token_system.stake_tokens(&"alice".to_string(), 2000).unwrap();
        
        // Advance blocks to accumulate rewards
        for _ in 0..1000 {
            token_system.advance_block();
        }
        
        let initial_balance = token_system.get_grid_balance(&"alice".to_string());
        token_system.claim_rewards(&"alice".to_string()).unwrap();
        
        // Check that balance increased (rewards were added)
        assert!(token_system.get_grid_balance(&"alice".to_string()) > initial_balance);
    }

    #[test]
    fn test_governance_proposals() {
        let mut token_system = setup_token_system();
        
        // Setup staking for governance
        token_system.mint_grid_tokens(&"alice".to_string(), 5000).unwrap();
        token_system.stake_tokens(&"alice".to_string(), 2000).unwrap();
        
        // Create proposal
        token_system.create_proposal(
            &"alice".to_string(),
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            100
        ).unwrap();
        
        let proposal = token_system.get_proposal(0).unwrap();
        assert_eq!(proposal.proposer, "alice".to_string());
        assert_eq!(proposal.title, "Test Proposal".to_string());
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[test]
    fn test_governance_voting() {
        let mut token_system = setup_token_system();
        
        // Setup stakers
        token_system.mint_grid_tokens(&"alice".to_string(), 5000).unwrap();
        token_system.mint_grid_tokens(&"bob".to_string(), 5000).unwrap();
        token_system.stake_tokens(&"alice".to_string(), 2000).unwrap();
        token_system.stake_tokens(&"bob".to_string(), 3000).unwrap();
        
        // Create proposal
        token_system.create_proposal(
            &"alice".to_string(),
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            100
        ).unwrap();
        
        // Vote on proposal
        token_system.vote_on_proposal(&"bob".to_string(), 0, true).unwrap();
        
        let proposal = token_system.get_proposal(0).unwrap();
        assert_eq!(proposal.votes_for, 3000);
        assert_eq!(proposal.votes_against, 0);
    }

    #[test]
    fn test_comprehensive_workflow() {
        let mut token_system = setup_token_system();
        
        // Setup accounts
        token_system.mint_grid_tokens(&"alice".to_string(), 10000).unwrap();
        token_system.mint_grid_tokens(&"bob".to_string(), 10000).unwrap();
        token_system.mint_watt_tokens(&"alice".to_string(), 5000).unwrap();
        
        // Stake for governance
        token_system.stake_tokens(&"alice".to_string(), 5000).unwrap();
        token_system.stake_tokens(&"bob".to_string(), 7000).unwrap();
        
        // Create and vote on proposal
        token_system.create_proposal(
            &"alice".to_string(),
            "Increase staking rewards".to_string(),
            "Proposal to increase staking rewards".to_string(),
            50
        ).unwrap();
        
        token_system.vote_on_proposal(&"bob".to_string(), 0, true).unwrap();
        
        // Transfer tokens
        token_system.transfer_grid_tokens(&"alice".to_string(), &"bob".to_string(), 1000).unwrap();
        
        // Advance blocks and finalize proposal
        for _ in 0..52 {
            token_system.advance_block();
        }
        
        token_system.finalize_proposal(0).unwrap();
        
        let proposal = token_system.get_proposal(0).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchError},
        pallet_prelude::*,
        traits::{Get, ReservableCurrency, Currency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_arithmetic::{
        traits::{Saturating, Zero},
        Percent, Permill,
    };
    use sp_std::vec::Vec;
    use codec::{Encode, Decode};
    use scale_info::TypeInfo;

    /// Current pallet version
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Pallet configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency used for reserving tokens
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Maximum number of governance proposals
        #[pallet::constant]
        type MaxProposals: Get<u32>;

        /// Maximum number of stakers
        #[pallet::constant]
        type MaxStakers: Get<u32>;

        /// Minimum stake amount for governance participation
        #[pallet::constant]
        type MinStakeAmount: Get<u64>;

        /// Annual staking reward rate (as percentage)
        #[pallet::constant]
        type StakingRewardRate: Get<Percent>;

        /// WATT token price stability threshold
        #[pallet::constant]
        type PriceStabilityThreshold: Get<Permill>;

        /// Pallet ID for treasury
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    /// Token balance type
    pub type Balance = u64;

    /// Token ID type
    pub type TokenId = u32;

    /// Proposal ID type
    pub type ProposalId = u32;

    /// Token types in the system
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum TokenType {
        /// GRID token - Utility/governance token
        Grid,
        /// WATT token - Fiat-pegged stablecoin
        Watt,
    }

    /// Token information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct TokenInfo {
        /// Token type
        pub token_type: TokenType,
        /// Total supply
        pub total_supply: Balance,
        /// Current price (in basis points, 1 USD = 10000)
        pub price: u32,
        /// Whether the token is active
        pub is_active: bool,
    }

    /// Staking information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct StakeInfo {
        /// Staked amount
        pub amount: Balance,
        /// Staking start block
        pub start_block: T::BlockNumber,
        /// Last reward claim block
        pub last_reward_block: T::BlockNumber,
        /// Unclaimed rewards
        pub unclaimed_rewards: Balance,
    }

    /// Governance proposal
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Proposal {
        /// Proposal ID
        pub id: ProposalId,
        /// Proposer account
        pub proposer: AccountIdOf<T>,
        /// Proposal title
        pub title: Vec<u8>,
        /// Proposal description
        pub description: Vec<u8>,
        /// Voting deadline
        pub voting_deadline: T::BlockNumber,
        /// Votes in favor
        pub votes_for: Balance,
        /// Votes against
        pub votes_against: Balance,
        /// Proposal status
        pub status: ProposalStatus,
    }

    /// Proposal status
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProposalStatus {
        /// Proposal is active and accepting votes
        Active,
        /// Proposal passed
        Passed,
        /// Proposal failed
        Failed,
        /// Proposal was executed
        Executed,
    }

    /// Account ID type alias
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    /// GRID token balances
    #[pallet::storage]
    #[pallet::getter(fn grid_balances)]
    pub type GridBalances<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Balance,
        ValueQuery,
    >;

    /// WATT token balances
    #[pallet::storage]
    #[pallet::getter(fn watt_balances)]
    pub type WattBalances<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Balance,
        ValueQuery,
    >;

    /// Token information storage
    #[pallet::storage]
    #[pallet::getter(fn token_info)]
    pub type TokenInfoStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        TokenType,
        TokenInfo,
        OptionQuery,
    >;

    /// Staking information for accounts
    #[pallet::storage]
    #[pallet::getter(fn stakes)]
    pub type Stakes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        StakeInfo,
        OptionQuery,
    >;

    /// Current governance proposals
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalId,
        Proposal,
        OptionQuery,
    >;

    /// Next proposal ID
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, ProposalId, ValueQuery>;

    /// Votes on proposals
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ProposalId,
        Blake2_128Concat,
        T::AccountId,
        bool, // true = for, false = against
        OptionQuery,
    >;

    /// Total staked GRID tokens
    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    pub type TotalStaked<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// Events emitted by the pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// GRID tokens were minted
        GridTokensMinted { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// WATT tokens were minted
        WattTokensMinted { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// GRID tokens were burned
        GridTokensBurned { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// WATT tokens were burned
        WattTokensBurned { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// Tokens were staked
        TokensStaked { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// Tokens were unstaked
        TokensUnstaked { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// Staking rewards were claimed
        RewardsClaimed { 
            account: T::AccountId, 
            amount: Balance 
        },
        /// Governance proposal was created
        ProposalCreated { 
            proposal_id: ProposalId, 
            proposer: T::AccountId 
        },
        /// Vote was cast on a proposal
        VoteCast { 
            proposal_id: ProposalId, 
            voter: T::AccountId, 
            vote: bool 
        },
        /// Proposal was finalized
        ProposalFinalized { 
            proposal_id: ProposalId, 
            passed: bool 
        },
        /// WATT token price was updated
        WattPriceUpdated { 
            new_price: u32 
        },
        /// Tokens were transferred
        TokensTransferred { 
            token_type: TokenType, 
            from: T::AccountId, 
            to: T::AccountId, 
            amount: Balance 
        },
    }

    /// Errors that can occur in the pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient balance
        InsufficientBalance,
        /// Token type not found
        TokenNotFound,
        /// Minimum stake amount not met
        MinimumStakeNotMet,
        /// Account is not staking
        NotStaking,
        /// Proposal not found
        ProposalNotFound,
        /// Voting period has ended
        VotingPeriodEnded,
        /// Already voted on this proposal
        AlreadyVoted,
        /// Cannot vote on own proposal
        CannotVoteOnOwnProposal,
        /// Proposal is not active
        ProposalNotActive,
        /// Maximum number of proposals reached
        TooManyProposals,
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Invalid token type
        InvalidTokenType,
        /// Price stability threshold exceeded
        PriceStabilityThresholdExceeded,
    }

    /// Pallet dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initialize the token system
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn initialize_tokens(
            origin: OriginFor<T>,
            grid_supply: Balance,
            watt_supply: Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Initialize GRID token
            let grid_info = TokenInfo {
                token_type: TokenType::Grid,
                total_supply: grid_supply,
                price: 10000, // 1 USD = 10000 basis points
                is_active: true,
            };
            TokenInfoStorage::<T>::insert(TokenType::Grid, grid_info);

            // Initialize WATT token
            let watt_info = TokenInfo {
                token_type: TokenType::Watt,
                total_supply: watt_supply,
                price: 10000, // 1 USD = 10000 basis points
                is_active: true,
            };
            TokenInfoStorage::<T>::insert(TokenType::Watt, watt_info);

            Ok(())
        }

        /// Mint GRID tokens to an account
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn mint_grid_tokens(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;

            GridBalances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
            
            // Update total supply
            TokenInfoStorage::<T>::mutate(TokenType::Grid, |info| {
                if let Some(token_info) = info {
                    token_info.total_supply = token_info.total_supply.saturating_add(amount);
                }
            });

            Self::deposit_event(Event::GridTokensMinted { account: to, amount });
            Ok(())
        }

        /// Mint WATT tokens to an account
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn mint_watt_tokens(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;

            WattBalances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
            
            // Update total supply
            TokenInfoStorage::<T>::mutate(TokenType::Watt, |info| {
                if let Some(token_info) = info {
                    token_info.total_supply = token_info.total_supply.saturating_add(amount);
                }
            });

            Self::deposit_event(Event::WattTokensMinted { account: to, amount });
            Ok(())
        }

        /// Burn GRID tokens from an account
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn burn_grid_tokens(
            origin: OriginFor<T>,
            from: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let current_balance = GridBalances::<T>::get(&from);
            ensure!(current_balance >= amount, Error::<T>::InsufficientBalance);

            GridBalances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
            
            // Update total supply
            TokenInfoStorage::<T>::mutate(TokenType::Grid, |info| {
                if let Some(token_info) = info {
                    token_info.total_supply = token_info.total_supply.saturating_sub(amount);
                }
            });

            Self::deposit_event(Event::GridTokensBurned { account: from, amount });
            Ok(())
        }

        /// Burn WATT tokens from an account
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn burn_watt_tokens(
            origin: OriginFor<T>,
            from: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let current_balance = WattBalances::<T>::get(&from);
            ensure!(current_balance >= amount, Error::<T>::InsufficientBalance);

            WattBalances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
            
            // Update total supply
            TokenInfoStorage::<T>::mutate(TokenType::Watt, |info| {
                if let Some(token_info) = info {
                    token_info.total_supply = token_info.total_supply.saturating_sub(amount);
                }
            });

            Self::deposit_event(Event::WattTokensBurned { account: from, amount });
            Ok(())
        }

        /// Transfer GRID tokens between accounts
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn transfer_grid_tokens(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            let from_balance = GridBalances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

            GridBalances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
            GridBalances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));

            Self::deposit_event(Event::TokensTransferred { 
                token_type: TokenType::Grid, 
                from, 
                to, 
                amount 
            });
            Ok(())
        }

        /// Transfer WATT tokens between accounts
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn transfer_watt_tokens(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            let from_balance = WattBalances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

            WattBalances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
            WattBalances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));

            Self::deposit_event(Event::TokensTransferred { 
                token_type: TokenType::Watt, 
                from, 
                to, 
                amount 
            });
            Ok(())
        }

        /// Stake GRID tokens for governance participation
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn stake_tokens(
            origin: OriginFor<T>,
            amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(amount >= T::MinStakeAmount::get(), Error::<T>::MinimumStakeNotMet);

            let current_balance = GridBalances::<T>::get(&who);
            ensure!(current_balance >= amount, Error::<T>::InsufficientBalance);

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Update or create stake info
            Stakes::<T>::mutate(&who, |stake_info| {
                if let Some(existing_stake) = stake_info {
                    // Claim pending rewards before increasing stake
                    let pending_rewards = Self::calculate_pending_rewards(&who, existing_stake);
                    existing_stake.unclaimed_rewards = existing_stake.unclaimed_rewards.saturating_add(pending_rewards);
                    existing_stake.amount = existing_stake.amount.saturating_add(amount);
                    existing_stake.last_reward_block = current_block;
                } else {
                    *stake_info = Some(StakeInfo {
                        amount,
                        start_block: current_block,
                        last_reward_block: current_block,
                        unclaimed_rewards: 0,
                    });
                }
            });

            // Transfer tokens from balance to staking
            GridBalances::<T>::mutate(&who, |balance| *balance = balance.saturating_sub(amount));
            TotalStaked::<T>::mutate(|total| *total = total.saturating_add(amount));

            Self::deposit_event(Event::TokensStaked { account: who, amount });
            Ok(())
        }

        /// Unstake GRID tokens
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn unstake_tokens(
            origin: OriginFor<T>,
            amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut stake_info = Stakes::<T>::get(&who).ok_or(Error::<T>::NotStaking)?;
            ensure!(stake_info.amount >= amount, Error::<T>::InsufficientBalance);

            // Calculate and add pending rewards
            let pending_rewards = Self::calculate_pending_rewards(&who, &stake_info);
            stake_info.unclaimed_rewards = stake_info.unclaimed_rewards.saturating_add(pending_rewards);

            // Update stake
            stake_info.amount = stake_info.amount.saturating_sub(amount);
            stake_info.last_reward_block = <frame_system::Pallet<T>>::block_number();

            if stake_info.amount == 0 {
                Stakes::<T>::remove(&who);
            } else {
                Stakes::<T>::insert(&who, stake_info);
            }

            // Return tokens to balance
            GridBalances::<T>::mutate(&who, |balance| *balance = balance.saturating_add(amount));
            TotalStaked::<T>::mutate(|total| *total = total.saturating_sub(amount));

            Self::deposit_event(Event::TokensUnstaked { account: who, amount });
            Ok(())
        }

        /// Claim staking rewards
        #[pallet::call_index(9)]
        #[pallet::weight(10_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut stake_info = Stakes::<T>::get(&who).ok_or(Error::<T>::NotStaking)?;
            
            // Calculate total rewards (pending + unclaimed)
            let pending_rewards = Self::calculate_pending_rewards(&who, &stake_info);
            let total_rewards = stake_info.unclaimed_rewards.saturating_add(pending_rewards);

            ensure!(total_rewards > 0, Error::<T>::InsufficientBalance);

            // Update stake info
            stake_info.unclaimed_rewards = 0;
            stake_info.last_reward_block = <frame_system::Pallet<T>>::block_number();
            Stakes::<T>::insert(&who, stake_info);

            // Mint reward tokens
            GridBalances::<T>::mutate(&who, |balance| *balance = balance.saturating_add(total_rewards));

            Self::deposit_event(Event::RewardsClaimed { account: who, amount: total_rewards });
            Ok(())
        }

        /// Create a governance proposal
        #[pallet::call_index(10)]
        #[pallet::weight(10_000)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            voting_period: T::BlockNumber,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure the proposer has staked tokens
            ensure!(Stakes::<T>::contains_key(&who), Error::<T>::NotStaking);

            let proposal_id = NextProposalId::<T>::get();
            let voting_deadline = <frame_system::Pallet<T>>::block_number() + voting_period;

            let proposal = Proposal {
                id: proposal_id,
                proposer: who.clone(),
                title,
                description,
                voting_deadline,
                votes_for: 0,
                votes_against: 0,
                status: ProposalStatus::Active,
            };

            Proposals::<T>::insert(proposal_id, proposal);
            NextProposalId::<T>::put(proposal_id + 1);

            Self::deposit_event(Event::ProposalCreated { proposal_id, proposer: who });
            Ok(())
        }

        /// Vote on a governance proposal
        #[pallet::call_index(11)]
        #[pallet::weight(10_000)]
        pub fn vote_on_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            vote: bool, // true = for, false = against
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut proposal = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            
            // Check if proposal is active
            ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
            
            // Check if voting period has ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block <= proposal.voting_deadline, Error::<T>::VotingPeriodEnded);

            // Check if already voted
            ensure!(!Votes::<T>::contains_key(proposal_id, &who), Error::<T>::AlreadyVoted);

            // Cannot vote on own proposal
            ensure!(proposal.proposer != who, Error::<T>::CannotVoteOnOwnProposal);

            // Get voting weight (staked amount)
            let stake_info = Stakes::<T>::get(&who).ok_or(Error::<T>::NotStaking)?;
            let voting_weight = stake_info.amount;

            // Record vote
            Votes::<T>::insert(proposal_id, &who, vote);

            // Update proposal vote counts
            if vote {
                proposal.votes_for = proposal.votes_for.saturating_add(voting_weight);
            } else {
                proposal.votes_against = proposal.votes_against.saturating_add(voting_weight);
            }

            Proposals::<T>::insert(proposal_id, proposal);

            Self::deposit_event(Event::VoteCast { proposal_id, voter: who, vote });
            Ok(())
        }

        /// Finalize a governance proposal
        #[pallet::call_index(12)]
        #[pallet::weight(10_000)]
        pub fn finalize_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let mut proposal = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            
            // Check if proposal is active
            ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
            
            // Check if voting period has ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block > proposal.voting_deadline, Error::<T>::VotingPeriodEnded);

            // Determine if proposal passed
            let passed = proposal.votes_for > proposal.votes_against;
            proposal.status = if passed { ProposalStatus::Passed } else { ProposalStatus::Failed };

            Proposals::<T>::insert(proposal_id, proposal);

            Self::deposit_event(Event::ProposalFinalized { proposal_id, passed });
            Ok(())
        }

        /// Update WATT token price (oracle function)
        #[pallet::call_index(13)]
        #[pallet::weight(10_000)]
        pub fn update_watt_price(
            origin: OriginFor<T>,
            new_price: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            TokenInfoStorage::<T>::mutate(TokenType::Watt, |info| {
                if let Some(token_info) = info {
                    let old_price = token_info.price;
                    token_info.price = new_price;
                    
                    // Check price stability
                    let price_change = if new_price > old_price {
                        new_price - old_price
                    } else {
                        old_price - new_price
                    };
                    
                    let price_change_percentage = Permill::from_rational(price_change, old_price);
                    
                    // If price change exceeds threshold, trigger stability mechanism
                    if price_change_percentage > T::PriceStabilityThreshold::get() {
                        // In a real implementation, this would trigger mint/burn operations
                        // to maintain price stability
                    }
                }
            });

            Self::deposit_event(Event::WattPriceUpdated { new_price });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Calculate pending rewards for a staker
        fn calculate_pending_rewards(who: &T::AccountId, stake_info: &StakeInfo) -> Balance {
            let current_block = <frame_system::Pallet<T>>::block_number();
            let blocks_since_last_reward = current_block.saturating_sub(stake_info.last_reward_block);
            
            // Convert block number to u64 for calculation
            let blocks_as_u64 = blocks_since_last_reward.saturated_into::<u64>();
            
            // Calculate rewards based on annual rate
            // Assuming 6-second block time, ~5,256,000 blocks per year
            let annual_blocks = 5_256_000u64;
            let annual_rate = T::StakingRewardRate::get().deconstruct() as u64;
            
            let rewards = stake_info.amount
                .saturating_mul(annual_rate)
                .saturating_mul(blocks_as_u64)
                .saturating_div(annual_blocks)
                .saturating_div(100); // Convert percentage to decimal
            
            rewards
        }

        /// Get account's GRID token balance
        pub fn get_grid_balance(who: &T::AccountId) -> Balance {
            GridBalances::<T>::get(who)
        }

        /// Get account's WATT token balance
        pub fn get_watt_balance(who: &T::AccountId) -> Balance {
            WattBalances::<T>::get(who)
        }

        /// Get account's staking information
        pub fn get_stake_info(who: &T::AccountId) -> Option<StakeInfo> {
            Stakes::<T>::get(who)
        }

        /// Get token information
        pub fn get_token_info(token_type: TokenType) -> Option<TokenInfo> {
            TokenInfoStorage::<T>::get(token_type)
        }

        /// Get proposal information
        pub fn get_proposal(proposal_id: ProposalId) -> Option<Proposal> {
            Proposals::<T>::get(proposal_id)
        }

        /// Check if account has voted on a proposal
        pub fn has_voted(proposal_id: ProposalId, who: &T::AccountId) -> bool {
            Votes::<T>::contains_key(proposal_id, who)
        }
    }
}

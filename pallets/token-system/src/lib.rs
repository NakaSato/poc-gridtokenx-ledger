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
    GridMinted { to: AccountId, amount: Balance },
    /// WATT tokens were minted
    WattMinted { to: AccountId, amount: Balance },
    /// GRID tokens were burned
    GridBurned { from: AccountId, amount: Balance },
    /// WATT tokens were burned
    WattBurned { from: AccountId, amount: Balance },
    /// GRID tokens were transferred
    GridTransferred { from: AccountId, to: AccountId, amount: Balance },
    /// WATT tokens were transferred
    WattTransferred { from: AccountId, to: AccountId, amount: Balance },
    /// Tokens were staked
    Staked { who: AccountId, amount: Balance },
    /// Tokens were unstaked
    Unstaked { who: AccountId, amount: Balance },
    /// Staking rewards were claimed
    RewardsClaimed { who: AccountId, amount: Balance },
    /// New proposal was created
    ProposalCreated { id: ProposalId, proposer: AccountId, title: String },
    /// Vote was cast on a proposal
    VoteCast { proposal_id: ProposalId, voter: AccountId, support: bool, power: Balance },
    /// Proposal was finalized
    ProposalFinalized { id: ProposalId, status: ProposalStatus },
    /// Token price was updated
    PriceUpdated { token_type: TokenType, new_price: u32 },
}

/// Errors that can occur in the token system
#[derive(Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Token not found")]
    TokenNotFound,
    #[error("Proposal not found")]
    ProposalNotFound,
    #[error("Proposal not active")]
    ProposalNotActive,
    #[error("Already voted")]
    AlreadyVoted,
    #[error("Voting period ended")]
    VotingPeriodEnded,
    #[error("Cannot vote on own proposal")]
    CannotVoteOnOwnProposal,
    #[error("Not staking")]
    NotStaking,
    #[error("Minimum stake amount not met")]
    MinimumStakeNotMet,
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("No rewards to claim")]
    NoRewardsToClaim,
    #[error("Unauthorized operation")]
    Unauthorized,
}

/// Token system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSystemConfig {
    /// Minimum stake amount required
    pub min_stake_amount: Balance,
    /// Annual staking reward rate (in basis points)
    pub staking_reward_rate: u32,
    /// Voting period duration (in blocks)
    pub voting_period: BlockNumber,
    /// Minimum GRID balance for creating proposals
    pub min_proposal_balance: Balance,
    /// Grid token initial supply
    pub grid_initial_supply: Balance,
    /// WATT token initial supply
    pub watt_initial_supply: Balance,
}

impl Default for TokenSystemConfig {
    fn default() -> Self {
        Self {
            min_stake_amount: 1000,
            staking_reward_rate: 800, // 8% annual
            voting_period: 100,
            min_proposal_balance: 10000,
            grid_initial_supply: 1_000_000_000,
            watt_initial_supply: 1_000_000_000,
        }
    }
}

/// Main token system implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSystem {
    /// Configuration
    pub config: TokenSystemConfig,
    /// GRID token balances
    pub grid_balances: HashMap<AccountId, Balance>,
    /// WATT token balances
    pub watt_balances: HashMap<AccountId, Balance>,
    /// Token information
    pub token_info: HashMap<TokenType, TokenInfo>,
    /// Staking information
    pub stakes: HashMap<AccountId, StakeInfo>,
    /// Governance proposals
    pub proposals: HashMap<ProposalId, Proposal>,
    /// Votes on proposals
    pub votes: HashMap<(ProposalId, AccountId), Vote>,
    /// Total staked amount
    pub total_staked: Balance,
    /// Next proposal ID
    pub next_proposal_id: ProposalId,
    /// Current block number
    pub current_block: BlockNumber,
    /// Event history
    pub events: Vec<Event>,
}

impl Default for TokenSystem {
    fn default() -> Self {
        let config = TokenSystemConfig::default();
        let mut token_info = HashMap::new();
        
        // Initialize GRID token
        token_info.insert(TokenType::Grid, TokenInfo {
            token_type: TokenType::Grid,
            total_supply: config.grid_initial_supply,
            price: 10000, // 1 USD = 10000 basis points
            is_active: true,
        });
        
        // Initialize WATT token
        token_info.insert(TokenType::Watt, TokenInfo {
            token_type: TokenType::Watt,
            total_supply: config.watt_initial_supply,
            price: 10000, // 1 USD = 10000 basis points
            is_active: true,
        });
        
        Self {
            config,
            grid_balances: HashMap::new(),
            watt_balances: HashMap::new(),
            token_info,
            stakes: HashMap::new(),
            proposals: HashMap::new(),
            votes: HashMap::new(),
            total_staked: 0,
            next_proposal_id: 1,
            current_block: 0,
            events: Vec::new(),
        }
    }
}

impl TokenSystem {
    /// Create a new token system with given configuration
    pub fn new(config: TokenSystemConfig) -> Self {
        let mut system = Self::default();
        system.config = config;
        system
    }

    /// Advance the current block number
    pub fn advance_block(&mut self) {
        self.current_block += 1;
    }

    /// Set the current block number
    pub fn set_block(&mut self, block: BlockNumber) {
        self.current_block = block;
    }

    /// Get GRID token balance for an account
    pub fn grid_balance(&self, account: &AccountId) -> Balance {
        self.grid_balances.get(account).copied().unwrap_or(0)
    }

    /// Get WATT token balance for an account
    pub fn watt_balance(&self, account: &AccountId) -> Balance {
        self.watt_balances.get(account).copied().unwrap_or(0)
    }

    /// Get token information
    pub fn get_token_info(&self, token_type: &TokenType) -> Option<&TokenInfo> {
        self.token_info.get(token_type)
    }

    /// Get staking information for an account
    pub fn get_stake_info(&self, account: &AccountId) -> Option<&StakeInfo> {
        self.stakes.get(account)
    }

    /// Get proposal by ID
    pub fn get_proposal(&self, id: ProposalId) -> Option<&Proposal> {
        self.proposals.get(&id)
    }

    /// Get vote information
    pub fn get_vote(&self, proposal_id: ProposalId, voter: &AccountId) -> Option<&Vote> {
        self.votes.get(&(proposal_id, voter.clone()))
    }

    /// Emit an event
    fn emit_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Mint GRID tokens
    pub fn mint_grid(&mut self, to: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let current_balance = self.grid_balance(to);
        self.grid_balances.insert(to.clone(), current_balance + amount);
        
        // Update total supply
        if let Some(info) = self.token_info.get_mut(&TokenType::Grid) {
            info.total_supply += amount;
        }
        
        self.emit_event(Event::GridMinted { to: to.clone(), amount });
        Ok(())
    }

    /// Mint WATT tokens
    pub fn mint_watt(&mut self, to: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let current_balance = self.watt_balance(to);
        self.watt_balances.insert(to.clone(), current_balance + amount);
        
        // Update total supply
        if let Some(info) = self.token_info.get_mut(&TokenType::Watt) {
            info.total_supply += amount;
        }
        
        self.emit_event(Event::WattMinted { to: to.clone(), amount });
        Ok(())
    }

    /// Burn GRID tokens
    pub fn burn_grid(&mut self, from: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let current_balance = self.grid_balance(from);
        if current_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        self.grid_balances.insert(from.clone(), current_balance - amount);
        
        // Update total supply
        if let Some(info) = self.token_info.get_mut(&TokenType::Grid) {
            info.total_supply = info.total_supply.saturating_sub(amount);
        }
        
        self.emit_event(Event::GridBurned { from: from.clone(), amount });
        Ok(())
    }

    /// Burn WATT tokens
    pub fn burn_watt(&mut self, from: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let current_balance = self.watt_balance(from);
        if current_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        self.watt_balances.insert(from.clone(), current_balance - amount);
        
        // Update total supply
        if let Some(info) = self.token_info.get_mut(&TokenType::Watt) {
            info.total_supply = info.total_supply.saturating_sub(amount);
        }
        
        self.emit_event(Event::WattBurned { from: from.clone(), amount });
        Ok(())
    }

    /// Transfer GRID tokens
    pub fn transfer_grid(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let from_balance = self.grid_balance(from);
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        let to_balance = self.grid_balance(to);
        self.grid_balances.insert(from.clone(), from_balance - amount);
        self.grid_balances.insert(to.clone(), to_balance + amount);
        
        self.emit_event(Event::GridTransferred { 
            from: from.clone(), 
            to: to.clone(), 
            amount 
        });
        Ok(())
    }

    /// Transfer WATT tokens
    pub fn transfer_watt(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let from_balance = self.watt_balance(from);
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        let to_balance = self.watt_balance(to);
        self.watt_balances.insert(from.clone(), from_balance - amount);
        self.watt_balances.insert(to.clone(), to_balance + amount);
        
        self.emit_event(Event::WattTransferred { 
            from: from.clone(), 
            to: to.clone(), 
            amount 
        });
        Ok(())
    }

    /// Stake GRID tokens
    pub fn stake(&mut self, who: &AccountId, amount: Balance) -> Result<(), TokenError> {
        if amount < self.config.min_stake_amount {
            return Err(TokenError::MinimumStakeNotMet);
        }
        
        let current_balance = self.grid_balance(who);
        if current_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        // Remove tokens from balance
        self.grid_balances.insert(who.clone(), current_balance - amount);
        
        // Add or update stake
        if let Some(stake_info) = self.stakes.get_mut(who) {
            stake_info.amount += amount;
        } else {
            self.stakes.insert(who.clone(), StakeInfo {
                amount,
                start_block: self.current_block,
                last_reward_block: self.current_block,
                unclaimed_rewards: 0,
            });
        }
        
        self.total_staked += amount;
        self.emit_event(Event::Staked { who: who.clone(), amount });
        Ok(())
    }

    /// Unstake GRID tokens
    pub fn unstake(&mut self, who: &AccountId, amount: Balance) -> Result<(), TokenError> {
        let stake_info = self.stakes.get_mut(who).ok_or(TokenError::NotStaking)?;
        
        if stake_info.amount < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        // Update stake info
        stake_info.amount -= amount;
        if stake_info.amount == 0 {
            self.stakes.remove(who);
        }
        
        // Return tokens to balance
        let current_balance = self.grid_balance(who);
        self.grid_balances.insert(who.clone(), current_balance + amount);
        
        self.total_staked -= amount;
        self.emit_event(Event::Unstaked { who: who.clone(), amount });
        Ok(())
    }

    /// Calculate rewards for a staker
    pub fn calculate_rewards(&self, who: &AccountId) -> Balance {
        if let Some(stake_info) = self.stakes.get(who) {
            let blocks_elapsed = self.current_block.saturating_sub(stake_info.last_reward_block);
            if blocks_elapsed > 0 {
                // Simple calculation: reward = stake * rate * blocks / (blocks_per_year)
                // Assuming 6000 blocks per year as an example
                let reward = stake_info.amount * self.config.staking_reward_rate as u64 * blocks_elapsed / (6000 * 10000);
                return reward + stake_info.unclaimed_rewards;
            }
        }
        0
    }

    /// Claim staking rewards
    pub fn claim_rewards(&mut self, who: &AccountId) -> Result<(), TokenError> {
        let total_rewards = self.calculate_rewards(who);
        if total_rewards == 0 {
            return Err(TokenError::NoRewardsToClaim);
        }
        
        // Update stake info
        if let Some(stake_info) = self.stakes.get_mut(who) {
            stake_info.last_reward_block = self.current_block;
            stake_info.unclaimed_rewards = 0;
        }
        
        // Mint rewards as GRID tokens
        self.mint_grid(who, total_rewards)?;
        
        self.emit_event(Event::RewardsClaimed { who: who.clone(), amount: total_rewards });
        Ok(())
    }

    /// Create a new governance proposal
    pub fn create_proposal(
        &mut self, 
        proposer: &AccountId, 
        title: String, 
        description: String
    ) -> Result<ProposalId, TokenError> {
        // Check if proposer has enough GRID tokens
        if self.grid_balance(proposer) < self.config.min_proposal_balance {
            return Err(TokenError::InsufficientBalance);
        }
        
        let proposal_id = self.next_proposal_id;
        let voting_deadline = self.current_block + self.config.voting_period;
        
        let proposal = Proposal {
            id: proposal_id,
            proposer: proposer.clone(),
            title: title.clone(),
            description,
            voting_deadline,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
        };
        
        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;
        
        self.emit_event(Event::ProposalCreated { 
            id: proposal_id, 
            proposer: proposer.clone(), 
            title 
        });
        
        Ok(proposal_id)
    }

    /// Vote on a proposal
    pub fn vote(
        &mut self, 
        proposal_id: ProposalId, 
        voter: &AccountId, 
        support: bool
    ) -> Result<(), TokenError> {
        // Check if proposal exists and is active
        let proposal = self.proposals.get(&proposal_id).ok_or(TokenError::ProposalNotFound)?;
        
        if proposal.status != ProposalStatus::Active {
            return Err(TokenError::ProposalNotActive);
        }
        
        if self.current_block > proposal.voting_deadline {
            return Err(TokenError::VotingPeriodEnded);
        }
        
        // Check if already voted
        if self.votes.contains_key(&(proposal_id, voter.clone())) {
            return Err(TokenError::AlreadyVoted);
        }
        
        // Cannot vote on own proposal
        if &proposal.proposer == voter {
            return Err(TokenError::CannotVoteOnOwnProposal);
        }
        
        // Voting power equals staked amount
        let voting_power = self.stakes.get(voter).map(|s| s.amount).unwrap_or(0);
        
        if voting_power == 0 {
            return Err(TokenError::NotStaking);
        }
        
        // Record vote
        let vote = Vote {
            voter: voter.clone(),
            proposal_id,
            support,
            voting_power,
        };
        
        self.votes.insert((proposal_id, voter.clone()), vote);
        
        // Update proposal vote counts
        let proposal = self.proposals.get_mut(&proposal_id).unwrap();
        if support {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }
        
        self.emit_event(Event::VoteCast { 
            proposal_id, 
            voter: voter.clone(), 
            support, 
            power: voting_power 
        });
        
        Ok(())
    }

    /// Finalize a proposal (check if it passed and update status)
    pub fn finalize_proposal(&mut self, proposal_id: ProposalId) -> Result<(), TokenError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(TokenError::ProposalNotFound)?;
        
        if proposal.status != ProposalStatus::Active {
            return Err(TokenError::ProposalNotActive);
        }
        
        // Check if voting period has ended
        if self.current_block <= proposal.voting_deadline {
            return Err(TokenError::VotingPeriodEnded);
        }
        
        // Determine result
        let new_status = if proposal.votes_for > proposal.votes_against {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Failed
        };
        
        proposal.status = new_status.clone();
        
        self.emit_event(Event::ProposalFinalized { 
            id: proposal_id, 
            status: new_status 
        });
        
        Ok(())
    }

    /// Update token price (for price stability mechanisms)
    pub fn update_token_price(&mut self, token_type: TokenType, new_price: u32) -> Result<(), TokenError> {
        if let Some(info) = self.token_info.get_mut(&token_type) {
            info.price = new_price;
            self.emit_event(Event::PriceUpdated { token_type, new_price });
            Ok(())
        } else {
            Err(TokenError::TokenNotFound)
        }
    }

    /// Get total supply of a token
    pub fn total_supply(&self, token_type: &TokenType) -> Balance {
        self.token_info.get(token_type).map(|info| info.total_supply).unwrap_or(0)
    }

    /// Get total staked amount
    pub fn total_staked(&self) -> Balance {
        self.total_staked
    }

    /// Get all events
    pub fn get_events(&self) -> &[Event] {
        &self.events
    }

    /// Clear events (for testing)
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_system() -> TokenSystem {
        let mut system = TokenSystem::default();
        // Give some initial tokens for testing
        system.mint_grid(&"alice".to_string(), 100000).unwrap();
        system.mint_grid(&"bob".to_string(), 50000).unwrap();
        system.mint_watt(&"alice".to_string(), 10000).unwrap();
        system.mint_watt(&"bob".to_string(), 5000).unwrap();
        system.clear_events();
        system
    }

    #[test]
    fn test_mint_tokens() {
        let mut system = TokenSystem::default();
        
        // Mint GRID tokens
        system.mint_grid(&"alice".to_string(), 1000).unwrap();
        assert_eq!(system.grid_balance(&"alice".to_string()), 1000);
        
        // Mint WATT tokens
        system.mint_watt(&"alice".to_string(), 500).unwrap();
        assert_eq!(system.watt_balance(&"alice".to_string()), 500);
        
        // Check events
        assert_eq!(system.events.len(), 2);
    }

    #[test]
    fn test_burn_tokens() {
        let mut system = create_test_system();
        
        // Burn GRID tokens
        system.burn_grid(&"alice".to_string(), 1000).unwrap();
        assert_eq!(system.grid_balance(&"alice".to_string()), 99000);
        
        // Test insufficient balance
        assert_eq!(
            system.burn_grid(&"alice".to_string(), 200000),
            Err(TokenError::InsufficientBalance)
        );
    }

    #[test]
    fn test_transfer_tokens() {
        let mut system = create_test_system();
        
        // Transfer GRID tokens
        system.transfer_grid(&"alice".to_string(), &"bob".to_string(), 1000).unwrap();
        assert_eq!(system.grid_balance(&"alice".to_string()), 99000);
        assert_eq!(system.grid_balance(&"bob".to_string()), 51000);
        
        // Test insufficient balance
        assert_eq!(
            system.transfer_grid(&"alice".to_string(), &"bob".to_string(), 200000),
            Err(TokenError::InsufficientBalance)
        );
    }

    #[test]
    fn test_staking() {
        let mut system = create_test_system();
        
        // Stake tokens
        system.stake(&"alice".to_string(), 10000).unwrap();
        assert_eq!(system.grid_balance(&"alice".to_string()), 90000);
        assert_eq!(system.get_stake_info(&"alice".to_string()).unwrap().amount, 10000);
        assert_eq!(system.total_staked(), 10000);
        
        // Test minimum stake amount
        assert_eq!(
            system.stake(&"alice".to_string(), 500),
            Err(TokenError::MinimumStakeNotMet)
        );
    }

    #[test]
    fn test_unstaking() {
        let mut system = create_test_system();
        
        // Stake and then unstake
        system.stake(&"alice".to_string(), 10000).unwrap();
        system.unstake(&"alice".to_string(), 5000).unwrap();
        
        assert_eq!(system.grid_balance(&"alice".to_string()), 95000);
        assert_eq!(system.get_stake_info(&"alice".to_string()).unwrap().amount, 5000);
        assert_eq!(system.total_staked(), 5000);
    }

    #[test]
    fn test_rewards() {
        let mut system = create_test_system();
        
        // Stake tokens
        system.stake(&"alice".to_string(), 10000).unwrap();
        
        // Advance some blocks
        system.set_block(100);
        
        // Calculate rewards
        let rewards = system.calculate_rewards(&"alice".to_string());
        assert!(rewards > 0);
        
        // Claim rewards
        let initial_balance = system.grid_balance(&"alice".to_string());
        system.claim_rewards(&"alice".to_string()).unwrap();
        assert!(system.grid_balance(&"alice".to_string()) > initial_balance);
    }

    #[test]
    fn test_governance_proposal() {
        let mut system = create_test_system();
        
        // Create proposal
        let proposal_id = system.create_proposal(
            &"alice".to_string(),
            "Test Proposal".to_string(),
            "This is a test proposal".to_string()
        ).unwrap();
        
        assert_eq!(proposal_id, 1);
        let proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[test]
    fn test_voting() {
        let mut system = create_test_system();
        
        // Stake tokens for voting power
        system.stake(&"alice".to_string(), 10000).unwrap();
        system.stake(&"bob".to_string(), 5000).unwrap();
        
        // Create proposal
        let proposal_id = system.create_proposal(
            &"alice".to_string(),
            "Test Proposal".to_string(),
            "This is a test proposal".to_string()
        ).unwrap();
        
        // Vote (should fail - cannot vote on own proposal)
        assert_eq!(
            system.vote(proposal_id, &"alice".to_string(), true),
            Err(TokenError::CannotVoteOnOwnProposal)
        );
        
        // Bob votes
        system.vote(proposal_id, &"bob".to_string(), true).unwrap();
        
        let proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 5000);
        assert_eq!(proposal.votes_against, 0);
    }

    #[test]
    fn test_proposal_finalization() {
        let mut system = create_test_system();
        
        // Setup for voting
        system.stake(&"alice".to_string(), 10000).unwrap();
        system.stake(&"bob".to_string(), 5000).unwrap();
        
        let proposal_id = system.create_proposal(
            &"alice".to_string(),
            "Test Proposal".to_string(),
            "This is a test proposal".to_string()
        ).unwrap();
        
        // Vote
        system.vote(proposal_id, &"bob".to_string(), true).unwrap();
        
        // Try to finalize before voting period ends (should fail)
        assert_eq!(
            system.finalize_proposal(proposal_id),
            Err(TokenError::VotingPeriodEnded)
        );
        
        // Advance past voting period
        system.set_block(200);
        
        // Finalize proposal
        system.finalize_proposal(proposal_id).unwrap();
        
        let proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }

    #[test]
    fn test_price_updates() {
        let mut system = create_test_system();
        
        // Update GRID token price
        system.update_token_price(TokenType::Grid, 15000).unwrap();
        
        let grid_info = system.get_token_info(&TokenType::Grid).unwrap();
        assert_eq!(grid_info.price, 15000);
        
        // Check event
        assert_eq!(system.events.len(), 1);
        if let Event::PriceUpdated { token_type, new_price } = &system.events[0] {
            assert_eq!(*token_type, TokenType::Grid);
            assert_eq!(*new_price, 15000);
        } else {
            panic!("Expected PriceUpdated event");
        }
    }

    #[test]
    fn test_total_supply() {
        let system = create_test_system();
        
        // Check initial total supplies
        assert_eq!(system.total_supply(&TokenType::Grid), 1_000_000_000 + 150000); // Initial + minted
        assert_eq!(system.total_supply(&TokenType::Watt), 1_000_000_000 + 15000); // Initial + minted
    }

    #[test]
    fn test_error_handling() {
        let mut system = create_test_system();
        
        // Test various error conditions
        assert_eq!(
            system.transfer_grid(&"alice".to_string(), &"bob".to_string(), 200000),
            Err(TokenError::InsufficientBalance)
        );
        
        assert_eq!(
            system.get_proposal(999),
            None
        );
        
        assert_eq!(
            system.unstake(&"charlie".to_string(), 1000),
            Err(TokenError::NotStaking)
        );
    }

    #[test]
    fn test_comprehensive_workflow() {
        let mut system = create_test_system();
        
        // 1. Users stake tokens
        system.stake(&"alice".to_string(), 20000).unwrap();
        system.stake(&"bob".to_string(), 10000).unwrap();
        
        // 2. Create a proposal
        let proposal_id = system.create_proposal(
            &"alice".to_string(),
            "Increase staking rewards".to_string(),
            "Proposal to increase staking rewards by 1%".to_string()
        ).unwrap();
        
        // 3. Vote on the proposal
        system.vote(proposal_id, &"bob".to_string(), true).unwrap();
        
        // 4. Advance time and claim rewards
        system.set_block(50);
        system.claim_rewards(&"alice".to_string()).unwrap();
        
        // 5. Finalize the proposal
        system.set_block(150);
        system.finalize_proposal(proposal_id).unwrap();
        
        // 6. Update token price (simulate market activity)
        system.update_token_price(TokenType::Watt, 9500).unwrap();
        
        // 7. Transfer tokens
        system.transfer_watt(&"alice".to_string(), &"bob".to_string(), 1000).unwrap();
        
        // Verify final state
        assert!(system.grid_balance(&"alice".to_string()) > 80000); // Original + rewards
        assert_eq!(system.watt_balance(&"alice".to_string()), 9000); // 10000 - 1000 transferred
        assert_eq!(system.watt_balance(&"bob".to_string()), 6000); // 5000 + 1000 received
        
        let proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed);
        
        // Check that events were emitted
        assert!(system.events.len() > 0);
    }
}

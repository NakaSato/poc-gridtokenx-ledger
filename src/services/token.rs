// Token system service
// This module handles token creation, management, and governance

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type aliases for clarity
pub type TokenId = String;
pub type TokenAmount = Balance;

/// Token types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Energy,
    Governance,
    Utility,
    Reward,
}

/// Token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: TokenAmount,
    pub description: String,
    pub icon_url: Option<String>,
}

/// Token balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub token_id: TokenId,
    pub balance: TokenAmount,
    pub locked: TokenAmount,
    pub staked: TokenAmount,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: AccountId,
    pub voting_period: u64,
    pub execution_period: u64,
    pub threshold: TokenAmount,
    pub votes_for: TokenAmount,
    pub votes_against: TokenAmount,
    pub votes_abstain: TokenAmount,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_ends_at: u64,
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

/// Vote type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vote {
    For,
    Against,
    Abstain,
}

/// Staking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInfo {
    pub staker: AccountId,
    pub amount: TokenAmount,
    pub reward_rate: f64,
    pub staked_at: u64,
    pub lock_period: u64,
    pub rewards_earned: TokenAmount,
}

/// Token service for managing tokens and governance
pub struct TokenService {
    /// Token metadata storage
    pub tokens: HashMap<TokenId, TokenMetadata>,
    /// Account balances
    pub balances: HashMap<(AccountId, TokenId), TokenBalance>,
    /// Governance proposals
    pub proposals: HashMap<String, GovernanceProposal>,
    /// Voting records
    pub votes: HashMap<(AccountId, String), Vote>,
    /// Staking information
    pub staking: HashMap<AccountId, Vec<StakingInfo>>,
    /// Token allowances
    pub allowances: HashMap<(AccountId, AccountId, TokenId), TokenAmount>,
}

impl TokenService {
    /// Create a new token service
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            balances: HashMap::new(),
            proposals: HashMap::new(),
            votes: HashMap::new(),
            staking: HashMap::new(),
            allowances: HashMap::new(),
        }
    }

    /// Create a new token
    pub fn create_token(&mut self, creator: &AccountId, metadata: TokenMetadata) -> CoreResult<TokenId> {
        let token_id = format!("token_{}", self.tokens.len() + 1);
        
        self.tokens.insert(token_id.clone(), metadata.clone());
        
        // Mint initial supply to creator
        self.mint_tokens(&token_id, creator, metadata.total_supply)?;
        
        Ok(token_id)
    }

    /// Mint tokens to an account
    pub fn mint_tokens(&mut self, token_id: &TokenId, to: &AccountId, amount: TokenAmount) -> CoreResult<()> {
        if !self.tokens.contains_key(token_id) {
            return Err(CoreError::InvalidInput("Token does not exist".to_string()));
        }

        let balance_key = (to.clone(), token_id.clone());
        let balance = self.balances.entry(balance_key).or_insert_with(|| TokenBalance {
            token_id: token_id.clone(),
            balance: 0,
            locked: 0,
            staked: 0,
        });

        balance.balance += amount;
        Ok(())
    }

    /// Transfer tokens between accounts
    pub fn transfer_tokens(
        &mut self,
        token_id: &TokenId,
        from: &AccountId,
        to: &AccountId,
        amount: TokenAmount,
    ) -> CoreResult<()> {
        if !self.tokens.contains_key(token_id) {
            return Err(CoreError::InvalidInput("Token does not exist".to_string()));
        }

        // Check balance
        let from_balance_key = (from.clone(), token_id.clone());
        let from_balance = self.balances.get_mut(&from_balance_key)
            .ok_or_else(|| CoreError::InvalidInput("Insufficient balance".to_string()))?;

        if from_balance.balance < amount {
            return Err(CoreError::InvalidInput("Insufficient balance".to_string()));
        }

        // Update balances
        from_balance.balance -= amount;

        let to_balance_key = (to.clone(), token_id.clone());
        let to_balance = self.balances.entry(to_balance_key).or_insert_with(|| TokenBalance {
            token_id: token_id.clone(),
            balance: 0,
            locked: 0,
            staked: 0,
        });

        to_balance.balance += amount;
        Ok(())
    }

    /// Get token balance
    pub fn get_balance(&self, account: &AccountId, token_id: &TokenId) -> TokenAmount {
        let balance_key = (account.clone(), token_id.clone());
        self.balances.get(&balance_key)
            .map(|b| b.balance)
            .unwrap_or(0)
    }

    /// Create governance proposal
    pub fn create_proposal(
        &mut self,
        proposer: &AccountId,
        title: String,
        description: String,
        voting_period: u64,
    ) -> CoreResult<String> {
        let proposal_id = format!("proposal_{}", self.proposals.len() + 1);
        let now = chrono::Utc::now().timestamp() as u64;

        let proposal = GovernanceProposal {
            id: proposal_id.clone(),
            title,
            description,
            proposer: proposer.clone(),
            voting_period,
            execution_period: 7 * 24 * 60 * 60, // 7 days
            threshold: 1000, // Minimum votes required
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            status: ProposalStatus::Active,
            created_at: now,
            voting_ends_at: now + voting_period,
        };

        self.proposals.insert(proposal_id.clone(), proposal);
        Ok(proposal_id)
    }

    /// Vote on a proposal
    pub fn vote_on_proposal(&mut self, voter: &AccountId, proposal_id: &str, vote: Vote) -> CoreResult<()> {
        // Get voter's token balance first to avoid borrow checker issues
        let voting_power = self.get_total_balance(voter);
        
        let proposal = self.proposals.get_mut(proposal_id)
            .ok_or_else(|| CoreError::InvalidInput("Proposal not found".to_string()))?;

        // Check if voting period is still active
        let now = chrono::Utc::now().timestamp() as u64;
        if now > proposal.voting_ends_at {
            return Err(CoreError::InvalidInput("Voting period has ended".to_string()));
        }

        // Check if user has already voted
        let vote_key = (voter.clone(), proposal_id.to_string());
        if self.votes.contains_key(&vote_key) {
            return Err(CoreError::InvalidInput("Already voted".to_string()));
        }
        
        // Record vote
        match vote {
            Vote::For => proposal.votes_for += voting_power,
            Vote::Against => proposal.votes_against += voting_power,
            Vote::Abstain => proposal.votes_abstain += voting_power,
        }

        self.votes.insert(vote_key, vote);
        Ok(())
    }

    /// Get total token balance for account (across all tokens)
    pub fn get_total_balance(&self, account: &AccountId) -> TokenAmount {
        self.balances.iter()
            .filter(|((acc, _), _)| acc == account)
            .map(|(_, balance)| balance.balance)
            .sum()
    }

    /// Get staking information for account
    pub fn get_staking_info(&self, account: &AccountId) -> Vec<StakingInfo> {
        self.staking.get(account).cloned().unwrap_or_default()
    }

    /// Mint tokens (simplified interface)
    pub fn mint(&mut self, account: &AccountId, token_id: &str, amount: TokenAmount) -> CoreResult<()> {
        self.mint_tokens(&token_id.to_string(), account, amount)
    }

    /// Transfer tokens (simplified interface)
    pub fn transfer(&mut self, from: &AccountId, to: &AccountId, token_id: &str, amount: TokenAmount) -> CoreResult<()> {
        self.transfer_tokens(&token_id.to_string(), from, to, amount)
    }

    /// Stake tokens (simplified interface)
    pub fn stake(&mut self, account: &AccountId, amount: TokenAmount, _validator: Option<&AccountId>) -> CoreResult<()> {
        // Use a default governance token for staking
        self.stake_tokens(account, &"governance_token".to_string(), amount)
    }

    /// Unstake tokens (simplified interface)
    pub fn unstake(&mut self, account: &AccountId, amount: TokenAmount) -> CoreResult<()> {
        self.unstake_tokens(account, &"governance_token".to_string(), amount)
    }

    /// Stake tokens
    pub fn stake_tokens(&mut self, account: &AccountId, token_id: &TokenId, amount: TokenAmount) -> CoreResult<()> {
        if !self.tokens.contains_key(token_id) {
            return Err(CoreError::InvalidInput("Token does not exist".to_string()));
        }

        let balance_key = (account.clone(), token_id.clone());
        let balance = self.balances.get_mut(&balance_key)
            .ok_or_else(|| CoreError::InvalidInput("Insufficient balance".to_string()))?;

        if balance.balance < amount {
            return Err(CoreError::InvalidInput("Insufficient balance".to_string()));
        }

        balance.balance -= amount;
        balance.staked += amount;

        // Create staking info
        let staking_info = StakingInfo {
            staker: account.clone(),
            amount,
            reward_rate: 0.05, // 5% annual reward
            staked_at: chrono::Utc::now().timestamp() as u64,
            lock_period: 30 * 24 * 60 * 60, // 30 days
            rewards_earned: 0,
        };

        self.staking.entry(account.clone()).or_insert_with(Vec::new).push(staking_info);
        Ok(())
    }

    /// Unstake tokens
    pub fn unstake_tokens(&mut self, account: &AccountId, token_id: &TokenId, amount: TokenAmount) -> CoreResult<()> {
        if !self.tokens.contains_key(token_id) {
            return Err(CoreError::InvalidInput("Token does not exist".to_string()));
        }

        let balance_key = (account.clone(), token_id.clone());
        let balance = self.balances.get_mut(&balance_key)
            .ok_or_else(|| CoreError::InvalidInput("No staked tokens".to_string()))?;

        if balance.staked < amount {
            return Err(CoreError::InvalidInput("Insufficient staked tokens".to_string()));
        }

        balance.staked -= amount;
        balance.balance += amount;

        // Remove from staking info (simplified)
        if let Some(staking_infos) = self.staking.get_mut(account) {
            staking_infos.retain(|info| info.amount != amount);
        }

        Ok(())
    }

    /// Get proposal details
    pub fn get_proposal(&self, proposal_id: &str) -> Option<&GovernanceProposal> {
        self.proposals.get(proposal_id)
    }
}

impl Default for TokenService {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSystem {
    pub grid_token: GridToken,
    pub watt_token: WattToken,
    pub user_balances: HashMap<String, UserTokenBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridToken {
    pub name: String,
    pub symbol: String,
    pub total_supply: f64,
    pub circulating_supply: f64,
    pub staking_rewards_rate: f64, // Annual percentage yield
    pub governance_proposals: Vec<GovernanceProposal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WattToken {
    pub name: String,
    pub symbol: String,
    pub fiat_peg: FiatCurrency,
    pub total_supply: f64,
    pub fiat_reserves: f64, // Amount of fiat backing the stablecoin
    pub mint_burn_history: Vec<MintBurnRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenBalance {
    pub address: String,
    pub grid_balance: f64,
    pub watt_balance: f64,
    pub staked_grid: f64,
    pub staking_rewards: f64,
    pub last_reward_claim: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: f64,
    pub votes_against: f64,
    pub voting_deadline: chrono::DateTime<chrono::Utc>,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintBurnRecord {
    pub transaction_id: String,
    pub operation: MintBurnOperation,
    pub amount: f64,
    pub fiat_amount: f64,
    pub user_address: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MintBurnOperation {
    Mint,
    Burn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FiatCurrency {
    USD,
    EUR,
    GBP,
    JPY,
    AUD,
}

impl TokenSystem {
    pub fn new() -> Self {
        TokenSystem {
            grid_token: GridToken::new(),
            watt_token: WattToken::new(FiatCurrency::USD),
            user_balances: HashMap::new(),
        }
    }

    pub fn create_user_account(&mut self, address: String) -> Result<(), String> {
        if self.user_balances.contains_key(&address) {
            return Err("User account already exists".to_string());
        }

        let user_balance = UserTokenBalance {
            address: address.clone(),
            grid_balance: 0.0,
            watt_balance: 0.0,
            staked_grid: 0.0,
            staking_rewards: 0.0,
            last_reward_claim: chrono::Utc::now(),
        };

        self.user_balances.insert(address, user_balance);
        Ok(())
    }

    pub fn mint_watt_tokens(&mut self, user_address: &str, fiat_amount: f64) -> Result<String, String> {
        if fiat_amount <= 0.0 {
            return Err("Fiat amount must be positive".to_string());
        }

        // 1:1 peg with fiat
        let watt_amount = fiat_amount;
        
        // Update user balance
        let user_balance = self.user_balances.get_mut(user_address)
            .ok_or("User account not found")?;
        user_balance.watt_balance += watt_amount;

        // Update token supply and reserves
        self.watt_token.total_supply += watt_amount;
        self.watt_token.fiat_reserves += fiat_amount;

        // Record the mint operation
        let record = MintBurnRecord {
            transaction_id: Uuid::new_v4().to_string(),
            operation: MintBurnOperation::Mint,
            amount: watt_amount,
            fiat_amount,
            user_address: user_address.to_string(),
            timestamp: chrono::Utc::now(),
        };

        self.watt_token.mint_burn_history.push(record.clone());
        Ok(record.transaction_id)
    }

    #[allow(dead_code)]
    pub fn burn_watt_tokens(&mut self, user_address: &str, watt_amount: f64) -> Result<String, String> {
        if watt_amount <= 0.0 {
            return Err("WATT amount must be positive".to_string());
        }

        // Check user balance
        let user_balance = self.user_balances.get_mut(user_address)
            .ok_or("User account not found")?;
        
        if user_balance.watt_balance < watt_amount {
            return Err("Insufficient WATT balance".to_string());
        }

        // 1:1 peg with fiat
        let fiat_amount = watt_amount;

        // Update user balance
        user_balance.watt_balance -= watt_amount;

        // Update token supply and reserves
        self.watt_token.total_supply -= watt_amount;
        self.watt_token.fiat_reserves -= fiat_amount;

        // Record the burn operation
        let record = MintBurnRecord {
            transaction_id: Uuid::new_v4().to_string(),
            operation: MintBurnOperation::Burn,
            amount: watt_amount,
            fiat_amount,
            user_address: user_address.to_string(),
            timestamp: chrono::Utc::now(),
        };

        self.watt_token.mint_burn_history.push(record.clone());
        Ok(record.transaction_id)
    }

    #[allow(dead_code)]
    pub fn stake_grid_tokens(&mut self, user_address: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Stake amount must be positive".to_string());
        }

        let user_balance = self.user_balances.get_mut(user_address)
            .ok_or("User account not found")?;

        if user_balance.grid_balance < amount {
            return Err("Insufficient GRID balance".to_string());
        }

        // Update user balances
        user_balance.grid_balance -= amount;
        user_balance.staked_grid += amount;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn unstake_grid_tokens(&mut self, user_address: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Unstake amount must be positive".to_string());
        }

        let user_balance = self.user_balances.get_mut(user_address)
            .ok_or("User account not found")?;

        if user_balance.staked_grid < amount {
            return Err("Insufficient staked GRID balance".to_string());
        }

        // Update user balances
        user_balance.staked_grid -= amount;
        user_balance.grid_balance += amount;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn calculate_staking_rewards(&self, user_address: &str) -> Result<f64, String> {
        let user_balance = self.user_balances.get(user_address)
            .ok_or("User account not found")?;

        let time_elapsed = chrono::Utc::now()
            .signed_duration_since(user_balance.last_reward_claim)
            .num_seconds() as f64;

        let annual_seconds = 365.25 * 24.0 * 3600.0;
        let reward_rate = self.grid_token.staking_rewards_rate / annual_seconds;
        let rewards = user_balance.staked_grid * reward_rate * time_elapsed;

        Ok(rewards)
    }

    #[allow(dead_code)]
    pub fn claim_staking_rewards(&mut self, user_address: &str) -> Result<f64, String> {
        let rewards = self.calculate_staking_rewards(user_address)?;

        let user_balance = self.user_balances.get_mut(user_address)
            .ok_or("User account not found")?;

        user_balance.grid_balance += rewards;
        user_balance.staking_rewards += rewards;
        user_balance.last_reward_claim = chrono::Utc::now();

        Ok(rewards)
    }

    #[allow(dead_code)]
    pub fn create_governance_proposal(
        &mut self,
        proposer: &str,
        title: String,
        description: String,
        voting_period_days: i64,
    ) -> Result<String, String> {
        let user_balance = self.user_balances.get(proposer)
            .ok_or("User account not found")?;

        // Require minimum staked tokens to create proposal
        if user_balance.staked_grid < 1000.0 {
            return Err("Minimum 1000 staked GRID tokens required to create proposal".to_string());
        }

        let proposal = GovernanceProposal {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            proposer: proposer.to_string(),
            votes_for: 0.0,
            votes_against: 0.0,
            voting_deadline: chrono::Utc::now() + chrono::Duration::days(voting_period_days),
            status: ProposalStatus::Active,
        };

        let proposal_id = proposal.id.clone();
        self.grid_token.governance_proposals.push(proposal);
        Ok(proposal_id)
    }

    #[allow(dead_code)]
    pub fn vote_on_proposal(
        &mut self,
        voter: &str,
        proposal_id: &str,
        vote_for: bool,
    ) -> Result<(), String> {
        let user_balance = self.user_balances.get(voter)
            .ok_or("User account not found")?;

        let voting_power = user_balance.staked_grid;
        if voting_power == 0.0 {
            return Err("No staked tokens to vote with".to_string());
        }

        let proposal = self.grid_token.governance_proposals
            .iter_mut()
            .find(|p| p.id == proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Active {
            return Err("Proposal is not active".to_string());
        }

        if chrono::Utc::now() > proposal.voting_deadline {
            proposal.status = ProposalStatus::Expired;
            return Err("Voting deadline has passed".to_string());
        }

        // Record the vote
        if vote_for {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }

        Ok(())
    }

    pub fn get_user_balance(&self, user_address: &str) -> Option<&UserTokenBalance> {
        self.user_balances.get(user_address)
    }

    pub fn transfer_watt_tokens(
        &mut self,
        from: &str,
        to: &str,
        amount: f64,
    ) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Transfer amount must be positive".to_string());
        }

        // Check sender balance
        let sender_balance = self.user_balances.get(from)
            .ok_or("Sender account not found")?
            .watt_balance;

        if sender_balance < amount {
            return Err("Insufficient WATT balance".to_string());
        }

        // Perform transfer
        self.user_balances.get_mut(from).unwrap().watt_balance -= amount;
        
        // Create receiver account if it doesn't exist
        if !self.user_balances.contains_key(to) {
            self.create_user_account(to.to_string())?;
        }
        
        self.user_balances.get_mut(to).unwrap().watt_balance += amount;

        Ok(())
    }
}

impl GridToken {
    pub fn new() -> Self {
        GridToken {
            name: "Grid Token".to_string(),
            symbol: "GRID".to_string(),
            total_supply: 1_000_000_000.0, // 1 billion tokens
            circulating_supply: 0.0,
            staking_rewards_rate: 0.08, // 8% annual yield
            governance_proposals: Vec::new(),
        }
    }
}

impl WattToken {
    pub fn new(fiat_peg: FiatCurrency) -> Self {
        WattToken {
            name: "Watt Token".to_string(),
            symbol: "WATT".to_string(),
            fiat_peg,
            total_supply: 0.0,
            fiat_reserves: 0.0,
            mint_burn_history: Vec::new(),
        }
    }
}

impl Default for TokenSystem {
    fn default() -> Self {
        Self::new()
    }
}

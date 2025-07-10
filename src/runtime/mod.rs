// Runtime module for the Thai Energy Trading System
// This module integrates all components and provides the main runtime

use crate::primitives::*;
use crate::core::*;
use crate::services::*;
use serde::{Deserialize, Serialize};
use serde_json;

/// Main runtime for the Thai Energy Trading System
pub struct Runtime {
    /// Blockchain core
    pub blockchain: Blockchain,
    /// Token service
    pub token_service: TokenService,
    /// Energy trading service
    pub energy_trading_service: EnergyTradingService,
    /// System configuration
    pub config: SystemConfig,
    /// Runtime state
    pub state: RuntimeState,
}

/// Runtime state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime version
    pub version: RuntimeVersion,
    /// Current block number
    pub current_block: BlockNumber,
    /// System events
    pub events: Vec<SystemEvent>,
    /// Runtime statistics
    pub stats: RuntimeStats,
}

/// Runtime version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeVersion {
    /// Specification name
    pub spec_name: String,
    /// Implementation name
    pub impl_name: String,
    /// Version number
    pub spec_version: u32,
    /// Implementation version
    pub impl_version: u32,
    /// API versions
    pub apis: Vec<ApiVersion>,
}

/// API version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVersion {
    /// API name
    pub name: String,
    /// API version
    pub version: u32,
}

/// System event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    /// Event type
    pub event_type: EventType,
    /// Event data
    pub data: serde_json::Value,
    /// Block number
    pub block_number: BlockNumber,
    /// Timestamp
    pub timestamp: Timestamp,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    /// Block finalized
    BlockFinalized,
    /// Transaction executed
    TransactionExecuted,
    /// Token transferred
    TokenTransferred,
    /// Energy trade executed
    EnergyTradeExecuted,
    /// Proposal created
    ProposalCreated,
    /// Vote cast
    VoteCast,
    /// Validator slashed
    ValidatorSlashed,
    /// System error
    SystemError,
}

/// Runtime statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStats {
    /// Total blocks
    pub total_blocks: BlockNumber,
    /// Total transactions
    pub total_transactions: u64,
    /// Total energy trades
    pub total_energy_trades: u64,
    /// Total token transfers
    pub total_token_transfers: u64,
    /// Active proposals
    pub active_proposals: u64,
    /// Runtime uptime
    pub uptime: u64,
}

/// Transaction execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Success flag
    pub success: bool,
    /// Transaction hash
    pub transaction_hash: Hash,
    /// Block number
    pub block_number: BlockNumber,
    /// Gas used
    pub gas_used: u64,
    /// Events emitted
    pub events: Vec<SystemEvent>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Dispatch call types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Call {
    /// Token transfer
    TokenTransfer {
        to: AccountId,
        token: String,
        amount: Balance,
    },
    /// Energy trade
    EnergyTrade {
        order: EnergyOrder,
    },
    /// Governance proposal
    CreateProposal {
        proposal: GovernanceProposal,
    },
    /// Vote on proposal
    Vote {
        proposal_id: String,
        vote: Vote,
    },
    /// Stake tokens
    Stake {
        amount: Balance,
        validator: Option<AccountId>,
    },
    /// Unstake tokens
    Unstake {
        amount: Balance,
    },
}

impl Runtime {
    /// Create new runtime
    pub fn new(config: SystemConfig) -> Self {
        let blockchain = Blockchain::new(config.clone());
        let token_service = TokenService::new();
        let energy_trading_service = EnergyTradingService::new(config.chain_spec.genesis.energy_market.clone());

        let state = RuntimeState {
            version: RuntimeVersion::default(),
            current_block: 0,
            events: Vec::new(),
            stats: RuntimeStats::default(),
        };

        Self {
            blockchain,
            token_service,
            energy_trading_service,
            config,
            state,
        }
    }

    /// Initialize runtime with genesis accounts
    pub fn initialize_genesis(&mut self) -> CoreResult<()> {
        // Initialize genesis accounts
        for (account, balance) in &self.config.chain_spec.genesis.accounts {
            self.token_service.mint(account, "GRID", *balance)?;
            self.token_service.mint(account, "WATT", *balance)?;
        }

        // Initialize validators
        for validator in &self.config.chain_spec.genesis.validators {
            self.token_service.stake(validator, constants::MIN_VALIDATOR_STAKE, None)?;
        }

        // Emit genesis event
        self.emit_event(SystemEvent {
            event_type: EventType::BlockFinalized,
            data: serde_json::json!({"block_number": 0, "genesis": true}),
            block_number: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
        });

        Ok(())
    }

    /// Execute a call
    pub fn execute_call(&mut self, from: AccountId, call: Call) -> CoreResult<ExecutionResult> {
        let transaction_hash = format!("0x{:x}", fastrand::u64(..));
        let mut events = Vec::new();

        let result = match call {
            Call::TokenTransfer { to, token, amount } => {
                self.token_service.transfer(&from, &to, &token, amount)?;
                events.push(SystemEvent {
                    event_type: EventType::TokenTransferred,
                    data: serde_json::json!({
                        "from": from,
                        "to": to,
                        "token": token,
                        "amount": amount
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                self.state.stats.total_token_transfers += 1;
                Ok(()) as CoreResult<()>
            }
            Call::EnergyTrade { order } => {
                let order_id = self.energy_trading_service.place_order(order)?;
                events.push(SystemEvent {
                    event_type: EventType::EnergyTradeExecuted,
                    data: serde_json::json!({
                        "order_id": order_id,
                        "account": from
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                self.state.stats.total_energy_trades += 1;
                Ok(()) as CoreResult<()>
            }
            Call::CreateProposal { proposal } => {
                let proposal_id = self.token_service.create_proposal(
                    &from,
                    proposal.title.clone(),
                    proposal.description.clone(),
                    proposal.voting_period,
                )?;
                events.push(SystemEvent {
                    event_type: EventType::ProposalCreated,
                    data: serde_json::json!({
                        "proposal_id": proposal_id,
                        "proposer": from
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                self.state.stats.active_proposals += 1;
                Ok(()) as CoreResult<()>
            }
            Call::Vote { proposal_id, vote } => {
                // Clone the vote to avoid move issues
                let vote_clone = vote.clone();
                self.token_service.vote_on_proposal(&from, &proposal_id, vote)?;
                events.push(SystemEvent {
                    event_type: EventType::VoteCast,
                    data: serde_json::json!({
                        "voter": from,
                        "proposal_id": proposal_id,
                        "vote": vote_clone
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                Ok(())
            }
            Call::Stake { amount, validator } => {
                self.token_service.stake(&from, amount, validator.as_ref())?;
                events.push(SystemEvent {
                    event_type: EventType::TokenTransferred,
                    data: serde_json::json!({
                        "account": from,
                        "amount": amount,
                        "action": "stake"
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                Ok(())
            }
            Call::Unstake { amount } => {
                self.token_service.unstake(&from, amount)?;
                events.push(SystemEvent {
                    event_type: EventType::TokenTransferred,
                    data: serde_json::json!({
                        "account": from,
                        "amount": amount,
                        "action": "unstake"
                    }),
                    block_number: self.state.current_block,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                });
                Ok(())
            }
        };

        // Emit events
        for event in &events {
            self.emit_event(event.clone());
        }

        // Create execution result
        match result {
            Ok(()) => Ok(ExecutionResult {
                success: true,
                transaction_hash,
                block_number: self.state.current_block,
                gas_used: 1000, // Simplified gas calculation
                events,
                error: None,
            }),
            Err(error) => Ok(ExecutionResult {
                success: false,
                transaction_hash,
                block_number: self.state.current_block,
                gas_used: 500, // Less gas for failed transactions
                events,
                error: Some(error.to_string()),
            }),
        }
    }

    /// Produce new block
    pub fn produce_block(&mut self, author: AccountId) -> CoreResult<Block> {
        let block = self.blockchain.mine_block(author)?;
        
        // Update runtime state
        self.state.current_block = block.header.number;
        self.state.stats.total_blocks = block.header.number;
        self.state.stats.total_transactions += block.body.transactions.len() as u64;

        // Emit block finalized event
        self.emit_event(SystemEvent {
            event_type: EventType::BlockFinalized,
            data: serde_json::json!({
                "block_number": block.header.number,
                "transaction_count": block.body.transactions.len()
            }),
            block_number: block.header.number,
            timestamp: chrono::Utc::now().timestamp() as u64,
        });

        Ok(block)
    }

    /// Emit system event
    fn emit_event(&mut self, event: SystemEvent) {
        self.state.events.push(event);
        
        // Keep only last 1000 events
        if self.state.events.len() > 1000 {
            self.state.events.remove(0);
        }
    }

    /// Get runtime information
    pub fn get_runtime_info(&self) -> RuntimeInfo {
        RuntimeInfo {
            version: self.state.version.clone(),
            current_block: self.state.current_block,
            stats: self.state.stats.clone(),
            chain_spec: self.config.chain_spec.clone(),
        }
    }

    /// Get account information
    pub fn get_account_info(&self, account: &AccountId) -> AccountInfo {
        AccountInfo {
            account: account.clone(),
            grid_balance: self.token_service.get_balance(account, &"GRID".to_string()),
            watt_balance: self.token_service.get_balance(account, &"WATT".to_string()),
            staking_info: Some(self.token_service.get_staking_info(account).into_iter().next().unwrap_or_else(|| StakingInfo {
                staker: account.clone(),
                amount: 0,
                reward_rate: 0.0,
                staked_at: 0,
                lock_period: 0,
                rewards_earned: 0,
            })),
            participant_info: self.energy_trading_service.get_participant(account).cloned(),
        }
    }

    /// Get system status
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus {
            is_healthy: true,
            current_block: self.state.current_block,
            total_accounts: self.blockchain.state.accounts.len() as u64,
            total_tokens: self.token_service.tokens.len() as u64,
            active_trades: self.energy_trading_service.active_trades.len() as u64,
            active_proposals: self.token_service.proposals.len() as u64,
            uptime: self.state.stats.uptime,
        }
    }
}

/// Runtime information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeInfo {
    /// Runtime version
    pub version: RuntimeVersion,
    /// Current block
    pub current_block: BlockNumber,
    /// Runtime statistics
    pub stats: RuntimeStats,
    /// Chain specification
    pub chain_spec: ChainSpec,
}

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account ID
    pub account: AccountId,
    /// GRID token balance
    pub grid_balance: Balance,
    /// WATT token balance
    pub watt_balance: Balance,
    /// Staking information
    pub staking_info: Option<StakingInfo>,
    /// Participant information
    pub participant_info: Option<Participant>,
}

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System health
    pub is_healthy: bool,
    /// Current block
    pub current_block: BlockNumber,
    /// Total accounts
    pub total_accounts: u64,
    /// Total tokens
    pub total_tokens: u64,
    /// Active trades
    pub active_trades: u64,
    /// Active proposals
    pub active_proposals: u64,
    /// System uptime
    pub uptime: u64,
}

/// Default implementations
impl Default for RuntimeVersion {
    fn default() -> Self {
        Self {
            spec_name: "thai-energy-trading".to_string(),
            impl_name: "thai-energy-trading-runtime".to_string(),
            spec_version: 1,
            impl_version: 1,
            apis: vec![
                ApiVersion { name: "Core".to_string(), version: 1 },
                ApiVersion { name: "Metadata".to_string(), version: 1 },
                ApiVersion { name: "BlockBuilder".to_string(), version: 1 },
                ApiVersion { name: "TaggedTransactionQueue".to_string(), version: 1 },
                ApiVersion { name: "OffchainWorkerApi".to_string(), version: 1 },
                ApiVersion { name: "SessionKeys".to_string(), version: 1 },
                ApiVersion { name: "AccountNonceApi".to_string(), version: 1 },
            ],
        }
    }
}

impl Default for RuntimeStats {
    fn default() -> Self {
        Self {
            total_blocks: 0,
            total_transactions: 0,
            total_energy_trades: 0,
            total_token_transfers: 0,
            active_proposals: 0,
            uptime: 0,
        }
    }
}

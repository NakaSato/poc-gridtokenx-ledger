// Core primitive types and constants for the Thai Energy Trading System
// This module defines the fundamental data types used across the system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Account identifier type
pub type AccountId = String;

/// Balance type for token amounts
pub type Balance = u128;

/// Block number type
pub type BlockNumber = u64;

/// Transaction hash type
pub type Hash = String;

/// Timestamp type
pub type Timestamp = u64;

/// Energy amount in kWh
pub type EnergyAmount = f64;

/// Price in Thai Baht (THB)
pub type Price = f64;

/// Core system constants
pub mod constants {
    use super::*;

    /// One kWh equals one token (1:1 ratio)
    pub const KWH_TO_TOKEN_RATIO: f64 = 1.0;

    /// Minimum energy trade amount (kWh)
    pub const MIN_ENERGY_TRADE: EnergyAmount = 0.1;

    /// Maximum energy trade amount (kWh)
    pub const MAX_ENERGY_TRADE: EnergyAmount = 10_000.0;

    /// Grid fee percentage (1%)
    pub const GRID_FEE_PERCENTAGE: f64 = 0.01;

    /// Minimum stake amount for validators
    pub const MIN_VALIDATOR_STAKE: Balance = 10_000;

    /// Block time in seconds
    pub const BLOCK_TIME: u64 = 6;

    /// Maximum transactions per block
    pub const MAX_TRANSACTIONS_PER_BLOCK: u32 = 1000;

    /// Token decimal places
    pub const TOKEN_DECIMALS: u8 = 18;

    /// Default energy price (THB per kWh)
    pub const DEFAULT_ENERGY_PRICE: Price = 4.50;
}

/// Core error types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreError {
    /// Insufficient balance
    InsufficientBalance,
    /// Invalid account
    InvalidAccount,
    /// Invalid amount
    InvalidAmount,
    /// Invalid input
    InvalidInput(String),
    /// Transaction not found
    TransactionNotFound,
    /// Block not found
    BlockNotFound,
    /// Validation failed
    ValidationFailed(String),
    /// System error
    SystemError(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InsufficientBalance => write!(f, "Insufficient balance"),
            CoreError::InvalidAccount => write!(f, "Invalid account"),
            CoreError::InvalidAmount => write!(f, "Invalid amount"),
            CoreError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CoreError::TransactionNotFound => write!(f, "Transaction not found"),
            CoreError::BlockNotFound => write!(f, "Block not found"),
            CoreError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            CoreError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl std::error::Error for CoreError {}

/// Core result type
pub type CoreResult<T> = Result<T, CoreError>;

/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Network identifier
    pub network_id: String,
    /// Chain specification
    pub chain_spec: ChainSpec,
    /// Node configuration
    pub node_config: NodeConfig,
    /// Consensus configuration
    pub consensus_config: ConsensusConfig,
}

/// Chain specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSpec {
    /// Chain name
    pub name: String,
    /// Chain ID
    pub id: String,
    /// Genesis configuration
    pub genesis: GenesisConfig,
    /// Boot nodes
    pub boot_nodes: Vec<String>,
}

/// Genesis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    /// Initial accounts with balances
    pub accounts: HashMap<AccountId, Balance>,
    /// Initial validators
    pub validators: Vec<AccountId>,
    /// Initial energy market configuration
    pub energy_market: EnergyMarketConfig,
}

/// Energy market configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyMarketConfig {
    /// Market opening hours
    pub opening_hours: (u8, u8), // (start_hour, end_hour)
    /// Trading fee percentage
    pub trading_fee: f64,
    /// Minimum order size
    pub min_order_size: EnergyAmount,
    /// Maximum order size
    pub max_order_size: EnergyAmount,
}

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node name
    pub name: String,
    /// Network port
    pub port: u16,
    /// RPC port
    pub rpc_port: u16,
    /// WebSocket port
    pub ws_port: u16,
    /// Data directory
    pub data_dir: String,
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Consensus algorithm
    pub algorithm: ConsensusAlgorithm,
    /// Block time in seconds
    pub block_time: u64,
    /// Finality threshold
    pub finality_threshold: u32,
}

/// Consensus algorithm types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusAlgorithm {
    /// Proof of Authority
    PoA,
    /// Nominated Proof of Stake
    NPoS,
    /// Practical Byzantine Fault Tolerance
    PBFT,
}

/// Default implementations
impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            network_id: "thai-energy-trading".to_string(),
            chain_spec: ChainSpec::default(),
            node_config: NodeConfig::default(),
            consensus_config: ConsensusConfig::default(),
        }
    }
}

impl Default for ChainSpec {
    fn default() -> Self {
        Self {
            name: "Thai Energy Trading Network".to_string(),
            id: "thai-energy-trading".to_string(),
            genesis: GenesisConfig::default(),
            boot_nodes: Vec::new(),
        }
    }
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            accounts: HashMap::new(),
            validators: Vec::new(),
            energy_market: EnergyMarketConfig::default(),
        }
    }
}

impl Default for EnergyMarketConfig {
    fn default() -> Self {
        Self {
            opening_hours: (6, 22), // 6 AM to 10 PM
            trading_fee: constants::GRID_FEE_PERCENTAGE,
            min_order_size: constants::MIN_ENERGY_TRADE,
            max_order_size: constants::MAX_ENERGY_TRADE,
        }
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            name: "thai-energy-node".to_string(),
            port: 30333,
            rpc_port: 9933,
            ws_port: 9944,
            data_dir: "./data".to_string(),
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            algorithm: ConsensusAlgorithm::NPoS,
            block_time: constants::BLOCK_TIME,
            finality_threshold: 12,
        }
    }
}

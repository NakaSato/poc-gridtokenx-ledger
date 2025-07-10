// Utility functions and helpers
// This module contains common utilities used across the system

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cryptographic utilities
pub mod crypto {
    use super::*;

    /// Generate a random account ID
    pub fn generate_account_id() -> AccountId {
        format!("account_{:x}", fastrand::u64(..))
    }

    /// Generate a random hash
    pub fn generate_hash() -> Hash {
        format!("0x{:x}", fastrand::u64(..))
    }

    /// Simple hash function for testing
    pub fn simple_hash(data: &str) -> Hash {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("0x{:x}", hasher.finish())
    }

    /// Verify signature (simplified)
    pub fn verify_signature(message: &str, signature: &str, public_key: &str) -> bool {
        // Simplified signature verification
        // In a real implementation, this would use proper cryptographic verification
        !message.is_empty() && !signature.is_empty() && !public_key.is_empty()
    }

    /// Generate keypair (simplified)
    pub fn generate_keypair() -> (String, String) {
        let private_key = format!("priv_{:x}", fastrand::u64(..));
        let public_key = format!("pub_{:x}", fastrand::u64(..));
        (private_key, public_key)
    }
}

/// Time utilities
pub mod time {
    use super::*;

    /// Get current timestamp
    pub fn current_timestamp() -> Timestamp {
        chrono::Utc::now().timestamp() as u64
    }

    /// Convert timestamp to human readable format
    pub fn timestamp_to_string(timestamp: Timestamp) -> String {
        chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
    }

    /// Add days to timestamp
    pub fn add_days(timestamp: Timestamp, days: u64) -> Timestamp {
        timestamp + (days * 24 * 60 * 60)
    }

    /// Check if timestamp is in the future
    pub fn is_future(timestamp: Timestamp) -> bool {
        timestamp > current_timestamp()
    }

    /// Check if timestamp is in the past
    pub fn is_past(timestamp: Timestamp) -> bool {
        timestamp < current_timestamp()
    }
}

/// Math utilities
pub mod math {
    use super::*;

    /// Calculate percentage
    pub fn calculate_percentage(part: f64, total: f64) -> f64 {
        if total == 0.0 {
            0.0
        } else {
            (part / total) * 100.0
        }
    }

    /// Calculate grid fee
    pub fn calculate_grid_fee(amount: f64, fee_percentage: f64) -> f64 {
        amount * fee_percentage
    }

    /// Convert kWh to tokens (1:1 ratio)
    pub fn kwh_to_tokens(kwh: EnergyAmount) -> Balance {
        (kwh * constants::KWH_TO_TOKEN_RATIO * 10_f64.powi(constants::TOKEN_DECIMALS as i32)) as Balance
    }

    /// Convert tokens to kWh
    pub fn tokens_to_kwh(tokens: Balance) -> EnergyAmount {
        (tokens as f64) / (constants::KWH_TO_TOKEN_RATIO * 10_f64.powi(constants::TOKEN_DECIMALS as i32))
    }

    /// Calculate energy trade settlement
    pub fn calculate_trade_settlement(
        energy_amount: EnergyAmount,
        price_per_kwh: Price,
        grid_fee_percentage: f64,
    ) -> TradeSettlement {
        let total_cost = energy_amount * price_per_kwh;
        let grid_fee = calculate_grid_fee(total_cost, grid_fee_percentage);
        let net_amount = total_cost - grid_fee;

        TradeSettlement {
            energy_amount,
            price_per_kwh,
            total_cost,
            grid_fee,
            net_amount,
        }
    }

    /// Calculate compound interest for staking rewards
    pub fn calculate_staking_rewards(
        principal: Balance,
        annual_rate: f64,
        days: u64,
    ) -> Balance {
        let daily_rate = annual_rate / 365.0;
        let compound_factor = (1.0 + daily_rate).powi(days as i32);
        ((principal as f64) * compound_factor) as Balance - principal
    }
}

/// Trade settlement details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeSettlement {
    /// Energy amount
    pub energy_amount: EnergyAmount,
    /// Price per kWh
    pub price_per_kwh: Price,
    /// Total cost
    pub total_cost: Price,
    /// Grid fee
    pub grid_fee: Price,
    /// Net amount after fees
    pub net_amount: Price,
}

/// Validation utilities
pub mod validation {
    use super::*;

    /// Validate account ID format
    pub fn validate_account_id(account_id: &str) -> bool {
        !account_id.is_empty() && account_id.len() >= 8 && account_id.len() <= 64
    }

    /// Validate energy amount
    pub fn validate_energy_amount(amount: EnergyAmount) -> CoreResult<()> {
        if amount < constants::MIN_ENERGY_TRADE {
            return Err(CoreError::ValidationFailed("Energy amount too small".to_string()));
        }
        if amount > constants::MAX_ENERGY_TRADE {
            return Err(CoreError::ValidationFailed("Energy amount too large".to_string()));
        }
        Ok(())
    }

    /// Validate price
    pub fn validate_price(price: Price) -> CoreResult<()> {
        if price <= 0.0 {
            return Err(CoreError::ValidationFailed("Price must be positive".to_string()));
        }
        if price > 1000.0 {
            return Err(CoreError::ValidationFailed("Price too high".to_string()));
        }
        Ok(())
    }

    /// Validate balance
    pub fn validate_balance(balance: Balance) -> CoreResult<()> {
        if balance == 0 {
            return Err(CoreError::ValidationFailed("Balance cannot be zero".to_string()));
        }
        Ok(())
    }

    /// Validate proposal data
    pub fn validate_proposal(title: &str, description: &str) -> CoreResult<()> {
        if title.is_empty() || title.len() > 100 {
            return Err(CoreError::ValidationFailed("Invalid proposal title".to_string()));
        }
        if description.is_empty() || description.len() > 1000 {
            return Err(CoreError::ValidationFailed("Invalid proposal description".to_string()));
        }
        Ok(())
    }
}

/// Formatting utilities
pub mod format {
    use super::*;

    /// Format balance with decimals
    pub fn format_balance(balance: Balance, decimals: u8) -> String {
        let divisor = 10_u128.pow(decimals as u32);
        let integer_part = balance / divisor;
        let fractional_part = balance % divisor;
        
        if fractional_part == 0 {
            format!("{}", integer_part)
        } else {
            format!("{}.{:0width$}", integer_part, fractional_part, width = decimals as usize)
        }
    }

    /// Format energy amount
    pub fn format_energy_amount(amount: EnergyAmount) -> String {
        format!("{:.2} kWh", amount)
    }

    /// Format price
    pub fn format_price(price: Price) -> String {
        format!("{:.2} THB", price)
    }

    /// Format percentage
    pub fn format_percentage(value: f64) -> String {
        format!("{:.2}%", value)
    }

    /// Format timestamp
    pub fn format_timestamp(timestamp: Timestamp) -> String {
        time::timestamp_to_string(timestamp)
    }

    /// Format hash (shortened)
    pub fn format_hash_short(hash: &str) -> String {
        if hash.len() > 10 {
            format!("{}...{}", &hash[..6], &hash[hash.len()-4..])
        } else {
            hash.to_string()
        }
    }
}

/// Configuration utilities
pub mod config {
    use super::*;

    /// Load system configuration from environment or defaults
    pub fn load_system_config() -> SystemConfig {
        SystemConfig {
            network_id: std::env::var("NETWORK_ID").unwrap_or_else(|_| "thai-energy-trading".to_string()),
            chain_spec: load_chain_spec(),
            node_config: load_node_config(),
            consensus_config: load_consensus_config(),
        }
    }

    /// Load chain specification
    fn load_chain_spec() -> ChainSpec {
        ChainSpec {
            name: std::env::var("CHAIN_NAME").unwrap_or_else(|_| "Thai Energy Trading Network".to_string()),
            id: std::env::var("CHAIN_ID").unwrap_or_else(|_| "thai-energy-trading".to_string()),
            genesis: load_genesis_config(),
            boot_nodes: load_boot_nodes(),
        }
    }

    /// Load genesis configuration
    fn load_genesis_config() -> GenesisConfig {
        GenesisConfig {
            accounts: load_genesis_accounts(),
            validators: load_genesis_validators(),
            energy_market: load_energy_market_config(),
        }
    }

    /// Load genesis accounts
    fn load_genesis_accounts() -> HashMap<AccountId, Balance> {
        let mut accounts = HashMap::new();
        
        // Add some default accounts for testing
        accounts.insert("alice".to_string(), 1_000_000 * 10_u128.pow(constants::TOKEN_DECIMALS as u32));
        accounts.insert("bob".to_string(), 1_000_000 * 10_u128.pow(constants::TOKEN_DECIMALS as u32));
        accounts.insert("charlie".to_string(), 1_000_000 * 10_u128.pow(constants::TOKEN_DECIMALS as u32));
        
        accounts
    }

    /// Load genesis validators
    fn load_genesis_validators() -> Vec<AccountId> {
        vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ]
    }

    /// Load energy market configuration
    fn load_energy_market_config() -> EnergyMarketConfig {
        EnergyMarketConfig {
            opening_hours: (6, 22), // 6 AM to 10 PM
            trading_fee: constants::GRID_FEE_PERCENTAGE,
            min_order_size: constants::MIN_ENERGY_TRADE,
            max_order_size: constants::MAX_ENERGY_TRADE,
        }
    }

    /// Load node configuration
    fn load_node_config() -> NodeConfig {
        NodeConfig {
            name: std::env::var("NODE_NAME").unwrap_or_else(|_| "thai-energy-node".to_string()),
            port: std::env::var("NODE_PORT").unwrap_or_else(|_| "30333".to_string()).parse().unwrap_or(30333),
            rpc_port: std::env::var("RPC_PORT").unwrap_or_else(|_| "9933".to_string()).parse().unwrap_or(9933),
            ws_port: std::env::var("WS_PORT").unwrap_or_else(|_| "9944".to_string()).parse().unwrap_or(9944),
            data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
        }
    }

    /// Load consensus configuration
    fn load_consensus_config() -> ConsensusConfig {
        ConsensusConfig {
            algorithm: match std::env::var("CONSENSUS_ALGORITHM").unwrap_or_else(|_| "NPoS".to_string()).as_str() {
                "PoA" => ConsensusAlgorithm::PoA,
                "PBFT" => ConsensusAlgorithm::PBFT,
                _ => ConsensusAlgorithm::NPoS,
            },
            block_time: std::env::var("BLOCK_TIME").unwrap_or_else(|_| "6".to_string()).parse().unwrap_or(6),
            finality_threshold: std::env::var("FINALITY_THRESHOLD").unwrap_or_else(|_| "12".to_string()).parse().unwrap_or(12),
        }
    }

    /// Load boot nodes
    fn load_boot_nodes() -> Vec<String> {
        std::env::var("BOOT_NODES")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

/// Testing utilities
#[cfg(test)]
pub mod testing {
    use super::*;

    /// Create test account
    pub fn create_test_account(name: &str) -> AccountId {
        format!("test_{}", name)
    }

    /// Create test energy order
    pub fn create_test_energy_order(account: &str, order_type: crate::services::OrderType, amount: EnergyAmount, price: Price) -> crate::services::EnergyOrder {
        crate::services::EnergyOrder {
            id: format!("order_{}", fastrand::u64(..)),
            account: account.to_string(),
            order_type,
            energy_amount: amount,
            price_per_kwh: price,
            total_price: amount * price,
            status: crate::services::OrderStatus::Pending,
            created_at: time::current_timestamp(),
            expires_at: time::add_days(time::current_timestamp(), 1),
            filled_amount: 0.0,
        }
    }

    /// Create test governance proposal
    pub fn create_test_proposal(proposer: &str, title: &str, description: &str) -> crate::services::GovernanceProposal {
        crate::services::GovernanceProposal {
            id: format!("proposal_{}", fastrand::u64(..)),
            title: title.to_string(),
            description: description.to_string(),
            proposer: proposer.to_string(),
            proposal_type: crate::services::ProposalType::ParameterChange,
            voting_period: (time::current_timestamp(), time::add_days(time::current_timestamp(), 7)),
            votes_for: 0,
            votes_against: 0,
            status: crate::services::ProposalStatus::Voting,
            execution_time: None,
        }
    }

    /// Create test system config
    pub fn create_test_config() -> SystemConfig {
        SystemConfig::default()
    }
}

/// Logging utilities
pub mod logging {
    /// Log levels
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }

    /// Log entry
    #[derive(Debug, Clone)]
    pub struct LogEntry {
        pub level: LogLevel,
        pub message: String,
        pub timestamp: u64,
        pub module: String,
    }

    /// Simple logger
    pub struct Logger {
        pub level: LogLevel,
    }

    impl Logger {
        pub fn new(level: LogLevel) -> Self {
            Self { level }
        }

        pub fn log(&self, level: LogLevel, module: &str, message: &str) {
            if self.should_log(level) {
                let entry = LogEntry {
                    level,
                    message: message.to_string(),
                    timestamp: crate::utils::time::current_timestamp(),
                    module: module.to_string(),
                };
                self.print_log(&entry);
            }
        }

        fn should_log(&self, level: LogLevel) -> bool {
            match (self.level, level) {
                (LogLevel::Error, LogLevel::Error) => true,
                (LogLevel::Warn, LogLevel::Error | LogLevel::Warn) => true,
                (LogLevel::Info, LogLevel::Error | LogLevel::Warn | LogLevel::Info) => true,
                (LogLevel::Debug, LogLevel::Error | LogLevel::Warn | LogLevel::Info | LogLevel::Debug) => true,
                (LogLevel::Trace, _) => true,
                _ => false,
            }
        }

        fn print_log(&self, entry: &LogEntry) {
            let level_str = match entry.level {
                LogLevel::Error => "ERROR",
                LogLevel::Warn => "WARN",
                LogLevel::Info => "INFO",
                LogLevel::Debug => "DEBUG",
                LogLevel::Trace => "TRACE",
            };
            
            println!(
                "[{}] {} [{}] {}",
                crate::utils::time::timestamp_to_string(entry.timestamp),
                level_str,
                entry.module,
                entry.message
            );
        }
    }
}

// Re-export commonly used utilities
pub use crypto::{generate_account_id, generate_hash, simple_hash};
pub use time::{current_timestamp, timestamp_to_string};
pub use math::{calculate_percentage, calculate_grid_fee, kwh_to_tokens, tokens_to_kwh};
pub use validation::{validate_account_id, validate_energy_amount, validate_price};
pub use format::{format_balance, format_energy_amount, format_price, format_percentage};
pub use config::load_system_config;

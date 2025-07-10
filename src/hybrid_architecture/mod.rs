// Hybrid Architecture Module for Thailand Energy Trading System
// This module implements the three-part hybrid blockchain architecture:
// 1. Public Chain (Governance & Investment)
// 2. Consortium Chain (Operations)
// 3. Oracle Gateway (Interoperability)

pub mod public_chain;
pub mod consortium_chain;
pub mod oracle_gateway;
pub mod integration;
pub mod compliance;

pub use public_chain::PublicChain;
pub use consortium_chain::ConsortiumChain;
pub use oracle_gateway::OracleGateway;
pub use integration::HybridArchitecture;
pub use compliance::ComplianceManager;

use serde::{Deserialize, Serialize};
use crate::token_system::TokenSystem;
use crate::energy_trading::EnergyMarket;

/// Main hybrid architecture coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSystem {
    pub architecture: HybridArchitecture,
    pub compliance: ComplianceManager,
    pub configuration: HybridConfig,
    pub token_system: TokenSystem,
    pub energy_market: EnergyMarket,
}

impl HybridSystem {
    /// Initialize a new hybrid system
    pub fn new(config: HybridConfig) -> Self {
        Self {
            architecture: HybridArchitecture::new(),
            compliance: ComplianceManager::new(),
            configuration: config,
            token_system: TokenSystem::new(),
            energy_market: EnergyMarket::new(),
        }
    }

    /// Process a cross-chain transaction
    pub fn process_cross_chain_transaction(&mut self, transaction: CrossChainTransaction) -> Result<String, String> {
        // Validate compliance
        if let Ok(energy_data) = serde_json::from_value::<EnergyTransactionData>(transaction.data.clone()) {
            self.compliance.validate_energy_transaction(&energy_data)?;
        }
        
        // Route to appropriate chain
        match transaction.target_chain {
            ChainType::Public => self.architecture.public_chain.process_transaction(transaction),
            ChainType::Consortium => self.architecture.consortium_chain.process_transaction(transaction),
            ChainType::Oracle => self.architecture.oracle_gateway.process_transaction(transaction),
        }
    }

    /// Get system status
    pub fn get_system_status(&self) -> HybridSystemStatus {
        HybridSystemStatus {
            public_chain_status: format!("{:?}", self.architecture.public_chain.get_status()),
            consortium_chain_status: format!("{:?}", self.architecture.consortium_chain.get_status()),
            oracle_gateway_status: format!("{:?}", self.architecture.oracle_gateway.get_status()),
            compliance_status: format!("{:?}", self.compliance.get_status()),
        }
    }
}

/// Cross-chain transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainTransaction {
    pub id: String,
    pub source_chain: ChainType,
    pub target_chain: ChainType,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChainType {
    Public,
    Consortium,
    Oracle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    EnergyTrade,
    GovernanceVote,
    TokenTransfer,
    OracleUpdate,
    ComplianceReport,
}

/// System status structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSystemStatus {
    pub public_chain_status: String,
    pub consortium_chain_status: String,
    pub oracle_gateway_status: String,
    pub compliance_status: String,
}

/// Configuration for the hybrid architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridConfig {
    pub public_chain_config: PublicChainConfig,
    pub consortium_chain_config: ConsortiumChainConfig,
    pub oracle_config: OracleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicChainConfig {
    pub consensus: String,
    pub block_time: u64,
    pub validators: usize,
    pub public_participation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsortiumChainConfig {
    pub consensus: String,
    pub block_time: u64,
    pub validators: usize,
    pub permission_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    pub data_sources: Vec<String>,
    pub update_frequency: u64,
    pub fault_tolerance: f64,
}

impl HybridSystem {
    /// Initialize the hybrid system with Thai regulatory compliance
    pub fn initialize_thailand_compliance(&mut self) -> Result<(), String> {
        // Initialize SEC compliance
        self.compliance.initialize_sec_compliance()?;
        
        // Initialize ERC compliance
        self.compliance.initialize_erc_compliance()?;
        
        // Initialize PDPA compliance
        self.compliance.initialize_pdpa_compliance()?;
        
        Ok(())
    }

    /// Process a complete energy transaction across all three layers
    pub fn process_energy_transaction(&mut self, transaction_data: EnergyTransactionData) -> Result<String, String> {
        // 1. Validate compliance
        self.compliance.validate_energy_transaction(&transaction_data)?;
        
        // 2. Convert to EnergyTrade format
        let energy_trade = crate::energy_trading::EnergyTrade {
            trade_id: transaction_data.transaction_id.clone(),
            buyer: transaction_data.buyer,
            seller: transaction_data.seller,
            energy_amount: transaction_data.energy_amount,
            price_per_kwh: transaction_data.price_per_kwh,
            total_cost: transaction_data.energy_amount * transaction_data.price_per_kwh,
            grid_fee: 0.0, // Calculate based on system rules
            timestamp: transaction_data.timestamp,
        };
        
        // 3. Process through hybrid architecture
        let transaction_id = self.architecture.process_energy_trade(energy_trade)?;
        
        // 4. Log for audit
        self.compliance.log_transaction(&transaction_id)?;
        
        Ok(transaction_id)
    }
}

/// Energy transaction data structure for hybrid processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTransactionData {
    pub transaction_id: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub buyer: String,
    pub seller: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridIntegrationData {
    pub grid_connection_point: String,
    pub grid_load_factor: f64,
    pub wheeling_charges: f64,
    pub congestion_factor: f64,
}

/// System status across all three layers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub public_chain_status: String,
    pub consortium_chain_status: String,
    pub oracle_gateway_status: String,
    pub compliance_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStatus {
    pub is_operational: bool,
    pub current_block_height: u64,
    pub validator_count: usize,
    pub transaction_throughput: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub is_operational: bool,
    pub active_data_feeds: usize,
    pub data_freshness: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub sec_compliant: bool,
    pub erc_compliant: bool,
    pub pdpa_compliant: bool,
    pub last_audit: chrono::DateTime<chrono::Utc>,
}

impl Default for HybridSystem {
    fn default() -> Self {
        Self::new(HybridConfig::default())
    }
}

impl Default for HybridConfig {
    fn default() -> Self {
        Self {
            public_chain_config: PublicChainConfig::default(),
            consortium_chain_config: ConsortiumChainConfig::default(),
            oracle_config: OracleConfig::default(),
        }
    }
}

impl Default for PublicChainConfig {
    fn default() -> Self {
        Self {
            consensus: "nominated-pos".to_string(),
            block_time: 6,
            validators: 100,
            public_participation: true,
        }
    }
}

impl Default for ConsortiumChainConfig {
    fn default() -> Self {
        Self {
            consensus: "pbft".to_string(),
            block_time: 1,
            validators: 21,
            permission_required: true,
        }
    }
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            data_sources: vec!["weather".to_string(), "grid".to_string(), "prices".to_string()],
            update_frequency: 1,
            fault_tolerance: 0.67,
        }
    }
}

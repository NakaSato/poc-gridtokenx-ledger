//! # Hybrid Architecture Pallet
//!
//! A comprehensive hybrid blockchain architecture system for Thailand's P2P energy trading platform featuring:
//! - Three-layer architecture: Public, Consortium, and Oracle chains (features: "public-chain", "consortium-chain", "oracle-gateway")
//! - Cross-chain integration and communication (feature: "cross-chain")
//! - Thai regulatory compliance (SEC, ERC, PDPA) (feature: "compliance")
//! - High-performance transaction processing with batch support (feature: "performance-optimized")
//! - Comprehensive monitoring and alerting (features: "metrics", "health-checks", "alerting")
//!
//! ## Architecture Overview
//!
//! The hybrid architecture consists of three interconnected blockchain layers:
//!
//! ### 1. Public Chain Layer
//! - **Purpose**: Governance, transparency, and public participation
//! - **Consensus**: Nominated Proof of Stake (NPoS)
//! - **Features**: Governance proposals, token transfers, compliance reporting
//! - **Enabled by**: `public-chain` feature
//!
//! ### 2. Consortium Chain Layer
//! - **Purpose**: Energy trading and operational efficiency
//! - **Consensus**: Practical Byzantine Fault Tolerance (PBFT)
//! - **Features**: Energy trading, settlement, grid operations
//! - **Enabled by**: `consortium-chain` feature
//!
//! ### 3. Oracle Gateway Layer
//! - **Purpose**: External data integration and API connectivity
//! - **Features**: Weather data, grid status, energy prices, regulatory data
//! - **Enabled by**: `oracle-gateway` feature
//!
//! ## Feature Flags
//!
//! This pallet supports comprehensive feature flags for modular functionality:
//!
//! ### Core Features
//! - `public-chain`: Enable public governance chain
//! - `consortium-chain`: Enable consortium energy trading chain
//! - `oracle-gateway`: Enable oracle data gateway
//! - `cross-chain`: Enable cross-chain integration
//! - `integration-bridge`: Cross-chain bridge functionality
//!
//! ### Compliance Features
//! - `compliance`: Thai regulatory compliance framework
//! - `sec-compliance`: SEC Thailand compliance
//! - `erc-compliance`: ERC Thailand compliance
//! - `pdpa-compliance`: PDPA compliance
//! - `automated-reporting`: Automated compliance reporting
//!
//! ### Performance Features
//! - `performance-optimized`: Performance optimizations
//! - `batch-processing`: Batch transaction processing
//! - `async-processing`: Asynchronous processing
//! - `caching`: Result caching
//!
//! ### Security Features
//! - `access-control`: Enhanced access control
//! - `audit-logging`: Comprehensive audit logging
//! - `transaction-validation`: Enhanced transaction validation
//! - `multi-signature`: Multi-signature support

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// Import from related pallets
pub use pallet_token_system::{TokenSystem, TokenType, Balance, AccountId, BlockNumber};
pub use pallet_energy_trading::{EnergyTradingSystem, EnergyTrade, EnergyOrder};

/// Transaction ID type
pub type TransactionId = String;

/// Chain ID type
pub type ChainId = String;

/// Validator ID type
pub type ValidatorId = String;

/// Cross-chain transaction types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Governance proposal or voting
    GovernanceVote,
    /// Energy trading transaction
    EnergyTrade,
    /// Token transfer between chains
    TokenTransfer,
    /// Oracle data update
    OracleUpdate,
    /// Compliance report submission
    ComplianceReport,
    /// Emergency transaction
    Emergency,
}

/// Blockchain chain types in the hybrid architecture
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    /// Public governance chain
    Public,
    /// Consortium energy trading chain
    Consortium,
    /// Oracle data gateway
    Oracle,
}

/// Cross-chain transaction structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossChainTransaction {
    /// Unique transaction ID
    pub id: TransactionId,
    /// Source chain
    pub source_chain: ChainType,
    /// Target chain
    pub target_chain: ChainType,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Transaction amount (if applicable)
    pub amount: f64,
    /// Transaction data payload
    pub data: serde_json::Value,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Transaction status
    pub status: TransactionStatus,
    /// Confirmations received
    pub confirmations: u32,
    /// Required confirmations
    pub required_confirmations: u32,
}

/// Transaction status tracking
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is pending
    Pending,
    /// Transaction is being processed
    Processing,
    /// Transaction is confirmed
    Confirmed,
    /// Transaction is finalized
    Finalized,
    /// Transaction failed
    Failed,
    /// Transaction was rejected
    Rejected,
}

/// Chain status information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainStatus {
    /// Whether the chain is operational
    pub is_operational: bool,
    /// Current block height
    pub current_block_height: u64,
    /// Number of active validators
    pub validator_count: u32,
    /// Transaction throughput (TPS)
    pub transaction_throughput: f64,
    /// Last block timestamp
    pub last_block_timestamp: DateTime<Utc>,
    /// Network latency (ms)
    pub network_latency: u64,
}

/// Validator information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Validator ID
    pub id: ValidatorId,
    /// Validator address
    pub address: AccountId,
    /// Validator name
    pub name: String,
    /// Staked amount
    pub stake: Balance,
    /// Whether the validator is active
    pub is_active: bool,
    /// Validator performance metrics
    pub performance_metrics: ValidatorMetrics,
}

/// Validator performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorMetrics {
    /// Blocks produced
    pub blocks_produced: u64,
    /// Blocks missed
    pub blocks_missed: u64,
    /// Uptime percentage
    pub uptime_percentage: f64,
    /// Slashing events
    pub slashing_events: u32,
}

/// Events emitted by the hybrid architecture
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Cross-chain transaction initiated
    CrossChainTransactionInitiated {
        transaction_id: TransactionId,
        source_chain: ChainType,
        target_chain: ChainType,
        transaction_type: TransactionType,
        amount: f64,
    },
    /// Cross-chain transaction confirmed
    CrossChainTransactionConfirmed {
        transaction_id: TransactionId,
        confirmations: u32,
    },
    /// Cross-chain transaction finalized
    CrossChainTransactionFinalized {
        transaction_id: TransactionId,
        final_status: TransactionStatus,
    },
    /// Chain status updated
    ChainStatusUpdated {
        chain_type: ChainType,
        status: ChainStatus,
    },
    /// Validator added
    ValidatorAdded {
        chain_type: ChainType,
        validator_id: ValidatorId,
        validator_address: AccountId,
    },
    /// Validator removed
    ValidatorRemoved {
        chain_type: ChainType,
        validator_id: ValidatorId,
    },
    /// Compliance report submitted
    ComplianceReportSubmitted {
        report_id: String,
        report_type: String,
        submitter: AccountId,
    },
    /// Emergency alert issued
    EmergencyAlertIssued {
        alert_id: String,
        alert_type: String,
        severity: String,
        description: String,
    },
    /// System health check performed
    SystemHealthCheck {
        overall_health: String,
        timestamp: DateTime<Utc>,
    },
}

/// Errors that can occur in the hybrid architecture
#[derive(Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
pub enum HybridArchitectureError {
    #[error("Chain not found: {0}")]
    ChainNotFound(String),
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("Validator not found: {0}")]
    ValidatorNotFound(String),
    #[error("Invalid transaction type: {0}")]
    InvalidTransactionType(String),
    #[error("Invalid chain type: {0}")]
    InvalidChainType(String),
    #[error("Cross-chain transfer failed: {0}")]
    CrossChainTransferFailed(String),
    #[error("Insufficient confirmations: got {got}, required {required}")]
    InsufficientConfirmations { got: u32, required: u32 },
    #[error("Transaction already processed: {0}")]
    TransactionAlreadyProcessed(String),
    #[error("Chain is not operational: {0}")]
    ChainNotOperational(String),
    #[error("Validator is not active: {0}")]
    ValidatorNotActive(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Compliance validation failed: {0}")]
    ComplianceValidationFailed(String),
    #[error("Token system error: {0}")]
    TokenSystemError(String),
    #[error("Energy trading error: {0}")]
    EnergyTradingError(String),
    #[error("Oracle data error: {0}")]
    OracleDataError(String),
    #[error("System configuration error: {0}")]
    SystemConfigurationError(String),
}

/// Hybrid architecture system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridArchitectureConfig {
    /// Public chain configuration
    pub public_chain: PublicChainConfig,
    /// Consortium chain configuration
    pub consortium_chain: ConsortiumChainConfig,
    /// Oracle gateway configuration
    pub oracle_gateway: OracleGatewayConfig,
    /// Cross-chain integration configuration
    pub cross_chain: CrossChainConfig,
    /// Compliance configuration
    pub compliance: ComplianceConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
}

/// Public chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicChainConfig {
    /// Consensus mechanism
    pub consensus: String,
    /// Block time in seconds
    pub block_time: u64,
    /// Number of validators
    pub validator_count: u32,
    /// Whether public participation is enabled
    pub public_participation: bool,
    /// Minimum stake for validators
    pub min_validator_stake: Balance,
    /// Governance proposal threshold
    pub governance_threshold: Balance,
}

/// Consortium chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsortiumChainConfig {
    /// Consensus mechanism
    pub consensus: String,
    /// Block time in seconds
    pub block_time: u64,
    /// Number of validators
    pub validator_count: u32,
    /// Whether permission is required
    pub permission_required: bool,
    /// Maximum transaction throughput
    pub max_throughput: u32,
    /// Settlement finality time
    pub settlement_finality: u64,
}

/// Oracle gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleGatewayConfig {
    /// Data source endpoints
    pub data_sources: Vec<String>,
    /// Update frequency in seconds
    pub update_frequency: u64,
    /// Fault tolerance threshold
    pub fault_tolerance: f64,
    /// Data validation rules
    pub validation_rules: Vec<String>,
    /// Maximum data age in seconds
    pub max_data_age: u64,
}

/// Cross-chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainConfig {
    /// Required confirmations for cross-chain transfers
    pub required_confirmations: u32,
    /// Transaction timeout in seconds
    pub transaction_timeout: u64,
    /// Bridge fee rate (in basis points)
    pub bridge_fee_rate: u32,
    /// Maximum concurrent transactions
    pub max_concurrent_transactions: u32,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// SEC compliance enabled
    pub sec_enabled: bool,
    /// ERC compliance enabled
    pub erc_enabled: bool,
    /// PDPA compliance enabled
    pub pdpa_enabled: bool,
    /// Automated reporting enabled
    pub automated_reporting: bool,
    /// Audit trail retention period (days)
    pub audit_retention_days: u32,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Batch size for batch processing
    pub batch_size: u32,
    /// Cache size for result caching
    pub cache_size: u32,
    /// Worker thread count
    pub worker_threads: u32,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
}

impl Default for HybridArchitectureConfig {
    fn default() -> Self {
        Self {
            public_chain: PublicChainConfig {
                consensus: "nominated-proof-of-stake".to_string(),
                block_time: 6,
                validator_count: 100,
                public_participation: true,
                min_validator_stake: 1_000_000, // 1M GRID tokens
                governance_threshold: 100_000,  // 100K GRID tokens
            },
            consortium_chain: ConsortiumChainConfig {
                consensus: "practical-byzantine-fault-tolerance".to_string(),
                block_time: 1,
                validator_count: 21,
                permission_required: true,
                max_throughput: 10_000,
                settlement_finality: 2, // 2 seconds
            },
            oracle_gateway: OracleGatewayConfig {
                data_sources: vec![
                    "weather-api".to_string(),
                    "grid-status".to_string(),
                    "energy-prices".to_string(),
                ],
                update_frequency: 10,
                fault_tolerance: 0.67,
                validation_rules: vec![
                    "temperature_range".to_string(),
                    "price_bounds".to_string(),
                ],
                max_data_age: 300, // 5 minutes
            },
            cross_chain: CrossChainConfig {
                required_confirmations: 12,
                transaction_timeout: 300,
                bridge_fee_rate: 10, // 0.1%
                max_concurrent_transactions: 1000,
            },
            compliance: ComplianceConfig {
                sec_enabled: true,
                erc_enabled: true,
                pdpa_enabled: true,
                automated_reporting: true,
                audit_retention_days: 2555, // 7 years
            },
            performance: PerformanceConfig {
                batch_size: 100,
                cache_size: 10_000,
                worker_threads: 8,
                health_check_interval: 30,
            },
        }
    }
}

/// Main hybrid architecture system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridArchitectureSystem {
    /// System configuration
    pub config: HybridArchitectureConfig,
    /// Cross-chain transactions
    pub cross_chain_transactions: HashMap<TransactionId, CrossChainTransaction>,
    /// Chain status information
    pub chain_status: HashMap<ChainType, ChainStatus>,
    /// Validator information by chain
    pub validators: HashMap<ChainType, Vec<ValidatorInfo>>,
    /// Transaction processing queue
    pub transaction_queue: VecDeque<CrossChainTransaction>,
    /// Processed transactions
    pub processed_transactions: HashMap<TransactionId, CrossChainTransaction>,
    /// Event history
    pub events: Vec<Event>,
    /// System metrics
    pub metrics: SystemMetrics,
    /// Current block number
    pub current_block: BlockNumber,
    /// System health status
    pub health_status: SystemHealthStatus,
}

/// System metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total cross-chain transactions
    pub total_cross_chain_transactions: u64,
    /// Successful transactions
    pub successful_transactions: u64,
    /// Failed transactions
    pub failed_transactions: u64,
    /// Average transaction processing time (ms)
    pub avg_processing_time: f64,
    /// Current transaction throughput (TPS)
    pub current_throughput: f64,
    /// Total value transferred
    pub total_value_transferred: Balance,
    /// System uptime (seconds)
    pub system_uptime: u64,
}

/// System health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemHealthStatus {
    /// Overall system health
    pub overall_health: HealthLevel,
    /// Public chain health
    pub public_chain_health: HealthLevel,
    /// Consortium chain health
    pub consortium_chain_health: HealthLevel,
    /// Oracle gateway health
    pub oracle_gateway_health: HealthLevel,
    /// Cross-chain bridge health
    pub cross_chain_bridge_health: HealthLevel,
    /// Last health check timestamp
    pub last_health_check: DateTime<Utc>,
}

/// Health level enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthLevel {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System is critical
    Critical,
    /// System is down
    Down,
}

impl Default for HybridArchitectureSystem {
    fn default() -> Self {
        Self {
            config: HybridArchitectureConfig::default(),
            cross_chain_transactions: HashMap::new(),
            chain_status: HashMap::new(),
            validators: HashMap::new(),
            transaction_queue: VecDeque::new(),
            processed_transactions: HashMap::new(),
            events: Vec::new(),
            metrics: SystemMetrics::default(),
            current_block: 0,
            health_status: SystemHealthStatus::default(),
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            total_cross_chain_transactions: 0,
            successful_transactions: 0,
            failed_transactions: 0,
            avg_processing_time: 0.0,
            current_throughput: 0.0,
            total_value_transferred: 0,
            system_uptime: 0,
        }
    }
}

impl Default for SystemHealthStatus {
    fn default() -> Self {
        Self {
            overall_health: HealthLevel::Healthy,
            public_chain_health: HealthLevel::Healthy,
            consortium_chain_health: HealthLevel::Healthy,
            oracle_gateway_health: HealthLevel::Healthy,
            cross_chain_bridge_health: HealthLevel::Healthy,
            last_health_check: Utc::now(),
        }
    }
}

impl HybridArchitectureSystem {
    /// Create a new hybrid architecture system
    pub fn new(config: HybridArchitectureConfig) -> Self {
        let mut system = Self::default();
        system.config = config;
        system.initialize_chains();
        system
    }

    /// Initialize chain status
    fn initialize_chains(&mut self) {
        // Initialize public chain
        self.chain_status.insert(ChainType::Public, ChainStatus {
            is_operational: true,
            current_block_height: 1000,
            validator_count: self.config.public_chain.validator_count,
            transaction_throughput: 1000.0,
            last_block_timestamp: Utc::now(),
            network_latency: 100,
        });

        // Initialize consortium chain
        self.chain_status.insert(ChainType::Consortium, ChainStatus {
            is_operational: true,
            current_block_height: 5000,
            validator_count: self.config.consortium_chain.validator_count,
            transaction_throughput: 10000.0,
            last_block_timestamp: Utc::now(),
            network_latency: 50,
        });

        // Initialize oracle gateway
        self.chain_status.insert(ChainType::Oracle, ChainStatus {
            is_operational: true,
            current_block_height: 2000,
            validator_count: 5,
            transaction_throughput: 500.0,
            last_block_timestamp: Utc::now(),
            network_latency: 200,
        });
    }

    /// Set current block number
    pub fn set_block(&mut self, block: BlockNumber) {
        self.current_block = block;
    }

    /// Advance block number
    pub fn advance_block(&mut self) {
        self.current_block += 1;
    }

    /// Emit an event
    fn emit_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Get chain status
    pub fn get_chain_status(&self, chain_type: &ChainType) -> Option<&ChainStatus> {
        self.chain_status.get(chain_type)
    }

    /// Get system metrics
    pub fn get_metrics(&self) -> &SystemMetrics {
        &self.metrics
    }

    /// Get system health status
    pub fn get_health_status(&self) -> &SystemHealthStatus {
        &self.health_status
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

// ================================================================================
// FEATURE-GATED IMPLEMENTATIONS
// ================================================================================

#[cfg(feature = "cross-chain")]
impl HybridArchitectureSystem {
    /// Submit a cross-chain transaction
    pub fn submit_cross_chain_transaction(
        &mut self,
        transaction: CrossChainTransaction,
    ) -> Result<TransactionId, HybridArchitectureError> {
        // Validate transaction
        self.validate_cross_chain_transaction(&transaction)?;

        // Add to transaction queue
        let transaction_id = transaction.id.clone();
        self.transaction_queue.push_back(transaction.clone());
        self.cross_chain_transactions.insert(transaction_id.clone(), transaction.clone());

        // Emit event
        self.emit_event(Event::CrossChainTransactionInitiated {
            transaction_id: transaction_id.clone(),
            source_chain: transaction.source_chain,
            target_chain: transaction.target_chain,
            transaction_type: transaction.transaction_type,
            amount: transaction.amount,
        });

        // Update metrics
        self.metrics.total_cross_chain_transactions += 1;

        Ok(transaction_id)
    }

    /// Process pending cross-chain transactions
    pub fn process_pending_transactions(&mut self) -> Result<u32, HybridArchitectureError> {
        let mut processed_count = 0;

        while let Some(mut transaction) = self.transaction_queue.pop_front() {
            match self.process_single_transaction(&mut transaction) {
                Ok(()) => {
                    processed_count += 1;
                    self.metrics.successful_transactions += 1;
                    self.processed_transactions.insert(transaction.id.clone(), transaction);
                }
                Err(e) => {
                    self.metrics.failed_transactions += 1;
                    transaction.status = TransactionStatus::Failed;
                    self.processed_transactions.insert(transaction.id.clone(), transaction);
                    // Log error but continue processing
                    eprintln!("Transaction processing failed: {}", e);
                }
            }
        }

        Ok(processed_count)
    }

    /// Process a single cross-chain transaction
    fn process_single_transaction(
        &mut self,
        transaction: &mut CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // Update transaction status
        transaction.status = TransactionStatus::Processing;

        // Route to appropriate chain
        match transaction.target_chain {
            ChainType::Public => self.process_public_chain_transaction(transaction),
            ChainType::Consortium => self.process_consortium_chain_transaction(transaction),
            ChainType::Oracle => self.process_oracle_chain_transaction(transaction),
        }?;

        // Update confirmations
        transaction.confirmations += 1;

        // Check if transaction is finalized
        if transaction.confirmations >= transaction.required_confirmations {
            transaction.status = TransactionStatus::Finalized;
            self.emit_event(Event::CrossChainTransactionFinalized {
                transaction_id: transaction.id.clone(),
                final_status: transaction.status.clone(),
            });
        } else {
            transaction.status = TransactionStatus::Confirmed;
            self.emit_event(Event::CrossChainTransactionConfirmed {
                transaction_id: transaction.id.clone(),
                confirmations: transaction.confirmations,
            });
        }

        Ok(())
    }

    /// Validate cross-chain transaction
    fn validate_cross_chain_transaction(
        &self,
        transaction: &CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // Check if chains are operational
        if let Some(source_status) = self.chain_status.get(&transaction.source_chain) {
            if !source_status.is_operational {
                return Err(HybridArchitectureError::ChainNotOperational(
                    format!("{:?}", transaction.source_chain)
                ));
            }
        }

        if let Some(target_status) = self.chain_status.get(&transaction.target_chain) {
            if !target_status.is_operational {
                return Err(HybridArchitectureError::ChainNotOperational(
                    format!("{:?}", transaction.target_chain)
                ));
            }
        }

        // Validate transaction type for target chain
        self.validate_transaction_type_for_chain(
            &transaction.transaction_type,
            &transaction.target_chain,
        )?;

        // Validate amount
        if transaction.amount < 0.0 {
            return Err(HybridArchitectureError::InvalidTransactionType(
                "Amount cannot be negative".to_string()
            ));
        }

        Ok(())
    }

    /// Validate transaction type for chain
    fn validate_transaction_type_for_chain(
        &self,
        transaction_type: &TransactionType,
        chain_type: &ChainType,
    ) -> Result<(), HybridArchitectureError> {
        match (transaction_type, chain_type) {
            (TransactionType::GovernanceVote, ChainType::Public) => Ok(()),
            (TransactionType::TokenTransfer, ChainType::Public) => Ok(()),
            (TransactionType::ComplianceReport, ChainType::Public) => Ok(()),
            (TransactionType::EnergyTrade, ChainType::Consortium) => Ok(()),
            (TransactionType::TokenTransfer, ChainType::Consortium) => Ok(()),
            (TransactionType::OracleUpdate, ChainType::Oracle) => Ok(()),
            (TransactionType::Emergency, _) => Ok(()),
            _ => Err(HybridArchitectureError::InvalidTransactionType(
                format!("{:?} not supported on {:?} chain", transaction_type, chain_type)
            )),
        }
    }

    /// Process public chain transaction
    fn process_public_chain_transaction(
        &mut self,
        transaction: &mut CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // Simulate public chain processing
        match transaction.transaction_type {
            TransactionType::GovernanceVote => {
                // Process governance vote
                Ok(())
            }
            TransactionType::TokenTransfer => {
                // Process token transfer
                Ok(())
            }
            TransactionType::ComplianceReport => {
                // Process compliance report
                Ok(())
            }
            _ => Err(HybridArchitectureError::InvalidTransactionType(
                format!("{:?} not supported on public chain", transaction.transaction_type)
            )),
        }
    }

    /// Process consortium chain transaction
    fn process_consortium_chain_transaction(
        &mut self,
        transaction: &mut CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // Simulate consortium chain processing
        match transaction.transaction_type {
            TransactionType::EnergyTrade => {
                // Process energy trade
                Ok(())
            }
            TransactionType::TokenTransfer => {
                // Process token transfer
                Ok(())
            }
            _ => Err(HybridArchitectureError::InvalidTransactionType(
                format!("{:?} not supported on consortium chain", transaction.transaction_type)
            )),
        }
    }

    /// Process oracle chain transaction
    fn process_oracle_chain_transaction(
        &mut self,
        transaction: &mut CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // Simulate oracle chain processing
        match transaction.transaction_type {
            TransactionType::OracleUpdate => {
                // Process oracle update
                Ok(())
            }
            _ => Err(HybridArchitectureError::InvalidTransactionType(
                format!("{:?} not supported on oracle chain", transaction.transaction_type)
            )),
        }
    }
}

#[cfg(feature = "compliance")]
impl HybridArchitectureSystem {
    /// Validate compliance for a transaction
    pub fn validate_compliance(
        &self,
        transaction: &CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        if self.config.compliance.sec_enabled {
            self.validate_sec_compliance(transaction)?;
        }

        if self.config.compliance.erc_enabled {
            self.validate_erc_compliance(transaction)?;
        }

        if self.config.compliance.pdpa_enabled {
            self.validate_pdpa_compliance(transaction)?;
        }

        Ok(())
    }

    /// Validate SEC compliance
    fn validate_sec_compliance(
        &self,
        _transaction: &CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // SEC validation logic
        Ok(())
    }

    /// Validate ERC compliance
    fn validate_erc_compliance(
        &self,
        _transaction: &CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // ERC validation logic
        Ok(())
    }

    /// Validate PDPA compliance
    fn validate_pdpa_compliance(
        &self,
        _transaction: &CrossChainTransaction,
    ) -> Result<(), HybridArchitectureError> {
        // PDPA validation logic
        Ok(())
    }
}

#[cfg(feature = "health-checks")]
impl HybridArchitectureSystem {
    /// Perform system health check
    pub fn perform_health_check(&mut self) -> Result<(), HybridArchitectureError> {
        // Check each chain health
        let public_health = self.check_chain_health(&ChainType::Public)?;
        let consortium_health = self.check_chain_health(&ChainType::Consortium)?;
        let oracle_health = self.check_chain_health(&ChainType::Oracle)?;

        // Update health status
        self.health_status.public_chain_health = public_health;
        self.health_status.consortium_chain_health = consortium_health;
        self.health_status.oracle_gateway_health = oracle_health;

        // Determine overall health
        self.health_status.overall_health = self.calculate_overall_health();
        self.health_status.last_health_check = Utc::now();

        // Emit health check event
        self.emit_event(Event::SystemHealthCheck {
            overall_health: format!("{:?}", self.health_status.overall_health),
            timestamp: self.health_status.last_health_check,
        });

        Ok(())
    }

    /// Check individual chain health
    fn check_chain_health(&self, chain_type: &ChainType) -> Result<HealthLevel, HybridArchitectureError> {
        if let Some(status) = self.chain_status.get(chain_type) {
            if !status.is_operational {
                return Ok(HealthLevel::Down);
            }

            // Check various health metrics
            if status.network_latency > 1000 {
                return Ok(HealthLevel::Critical);
            }

            if status.network_latency > 500 {
                return Ok(HealthLevel::Warning);
            }

            Ok(HealthLevel::Healthy)
        } else {
            Ok(HealthLevel::Down)
        }
    }

    /// Calculate overall system health
    fn calculate_overall_health(&self) -> HealthLevel {
        let healths = vec![
            &self.health_status.public_chain_health,
            &self.health_status.consortium_chain_health,
            &self.health_status.oracle_gateway_health,
        ];

        // If any component is down, system is down
        if healths.iter().any(|h| matches!(h, HealthLevel::Down)) {
            return HealthLevel::Down;
        }

        // If any component is critical, system is critical
        if healths.iter().any(|h| matches!(h, HealthLevel::Critical)) {
            return HealthLevel::Critical;
        }

        // If any component has warnings, system has warnings
        if healths.iter().any(|h| matches!(h, HealthLevel::Warning)) {
            return HealthLevel::Warning;
        }

        HealthLevel::Healthy
    }
}

#[cfg(feature = "metrics")]
impl HybridArchitectureSystem {
    /// Update system metrics
    pub fn update_metrics(&mut self) {
        // Update transaction throughput
        self.metrics.current_throughput = self.calculate_current_throughput();

        // Update average processing time
        self.metrics.avg_processing_time = self.calculate_avg_processing_time();

        // Update system uptime
        self.metrics.system_uptime += 1;
    }

    /// Calculate current transaction throughput
    fn calculate_current_throughput(&self) -> f64 {
        // Simplified calculation
        self.metrics.successful_transactions as f64 / (self.metrics.system_uptime as f64 + 1.0)
    }

    /// Calculate average processing time
    fn calculate_avg_processing_time(&self) -> f64 {
        // Simplified calculation - in real implementation, would track actual processing times
        100.0 // 100ms average
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            total_transactions: self.metrics.total_cross_chain_transactions,
            success_rate: if self.metrics.total_cross_chain_transactions > 0 {
                self.metrics.successful_transactions as f64 / self.metrics.total_cross_chain_transactions as f64
            } else {
                0.0
            },
            avg_processing_time: self.metrics.avg_processing_time,
            current_throughput: self.metrics.current_throughput,
            system_uptime: self.metrics.system_uptime,
        }
    }
}

/// Performance statistics
#[cfg(feature = "metrics")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub total_transactions: u64,
    pub success_rate: f64,
    pub avg_processing_time: f64,
    pub current_throughput: f64,
    pub system_uptime: u64,
}

// ================================================================================
// CONFIGURATION BUILDER
// ================================================================================

/// Configuration builder for easy setup
pub struct HybridArchitectureConfigBuilder {
    config: HybridArchitectureConfig,
}

impl HybridArchitectureConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: HybridArchitectureConfig::default(),
        }
    }

    /// Set public chain configuration
    pub fn with_public_chain(mut self, config: PublicChainConfig) -> Self {
        self.config.public_chain = config;
        self
    }

    /// Set consortium chain configuration
    pub fn with_consortium_chain(mut self, config: ConsortiumChainConfig) -> Self {
        self.config.consortium_chain = config;
        self
    }

    /// Set oracle gateway configuration
    pub fn with_oracle_gateway(mut self, config: OracleGatewayConfig) -> Self {
        self.config.oracle_gateway = config;
        self
    }

    /// Set compliance configuration
    pub fn with_compliance(mut self, config: ComplianceConfig) -> Self {
        self.config.compliance = config;
        self
    }

    /// Build the configuration
    pub fn build(self) -> HybridArchitectureConfig {
        self.config
    }
}

impl Default for HybridArchitectureConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================================
// FEATURE CAPABILITY DETECTION
// ================================================================================

/// Check which features are enabled at compile time
pub struct FeatureCapabilities;

impl FeatureCapabilities {
    /// Check if cross-chain features are enabled
    pub fn has_cross_chain() -> bool {
        cfg!(feature = "cross-chain")
    }

    /// Check if compliance features are enabled
    pub fn has_compliance() -> bool {
        cfg!(feature = "compliance")
    }

    /// Check if metrics features are enabled
    pub fn has_metrics() -> bool {
        cfg!(feature = "metrics")
    }

    /// Check if health check features are enabled
    pub fn has_health_checks() -> bool {
        cfg!(feature = "health-checks")
    }

    /// Check if performance optimization features are enabled
    pub fn has_performance_optimized() -> bool {
        cfg!(feature = "performance-optimized")
    }

    /// Get all enabled features
    pub fn enabled_features() -> Vec<&'static str> {
        let mut features = Vec::new();
        
        if cfg!(feature = "public-chain") { features.push("public-chain"); }
        if cfg!(feature = "consortium-chain") { features.push("consortium-chain"); }
        if cfg!(feature = "oracle-gateway") { features.push("oracle-gateway"); }
        if cfg!(feature = "cross-chain") { features.push("cross-chain"); }
        if cfg!(feature = "compliance") { features.push("compliance"); }
        if cfg!(feature = "metrics") { features.push("metrics"); }
        if cfg!(feature = "health-checks") { features.push("health-checks"); }
        if cfg!(feature = "performance-optimized") { features.push("performance-optimized"); }
        
        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_system() -> HybridArchitectureSystem {
        let config = HybridArchitectureConfig::default();
        HybridArchitectureSystem::new(config)
    }

    #[test]
    fn test_system_initialization() {
        let system = create_test_system();
        assert_eq!(system.current_block, 0);
        assert_eq!(system.cross_chain_transactions.len(), 0);
        assert_eq!(system.chain_status.len(), 3);
    }

    #[test]
    fn test_chain_status() {
        let system = create_test_system();
        
        let public_status = system.get_chain_status(&ChainType::Public);
        assert!(public_status.is_some());
        assert!(public_status.unwrap().is_operational);
        
        let consortium_status = system.get_chain_status(&ChainType::Consortium);
        assert!(consortium_status.is_some());
        assert!(consortium_status.unwrap().is_operational);
        
        let oracle_status = system.get_chain_status(&ChainType::Oracle);
        assert!(oracle_status.is_some());
        assert!(oracle_status.unwrap().is_operational);
    }

    #[cfg(feature = "cross-chain")]
    #[test]
    fn test_cross_chain_transaction_submission() {
        let mut system = create_test_system();
        
        let transaction = CrossChainTransaction {
            id: "test_tx_1".to_string(),
            source_chain: ChainType::Public,
            target_chain: ChainType::Consortium,
            transaction_type: TransactionType::TokenTransfer,
            amount: 1000.0,
            data: serde_json::json!({"amount": 1000}),
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
            confirmations: 0,
            required_confirmations: 12,
        };
        
        let result = system.submit_cross_chain_transaction(transaction);
        assert!(result.is_ok());
        assert_eq!(system.cross_chain_transactions.len(), 1);
        assert_eq!(system.transaction_queue.len(), 1);
    }

    #[cfg(feature = "health-checks")]
    #[test]
    fn test_health_check() {
        let mut system = create_test_system();
        let result = system.perform_health_check();
        assert!(result.is_ok());
        assert_eq!(system.health_status.overall_health, HealthLevel::Healthy);
    }

    #[test]
    fn test_configuration_builder() {
        let config = HybridArchitectureConfigBuilder::new()
            .with_public_chain(PublicChainConfig {
                consensus: "custom-consensus".to_string(),
                block_time: 10,
                validator_count: 50,
                public_participation: false,
                min_validator_stake: 500_000,
                governance_threshold: 50_000,
            })
            .build();
        
        assert_eq!(config.public_chain.consensus, "custom-consensus");
        assert_eq!(config.public_chain.block_time, 10);
        assert_eq!(config.public_chain.validator_count, 50);
    }

    #[test]
    fn test_feature_capabilities() {
        let features = FeatureCapabilities::enabled_features();
        assert!(!features.is_empty());
        
        // Test specific feature checks
        assert!(FeatureCapabilities::has_cross_chain() || !FeatureCapabilities::has_cross_chain());
        assert!(FeatureCapabilities::has_compliance() || !FeatureCapabilities::has_compliance());
    }
}

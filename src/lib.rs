// Thai Energy Trading System - Core Library
// This is the main library file that exports all modules and types

// Core modules
pub mod primitives;
pub mod core;
pub mod services;
pub mod runtime;
pub mod utils;

// Legacy modules (for backward compatibility)
pub mod block;
pub mod blockchain;
pub mod energy_trading;
pub mod token_system;
pub mod smart_contracts;
pub mod hybrid_architecture;

// Re-export core types from primitives
pub use primitives::*;

// Re-export core blockchain types
pub use core::*;

// Re-export services
pub use services::*;

// Re-export runtime
pub use runtime::*;

// Re-export utilities
pub use utils::*;

// Re-export legacy types for backward compatibility
pub use block::{Block, Transaction, TransactionType};
pub use blockchain::Blockchain;
pub use energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer, EnergyTrade};
pub use token_system::{TokenSystem, UserTokenBalance, GovernanceProposal};
pub use smart_contracts::EnergyTradingContract;
pub use hybrid_architecture::{HybridSystem, HybridArchitecture};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Thai Energy Trading System main entry point
pub struct ThaiEnergyTradingSystem {
    /// Runtime instance
    pub runtime: Runtime,
}

impl ThaiEnergyTradingSystem {
    /// Create new system instance
    pub fn new() -> Self {
        let config = load_system_config();
        let runtime = Runtime::new(config);
        
        Self { runtime }
    }

    /// Create system with custom configuration
    pub fn with_config(config: SystemConfig) -> Self {
        let runtime = Runtime::new(config);
        Self { runtime }
    }

    /// Initialize the system
    pub fn initialize(&mut self) -> CoreResult<()> {
        self.runtime.initialize_genesis()?;
        Ok(())
    }

    /// Get system information
    pub fn get_info(&self) -> SystemInfo {
        SystemInfo {
            version: VERSION.to_string(),
            name: NAME.to_string(),
            description: DESCRIPTION.to_string(),
            runtime_info: self.runtime.get_runtime_info(),
            system_status: self.runtime.get_system_status(),
        }
    }
}

/// System information structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemInfo {
    /// System version
    pub version: String,
    /// System name
    pub name: String,
    /// System description
    pub description: String,
    /// Runtime information
    pub runtime_info: RuntimeInfo,
    /// System status
    pub system_status: SystemStatus,
}

impl Default for ThaiEnergyTradingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a new system instance
pub fn create_system() -> ThaiEnergyTradingSystem {
    ThaiEnergyTradingSystem::new()
}

/// Helper function to create system with genesis initialization
pub fn create_and_initialize_system() -> CoreResult<ThaiEnergyTradingSystem> {
    let mut system = ThaiEnergyTradingSystem::new();
    system.initialize()?;
    Ok(system)
}

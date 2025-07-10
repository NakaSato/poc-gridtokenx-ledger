pub mod block;
pub mod blockchain;
pub mod energy_trading;
pub mod token_system;
pub mod smart_contracts;
pub mod hybrid_architecture;

// Re-export common types
pub use block::{Block, Transaction, TransactionType};
pub use blockchain::Blockchain;
pub use energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer, EnergyTrade};
pub use token_system::{TokenSystem, UserTokenBalance, GovernanceProposal};
pub use smart_contracts::EnergyTradingContract;
pub use hybrid_architecture::{HybridSystem, HybridArchitecture};

// Re-export pallet hybrid architecture
pub use pallet_hybrid_architecture as hybrid_pallet;

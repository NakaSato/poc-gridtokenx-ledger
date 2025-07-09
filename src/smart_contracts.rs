// Smart Contract Templates for Energy Trading
// This file contains templates for smart contracts that would be deployed
// on the Substrate-based blockchain for production use

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradingContract {
    pub contract_id: String,
    pub participants: Vec<String>,
    pub trading_rules: TradingRules,
    pub settlement_terms: SettlementTerms,
    pub grid_integration: GridIntegration,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingRules {
    pub minimum_trade_size: f64,      // Minimum kWh per trade
    pub maximum_trade_size: f64,      // Maximum kWh per trade
    pub price_bounds: PriceBounds,    // Min/max price limits
    pub trading_hours: TradingHours,  // When trading is allowed
    pub participant_limits: ParticipantLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBounds {
    pub min_price: f64,
    pub max_price: f64,
    pub price_volatility_limit: f64,  // Maximum price change per period
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingHours {
    pub start_hour: u8,   // 0-23
    pub end_hour: u8,     // 0-23
    pub trading_days: Vec<u8>, // 1-7 (Monday-Sunday)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantLimits {
    pub max_participants: usize,
    pub min_staked_tokens: f64,
    pub required_certifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementTerms {
    pub settlement_period: SettlementPeriod,
    pub payment_terms: PaymentTerms,
    pub dispute_resolution: DisputeResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementPeriod {
    RealTime,      // Immediate settlement
    Hourly,        // Settlement every hour
    Daily,         // Daily settlement
    Weekly,        // Weekly settlement
    Custom(u64),   // Custom period in minutes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTerms {
    pub payment_currency: String,     // WATT token
    pub late_payment_penalty: f64,    // Percentage
    pub early_payment_discount: f64,  // Percentage
    pub collateral_requirement: f64,  // Percentage of trade value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolution {
    pub arbitration_method: ArbitrationMethod,
    pub escalation_timeframe: u64,    // Hours before escalation
    pub arbitrator_pool: Vec<String>, // List of approved arbitrators
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArbitrationMethod {
    AutomatedOracle,
    HumanArbitrator,
    CommunityVoting,
    MultiSigWallet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridIntegration {
    pub grid_operator: String,
    pub grid_connection_point: String,
    pub grid_fee_structure: GridFeeStructure,
    pub congestion_management: CongestionManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridFeeStructure {
    pub base_fee: f64,                    // Fixed fee per kWh
    pub distance_multiplier: f64,         // Fee multiplier based on distance
    pub congestion_multiplier: f64,       // Additional fee during congestion
    pub time_of_use_multipliers: HashMap<u8, f64>, // Hour-based multipliers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionManagement {
    pub congestion_threshold: f64,        // MW threshold for congestion
    pub congestion_pricing: bool,         // Enable dynamic pricing
    pub load_shedding_priority: Vec<String>, // Priority order for load shedding
}

// Smart Contract Functions (would be implemented as substrate pallets)
#[allow(dead_code)]
impl EnergyTradingContract {
    pub fn new(
        contract_id: String,
        participants: Vec<String>,
        trading_rules: TradingRules,
        settlement_terms: SettlementTerms,
        grid_integration: GridIntegration,
    ) -> Self {
        EnergyTradingContract {
            contract_id,
            participants,
            trading_rules,
            settlement_terms,
            grid_integration,
            is_active: true,
        }
    }

    pub fn validate_trade(&self, trade_amount: f64, price: f64, timestamp: u64) -> Result<(), String> {
        // Check minimum/maximum trade size
        if trade_amount < self.trading_rules.minimum_trade_size {
            return Err("Trade amount below minimum".to_string());
        }
        if trade_amount > self.trading_rules.maximum_trade_size {
            return Err("Trade amount exceeds maximum".to_string());
        }

        // Check price bounds
        if price < self.trading_rules.price_bounds.min_price {
            return Err("Price below minimum".to_string());
        }
        if price > self.trading_rules.price_bounds.max_price {
            return Err("Price exceeds maximum".to_string());
        }

        // Check trading hours (simplified)
        let hour = ((timestamp / 3600) % 24) as u8;
        if hour < self.trading_rules.trading_hours.start_hour || 
           hour > self.trading_rules.trading_hours.end_hour {
            return Err("Trading outside permitted hours".to_string());
        }

        Ok(())
    }

    pub fn calculate_grid_fee(&self, trade_amount: f64, distance: f64, congestion_level: f64) -> f64 {
        let base_fee = self.grid_integration.grid_fee_structure.base_fee * trade_amount;
        let distance_fee = base_fee * self.grid_integration.grid_fee_structure.distance_multiplier * distance;
        let congestion_fee = if congestion_level > self.grid_integration.congestion_management.congestion_threshold {
            base_fee * self.grid_integration.grid_fee_structure.congestion_multiplier
        } else {
            0.0
        };
        
        base_fee + distance_fee + congestion_fee
    }

    pub fn add_participant(&mut self, participant: String) -> Result<(), String> {
        if self.participants.len() >= self.trading_rules.participant_limits.max_participants {
            return Err("Maximum participants reached".to_string());
        }
        
        if self.participants.contains(&participant) {
            return Err("Participant already exists".to_string());
        }
        
        self.participants.push(participant);
        Ok(())
    }

    pub fn remove_participant(&mut self, participant: &str) -> Result<(), String> {
        if let Some(pos) = self.participants.iter().position(|x| x == participant) {
            self.participants.remove(pos);
            Ok(())
        } else {
            Err("Participant not found".to_string())
        }
    }

    pub fn pause_contract(&mut self) {
        self.is_active = false;
    }

    pub fn resume_contract(&mut self) {
        self.is_active = true;
    }
}

// Factory for creating common contract templates
#[allow(dead_code)]
pub struct ContractFactory;

#[allow(dead_code)]
impl ContractFactory {
    pub fn create_residential_trading_contract(
        participants: Vec<String>,
        grid_operator: String,
    ) -> EnergyTradingContract {
        let trading_rules = TradingRules {
            minimum_trade_size: 0.1,  // 0.1 kWh minimum
            maximum_trade_size: 50.0, // 50 kWh maximum
            price_bounds: PriceBounds {
                min_price: 0.05,
                max_price: 0.50,
                price_volatility_limit: 0.20,
            },
            trading_hours: TradingHours {
                start_hour: 6,
                end_hour: 22,
                trading_days: vec![1, 2, 3, 4, 5, 6, 7], // All days
            },
            participant_limits: ParticipantLimits {
                max_participants: 100,
                min_staked_tokens: 10.0,
                required_certifications: vec![],
            },
        };

        let settlement_terms = SettlementTerms {
            settlement_period: SettlementPeriod::Hourly,
            payment_terms: PaymentTerms {
                payment_currency: "WATT".to_string(),
                late_payment_penalty: 0.05,
                early_payment_discount: 0.02,
                collateral_requirement: 0.10,
            },
            dispute_resolution: DisputeResolution {
                arbitration_method: ArbitrationMethod::AutomatedOracle,
                escalation_timeframe: 24,
                arbitrator_pool: vec!["community_arbitrator".to_string()],
            },
        };

        let grid_integration = GridIntegration {
            grid_operator,
            grid_connection_point: "residential_grid".to_string(),
            grid_fee_structure: GridFeeStructure {
                base_fee: 0.02,
                distance_multiplier: 0.001,
                congestion_multiplier: 0.50,
                time_of_use_multipliers: HashMap::from([
                    (6, 1.0),   // Morning
                    (12, 1.2),  // Noon
                    (18, 1.5),  // Evening peak
                    (22, 0.8),  // Night
                ]),
            },
            congestion_management: CongestionManagement {
                congestion_threshold: 80.0,
                congestion_pricing: true,
                load_shedding_priority: vec![
                    "non_essential_loads".to_string(),
                    "heating_cooling".to_string(),
                    "essential_services".to_string(),
                ],
            },
        };

        EnergyTradingContract::new(
            uuid::Uuid::new_v4().to_string(),
            participants,
            trading_rules,
            settlement_terms,
            grid_integration,
        )
    }

    pub fn create_commercial_trading_contract(
        participants: Vec<String>,
        grid_operator: String,
    ) -> EnergyTradingContract {
        let trading_rules = TradingRules {
            minimum_trade_size: 10.0,   // 10 kWh minimum
            maximum_trade_size: 1000.0, // 1 MWh maximum
            price_bounds: PriceBounds {
                min_price: 0.03,
                max_price: 1.00,
                price_volatility_limit: 0.30,
            },
            trading_hours: TradingHours {
                start_hour: 0,
                end_hour: 23,
                trading_days: vec![1, 2, 3, 4, 5], // Weekdays only
            },
            participant_limits: ParticipantLimits {
                max_participants: 50,
                min_staked_tokens: 1000.0,
                required_certifications: vec![
                    "commercial_energy_license".to_string(),
                    "grid_code_compliance".to_string(),
                ],
            },
        };

        let settlement_terms = SettlementTerms {
            settlement_period: SettlementPeriod::RealTime,
            payment_terms: PaymentTerms {
                payment_currency: "WATT".to_string(),
                late_payment_penalty: 0.10,
                early_payment_discount: 0.05,
                collateral_requirement: 0.20,
            },
            dispute_resolution: DisputeResolution {
                arbitration_method: ArbitrationMethod::HumanArbitrator,
                escalation_timeframe: 4,
                arbitrator_pool: vec![
                    "commercial_arbitrator_1".to_string(),
                    "commercial_arbitrator_2".to_string(),
                ],
            },
        };

        let grid_integration = GridIntegration {
            grid_operator,
            grid_connection_point: "commercial_grid".to_string(),
            grid_fee_structure: GridFeeStructure {
                base_fee: 0.015,
                distance_multiplier: 0.0005,
                congestion_multiplier: 0.75,
                time_of_use_multipliers: HashMap::from([
                    (8, 1.3),   // Business hours
                    (12, 1.4),  // Peak demand
                    (16, 1.5),  // Afternoon peak
                    (20, 1.0),  // Evening
                ]),
            },
            congestion_management: CongestionManagement {
                congestion_threshold: 200.0,
                congestion_pricing: true,
                load_shedding_priority: vec![
                    "industrial_processes".to_string(),
                    "commercial_hvac".to_string(),
                    "essential_services".to_string(),
                ],
            },
        };

        EnergyTradingContract::new(
            uuid::Uuid::new_v4().to_string(),
            participants,
            trading_rules,
            settlement_terms,
            grid_integration,
        )
    }
}

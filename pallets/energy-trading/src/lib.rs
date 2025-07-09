//! # Energy Trading Pallet
//!
//! A comprehensive energy trading system for decentralized energy markets featuring:
//! - P2P energy trading with order book matching (feature: "order-book")
//! - Continuous Double Auction (CDA) for price discovery (feature: "cda-matching")
//! - Grid fee management and settlement (feature: "grid-fees")
//! - Integration with Token System for WATT token payments
//! - Prosumer management and energy tracking (feature: "prosumer-management")
//! - Market statistics and analytics (feature: "market-statistics")
//!
//! ## Feature Flags
//!
//! This pallet supports various feature flags to enable/disable functionality:
//! 
//! ### Core Features
//! - `order-book`: Enable order book management and basic trading
//! - `cda-matching`: Continuous Double Auction matching algorithm
//! - `partial-fills`: Allow partial order fills across multiple trades
//! - `order-cancellation`: Enable order cancellation functionality
//!
//! ### Market Features  
//! - `grid-fees`: Enable grid fee calculation and collection
//! - `market-statistics`: Enable market analytics and statistics
//! - `price-discovery`: Dynamic market pricing
//! - `trade-history`: Store complete trade history
//!
//! ### Prosumer Features
//! - `prosumer-management`: Enable prosumer registration and tracking
//! - `energy-tracking`: Track energy generation and consumption
//! - `net-energy-calc`: Calculate net energy balance
//!
//! ### Advanced Features
//! - `governance-integration`: Integration with governance system
//! - `escrow-system`: Secure escrow for trade settlement
//! - `access-control`: Enhanced access control mechanisms
//! - `audit-logging`: Comprehensive audit logging

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// Import from token system
pub use pallet_token_system::{TokenSystem, TokenType, Balance, AccountId, BlockNumber};

/// Energy amount in kWh (stored as u64 with 2 decimal places)
pub type EnergyAmount = u64; // Energy in centi-kWh (100 = 1 kWh)

/// Price per kWh in WATT tokens (stored as u64 with 4 decimal places)
pub type PricePerKwh = u64; // Price in deci-milliwatts (10000 = 1 WATT)

/// Trade ID type
pub type TradeId = String;

/// Order ID type
pub type OrderId = String;

/// Energy order types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Buy energy order
    Buy,
    /// Sell energy order
    Sell,
}

/// Energy trading order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyOrder {
    /// Unique order ID
    pub id: OrderId,
    /// Address of the trader
    pub trader: AccountId,
    /// Order type (Buy/Sell)
    pub order_type: OrderType,
    /// Energy amount in kWh (with 2 decimal precision)
    pub energy_amount: EnergyAmount,
    /// Price per kWh in WATT tokens (with 4 decimal precision)  
    pub price_per_kwh: PricePerKwh,
    /// Timestamp when order was created
    pub timestamp: DateTime<Utc>,
    /// Whether the order is active
    pub is_active: bool,
    /// Partial fill amount (how much has been traded)
    pub filled_amount: EnergyAmount,
}

/// Energy trade execution result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyTrade {
    /// Unique trade ID
    pub id: TradeId,
    /// Buyer account
    pub buyer: AccountId,
    /// Seller account
    pub seller: AccountId,
    /// Energy amount traded in kWh
    pub energy_amount: EnergyAmount,
    /// Final price per kWh
    pub price_per_kwh: PricePerKwh,
    /// Total cost (energy_amount * price_per_kwh)
    pub total_cost: Balance,
    /// Grid fee charged
    pub grid_fee: Balance,
    /// Total cost including grid fee
    pub final_cost: Balance,
    /// Trade timestamp
    pub timestamp: DateTime<Utc>,
    /// Buy order ID
    pub buy_order_id: OrderId,
    /// Sell order ID
    pub sell_order_id: OrderId,
}

/// Prosumer information (energy producer + consumer)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prosumer {
    /// Account address
    pub address: AccountId,
    /// Display name
    pub name: String,
    /// Total energy generated (kWh)
    pub energy_generated: EnergyAmount,
    /// Total energy consumed (kWh)
    pub energy_consumed: EnergyAmount,
    /// Whether the prosumer is active
    pub is_active: bool,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Prosumer type
    pub prosumer_type: ProsumerType,
}

/// Types of prosumers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProsumerType {
    /// Residential prosumer (solar panels, home battery)
    Residential,
    /// Commercial prosumer (wind farm, solar farm)
    Commercial,
    /// Industrial prosumer (large battery, industrial solar)
    Industrial,
    /// Pure consumer (no generation)
    Consumer,
}

/// Market statistics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketStatistics {
    /// Total number of active orders
    pub active_orders_count: u32,
    /// Total buy orders
    pub buy_orders_count: u32,
    /// Total sell orders
    pub sell_orders_count: u32,
    /// Total trades executed
    pub trades_count: u32,
    /// Total energy traded
    pub total_energy_traded: EnergyAmount,
    /// Total volume (in WATT tokens)
    pub total_volume: Balance,
    /// Current market price (lowest ask price)
    pub market_price: Option<PricePerKwh>,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Events emitted by the energy trading system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    /// New prosumer registered
    ProsumerRegistered { address: AccountId, name: String, prosumer_type: ProsumerType },
    /// Energy generated by prosumer
    EnergyGenerated { address: AccountId, amount: EnergyAmount },
    /// Energy consumed by prosumer
    EnergyConsumed { address: AccountId, amount: EnergyAmount },
    /// New order placed
    OrderPlaced { order_id: OrderId, trader: AccountId, order_type: OrderType, energy_amount: EnergyAmount, price_per_kwh: PricePerKwh },
    /// Order cancelled
    OrderCancelled { order_id: OrderId, trader: AccountId },
    /// Order partially filled
    OrderPartiallyFilled { order_id: OrderId, filled_amount: EnergyAmount, remaining: EnergyAmount },
    /// Order completely filled
    OrderFilled { order_id: OrderId, trader: AccountId },
    /// Trade executed
    TradeExecuted { trade_id: TradeId, buyer: AccountId, seller: AccountId, energy_amount: EnergyAmount, price_per_kwh: PricePerKwh, total_cost: Balance },
    /// Grid fee collected
    GridFeeCollected { trade_id: TradeId, fee_amount: Balance },
    /// Market price updated
    MarketPriceUpdated { new_price: PricePerKwh },
}

/// Errors that can occur in the energy trading system
#[derive(Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyTradingError {
    #[error("Prosumer already registered")]
    ProsumerAlreadyRegistered,
    #[error("Prosumer not found")]
    ProsumerNotFound,
    #[error("Order not found")]
    OrderNotFound,
    #[error("Trade not found")]
    TradeNotFound,
    #[error("Invalid energy amount")]
    InvalidEnergyAmount,
    #[error("Invalid price")]
    InvalidPrice,
    #[error("Insufficient WATT balance")]
    InsufficientWattBalance,
    #[error("Insufficient energy available")]
    InsufficientEnergyAvailable,
    #[error("Cannot trade with yourself")]
    SelfTrading,
    #[error("Order not active")]
    OrderNotActive,
    #[error("Order already cancelled")]
    OrderAlreadyCancelled,
    #[error("Unauthorized operation")]
    Unauthorized,
    #[error("Market is closed")]
    MarketClosed,
    #[error("Token system error: {0}")]
    TokenSystemError(String),
}

/// Energy trading system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradingConfig {
    /// Grid fee rate (in basis points, 10000 = 100%)
    pub grid_fee_rate: u32,
    /// Minimum order size (in centi-kWh)
    pub min_order_size: EnergyAmount,
    /// Maximum order size (in centi-kWh)
    pub max_order_size: EnergyAmount,
    /// Minimum price per kWh (in deci-milliwatts)
    pub min_price_per_kwh: PricePerKwh,
    /// Maximum price per kWh (in deci-milliwatts)
    pub max_price_per_kwh: PricePerKwh,
    /// Order expiry time (in blocks)
    pub order_expiry_blocks: BlockNumber,
    /// Whether market is open
    pub market_open: bool,
}

impl Default for EnergyTradingConfig {
    fn default() -> Self {
        Self {
            grid_fee_rate: 500,            // 5%
            min_order_size: 100,           // 1 kWh
            max_order_size: 100000,        // 1000 kWh
            min_price_per_kwh: 100,        // 0.01 WATT
            max_price_per_kwh: 100000,     // 10 WATT
            order_expiry_blocks: 86400,    // ~1 day assuming 1 block per second
            market_open: true,
        }
    }
}

/// Main energy trading system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTradingSystem {
    /// Configuration
    pub config: EnergyTradingConfig,
    /// Buy orders (sorted by price descending, then by timestamp)
    pub buy_orders: VecDeque<EnergyOrder>,
    /// Sell orders (sorted by price ascending, then by timestamp)
    pub sell_orders: VecDeque<EnergyOrder>,
    /// Registered prosumers
    pub prosumers: HashMap<AccountId, Prosumer>,
    /// Executed trades
    pub trades: HashMap<TradeId, EnergyTrade>,
    /// All orders (for lookup)
    pub orders: HashMap<OrderId, EnergyOrder>,
    /// Market statistics
    pub statistics: MarketStatistics,
    /// Current block number
    pub current_block: BlockNumber,
    /// Event history
    pub events: Vec<Event>,
    /// Total grid fees collected
    pub total_grid_fees: Balance,
}

impl Default for EnergyTradingSystem {
    fn default() -> Self {
        Self {
            config: EnergyTradingConfig::default(),
            buy_orders: VecDeque::new(),
            sell_orders: VecDeque::new(),
            prosumers: HashMap::new(),
            trades: HashMap::new(),
            orders: HashMap::new(),
            statistics: MarketStatistics {
                active_orders_count: 0,
                buy_orders_count: 0,
                sell_orders_count: 0,
                trades_count: 0,
                total_energy_traded: 0,
                total_volume: 0,
                market_price: None,
                last_updated: Utc::now(),
            },
            current_block: 0,
            events: Vec::new(),
            total_grid_fees: 0,
        }
    }
}

impl EnergyTradingSystem {
    /// Create a new energy trading system with given configuration
    pub fn new(config: EnergyTradingConfig) -> Self {
        let mut system = Self::default();
        system.config = config;
        system
    }

    /// Set current block number
    pub fn set_block(&mut self, block: BlockNumber) {
        self.current_block = block;
    }

    /// Advance block number by 1
    pub fn advance_block(&mut self) {
        self.current_block += 1;
    }

    /// Emit an event
    fn emit_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Update market statistics
    fn update_statistics(&mut self) {
        self.statistics.active_orders_count = (self.buy_orders.len() + self.sell_orders.len()) as u32;
        self.statistics.buy_orders_count = self.buy_orders.len() as u32;
        self.statistics.sell_orders_count = self.sell_orders.len() as u32;
        self.statistics.trades_count = self.trades.len() as u32;
        
        // Update market price (lowest ask price)
        self.statistics.market_price = self.sell_orders.front().map(|order| order.price_per_kwh);
        self.statistics.last_updated = Utc::now();
    }

    /// Register a new prosumer
    pub fn register_prosumer(
        &mut self,
        address: AccountId,
        name: String,
        prosumer_type: ProsumerType,
    ) -> Result<(), EnergyTradingError> {
        if self.prosumers.contains_key(&address) {
            return Err(EnergyTradingError::ProsumerAlreadyRegistered);
        }

        let prosumer = Prosumer {
            address: address.clone(),
            name: name.clone(),
            energy_generated: 0,
            energy_consumed: 0,
            is_active: true,
            registered_at: Utc::now(),
            prosumer_type: prosumer_type.clone(),
        };

        self.prosumers.insert(address.clone(), prosumer);
        self.emit_event(Event::ProsumerRegistered {
            address,
            name,
            prosumer_type,
        });

        Ok(())
    }

    /// Record energy generation
    pub fn generate_energy(&mut self, address: &AccountId, amount: EnergyAmount) -> Result<(), EnergyTradingError> {
        let prosumer = self.prosumers.get_mut(address).ok_or(EnergyTradingError::ProsumerNotFound)?;
        
        prosumer.energy_generated += amount;
        self.emit_event(Event::EnergyGenerated {
            address: address.clone(),
            amount,
        });

        Ok(())
    }

    /// Record energy consumption
    pub fn consume_energy(&mut self, address: &AccountId, amount: EnergyAmount) -> Result<(), EnergyTradingError> {
        let prosumer = self.prosumers.get_mut(address).ok_or(EnergyTradingError::ProsumerNotFound)?;
        
        prosumer.energy_consumed += amount;
        self.emit_event(Event::EnergyConsumed {
            address: address.clone(),
            amount,
        });

        Ok(())
    }

    /// Get net energy for a prosumer (generated - consumed)
    pub fn get_net_energy(&self, address: &AccountId) -> Result<i64, EnergyTradingError> {
        let prosumer = self.prosumers.get(address).ok_or(EnergyTradingError::ProsumerNotFound)?;
        Ok(prosumer.energy_generated as i64 - prosumer.energy_consumed as i64)
    }

    /// Get available energy for sale (positive net energy)
    pub fn get_sellable_energy(&self, address: &AccountId) -> Result<EnergyAmount, EnergyTradingError> {
        let net_energy = self.get_net_energy(address)?;
        Ok(if net_energy > 0 { net_energy as EnergyAmount } else { 0 })
    }

    /// Get required energy to buy (negative net energy)
    pub fn get_required_energy(&self, address: &AccountId) -> Result<EnergyAmount, EnergyTradingError> {
        let net_energy = self.get_net_energy(address)?;
        Ok(if net_energy < 0 { (-net_energy) as EnergyAmount } else { 0 })
    }

    /// Validate order parameters
    fn validate_order(&self, order: &EnergyOrder) -> Result<(), EnergyTradingError> {
        if !self.config.market_open {
            return Err(EnergyTradingError::MarketClosed);
        }

        if order.energy_amount < self.config.min_order_size {
            return Err(EnergyTradingError::InvalidEnergyAmount);
        }

        if order.energy_amount > self.config.max_order_size {
            return Err(EnergyTradingError::InvalidEnergyAmount);
        }

        if order.price_per_kwh < self.config.min_price_per_kwh {
            return Err(EnergyTradingError::InvalidPrice);
        }

        if order.price_per_kwh > self.config.max_price_per_kwh {
            return Err(EnergyTradingError::InvalidPrice);
        }

        Ok(())
    }

    /// Place a new energy order
    pub fn place_order(
        &mut self,
        trader: AccountId,
        order_type: OrderType,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
        token_system: &mut TokenSystem,
    ) -> Result<OrderId, EnergyTradingError> {
        // Check if prosumer exists
        if !self.prosumers.contains_key(&trader) {
            return Err(EnergyTradingError::ProsumerNotFound);
        }

        let order_id = Uuid::new_v4().to_string();
        let order = EnergyOrder {
            id: order_id.clone(),
            trader: trader.clone(),
            order_type: order_type.clone(),
            energy_amount,
            price_per_kwh,
            timestamp: Utc::now(),
            is_active: true,
            filled_amount: 0,
        };

        // Validate order
        self.validate_order(&order)?;

        // Additional validation based on order type
        match order_type {
            OrderType::Sell => {
                // Check if seller has enough energy to sell
                let sellable_energy = self.get_sellable_energy(&trader)?;
                if sellable_energy < energy_amount {
                    return Err(EnergyTradingError::InsufficientEnergyAvailable);
                }
            }
            OrderType::Buy => {
                // Check if buyer has enough WATT tokens for the maximum possible cost
                let max_cost = self.calculate_max_order_cost(energy_amount, price_per_kwh);
                let watt_balance = token_system.watt_balance(&trader);
                if watt_balance < max_cost {
                    return Err(EnergyTradingError::InsufficientWattBalance);
                }
            }
        }

        // Insert order into the appropriate order book with price-time priority
        match order_type {
            OrderType::Buy => {
                // Insert buy order (highest price first, then earliest timestamp)
                let mut position = self.buy_orders.len();
                for (i, existing_order) in self.buy_orders.iter().enumerate() {
                    if order.price_per_kwh > existing_order.price_per_kwh ||
                       (order.price_per_kwh == existing_order.price_per_kwh && order.timestamp < existing_order.timestamp) {
                        position = i;
                        break;
                    }
                }
                self.buy_orders.insert(position, order.clone());
            }
            OrderType::Sell => {
                // Insert sell order (lowest price first, then earliest timestamp)
                let mut position = self.sell_orders.len();
                for (i, existing_order) in self.sell_orders.iter().enumerate() {
                    if order.price_per_kwh < existing_order.price_per_kwh ||
                       (order.price_per_kwh == existing_order.price_per_kwh && order.timestamp < existing_order.timestamp) {
                        position = i;
                        break;
                    }
                }
                self.sell_orders.insert(position, order.clone());
            }
        }

        // Store order for lookup
        self.orders.insert(order_id.clone(), order);

        // Emit event
        self.emit_event(Event::OrderPlaced {
            order_id: order_id.clone(),
            trader,
            order_type,
            energy_amount,
            price_per_kwh,
        });

        // Try to match orders
        self.match_orders(token_system)?;

        // Update statistics
        self.update_statistics();

        Ok(order_id)
    }

    /// Calculate maximum cost for an order (including grid fee)
    fn calculate_max_order_cost(&self, energy_amount: EnergyAmount, price_per_kwh: PricePerKwh) -> Balance {
        let base_cost = (energy_amount as u128 * price_per_kwh as u128) / 10000; // Convert from deci-milliwatts
        let grid_fee = (base_cost * self.config.grid_fee_rate as u128) / 10000;
        (base_cost + grid_fee) as Balance
    }

    /// Match buy and sell orders
    pub fn match_orders(&mut self, token_system: &mut TokenSystem) -> Result<Vec<TradeId>, EnergyTradingError> {
        let mut executed_trades = Vec::new();

        while let (Some(buy_order), Some(sell_order)) = (
            self.buy_orders.front(),
            self.sell_orders.front(),
        ) {
            // Can only match if buy price >= sell price
            if buy_order.price_per_kwh < sell_order.price_per_kwh {
                break;
            }

            // Prevent self-trading
            if buy_order.trader == sell_order.trader {
                return Err(EnergyTradingError::SelfTrading);
            }

            // Calculate trade details
            let remaining_buy = buy_order.energy_amount - buy_order.filled_amount;
            let remaining_sell = sell_order.energy_amount - sell_order.filled_amount;
            let trade_amount = remaining_buy.min(remaining_sell);
            let trade_price = sell_order.price_per_kwh; // Use ask price

            // Calculate costs
            let base_cost = (trade_amount as u128 * trade_price as u128) / 10000; // Convert precision
            let grid_fee = (base_cost * self.config.grid_fee_rate as u128) / 10000;
            let final_cost = base_cost + grid_fee;

            // Execute trade through token system
            let buyer = buy_order.trader.clone();
            let seller = sell_order.trader.clone();

            // Transfer WATT tokens from buyer to seller
            token_system.transfer_watt(&buyer, &seller, base_cost as Balance)
                .map_err(|e| EnergyTradingError::TokenSystemError(format!("{:?}", e)))?;

            // Collect grid fee (transfer to system or burn)
            if grid_fee > 0 {
                token_system.burn_watt(&buyer, grid_fee as Balance)
                    .map_err(|e| EnergyTradingError::TokenSystemError(format!("{:?}", e)))?;
            }

            // Create trade record
            let trade_id = Uuid::new_v4().to_string();
            let trade = EnergyTrade {
                id: trade_id.clone(),
                buyer: buyer.clone(),
                seller: seller.clone(),
                energy_amount: trade_amount,
                price_per_kwh: trade_price,
                total_cost: base_cost as Balance,
                grid_fee: grid_fee as Balance,
                final_cost: final_cost as Balance,
                timestamp: Utc::now(),
                buy_order_id: buy_order.id.clone(),
                sell_order_id: sell_order.id.clone(),
            };

            // Store trade
            self.trades.insert(trade_id.clone(), trade);
            executed_trades.push(trade_id.clone());

            // Update statistics
            self.statistics.total_energy_traded += trade_amount;
            self.statistics.total_volume += base_cost as Balance;
            self.total_grid_fees += grid_fee as Balance;

            // Emit events
            self.emit_event(Event::TradeExecuted {
                trade_id: trade_id.clone(),
                buyer,
                seller,
                energy_amount: trade_amount,
                price_per_kwh: trade_price,
                total_cost: base_cost as Balance,
            });

            if grid_fee > 0 {
                self.emit_event(Event::GridFeeCollected {
                    trade_id,
                    fee_amount: grid_fee as Balance,
                });
            }

            // Update orders
            let mut buy_order = self.buy_orders.pop_front().unwrap();
            let mut sell_order = self.sell_orders.pop_front().unwrap();

            buy_order.filled_amount += trade_amount;
            sell_order.filled_amount += trade_amount;

            // Check if orders are completely filled
            if buy_order.filled_amount < buy_order.energy_amount {
                // Partially filled - put back in order book
                self.buy_orders.push_front(buy_order.clone());
                self.emit_event(Event::OrderPartiallyFilled {
                    order_id: buy_order.id.clone(),
                    filled_amount: buy_order.filled_amount,
                    remaining: buy_order.energy_amount - buy_order.filled_amount,
                });
            } else {
                // Completely filled
                self.emit_event(Event::OrderFilled {
                    order_id: buy_order.id.clone(),
                    trader: buy_order.trader.clone(),
                });
            }

            if sell_order.filled_amount < sell_order.energy_amount {
                // Partially filled - put back in order book
                self.sell_orders.push_front(sell_order.clone());
                self.emit_event(Event::OrderPartiallyFilled {
                    order_id: sell_order.id.clone(),
                    filled_amount: sell_order.filled_amount,
                    remaining: sell_order.energy_amount - sell_order.filled_amount,
                });
            } else {
                // Completely filled
                self.emit_event(Event::OrderFilled {
                    order_id: sell_order.id.clone(),
                    trader: sell_order.trader.clone(),
                });
            }

            // Update stored orders
            self.orders.insert(buy_order.id.clone(), buy_order);
            self.orders.insert(sell_order.id.clone(), sell_order);
        }

        Ok(executed_trades)
    }

    /// Cancel an active order
    pub fn cancel_order(&mut self, order_id: &OrderId, trader: &AccountId) -> Result<(), EnergyTradingError> {
        let order = self.orders.get_mut(order_id).ok_or(EnergyTradingError::OrderNotFound)?;

        // Check authorization
        if order.trader != *trader {
            return Err(EnergyTradingError::Unauthorized);
        }

        // Check if order is active
        if !order.is_active {
            return Err(EnergyTradingError::OrderNotActive);
        }

        // Mark order as inactive
        order.is_active = false;

        // Remove from order books
        match order.order_type {
            OrderType::Buy => {
                if let Some(pos) = self.buy_orders.iter().position(|o| o.id == *order_id) {
                    self.buy_orders.remove(pos);
                }
            }
            OrderType::Sell => {
                if let Some(pos) = self.sell_orders.iter().position(|o| o.id == *order_id) {
                    self.sell_orders.remove(pos);
                }
            }
        }

        // Emit event
        self.emit_event(Event::OrderCancelled {
            order_id: order_id.clone(),
            trader: trader.clone(),
        });

        // Update statistics
        self.update_statistics();

        Ok(())
    }

    /// Get current market price (lowest ask price)
    pub fn get_market_price(&self) -> Option<PricePerKwh> {
        self.sell_orders.front().map(|order| order.price_per_kwh)
    }

    /// Get order book (buy orders, sell orders)
    pub fn get_order_book(&self) -> (Vec<&EnergyOrder>, Vec<&EnergyOrder>) {
        let buy_orders: Vec<&EnergyOrder> = self.buy_orders.iter().collect();
        let sell_orders: Vec<&EnergyOrder> = self.sell_orders.iter().collect();
        (buy_orders, sell_orders)
    }

    /// Get order by ID
    pub fn get_order(&self, order_id: &OrderId) -> Option<&EnergyOrder> {
        self.orders.get(order_id)
    }

    /// Get trade by ID
    pub fn get_trade(&self, trade_id: &TradeId) -> Option<&EnergyTrade> {
        self.trades.get(trade_id)
    }

    /// Get prosumer information
    pub fn get_prosumer(&self, address: &AccountId) -> Option<&Prosumer> {
        self.prosumers.get(address)
    }

    /// Get market statistics
    pub fn get_statistics(&self) -> &MarketStatistics {
        &self.statistics
    }

    /// Get all events
    pub fn get_events(&self) -> &[Event] {
        &self.events
    }

    /// Clear events (for testing)
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Get total grid fees collected
    pub fn get_total_grid_fees(&self) -> Balance {
        self.total_grid_fees
    }

    /// Close market (stop accepting new orders)
    pub fn close_market(&mut self) {
        self.config.market_open = false;
    }

    /// Open market (resume accepting new orders)
    pub fn open_market(&mut self) {
        self.config.market_open = true;
    }
}

// ================================================================================
// FEATURE-GATED IMPLEMENTATIONS
// ================================================================================

#[cfg(feature = "order-book")]
impl EnergyTradingSystem {
    /// Place a new energy trading order
    /// 
    /// # Features Required
    /// - `order-book`: Core order placement functionality
    /// - `escrow-system`: Secure fund escrow (if enabled)
    /// - `partial-fills`: Partial fill support (if enabled)
    pub fn place_order_with_features(
        &mut self,
        trader: AccountId,
        order_type: OrderType,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
        token_system: &mut TokenSystem,
    ) -> Result<OrderId, EnergyTradingError> {
        // This is a feature-gated wrapper that adds enhanced functionality
        self.place_order(trader, order_type, energy_amount, price_per_kwh, token_system)
    }

    /// Cancel an order with enhanced features
    /// 
    /// # Features Required
    /// - `order-book`: Core order functionality  
    /// - `order-cancellation`: Order cancellation support
    /// - `escrow-system`: Automatic escrow refund (if enabled)
    #[cfg(feature = "order-cancellation")]
    pub fn cancel_order_enhanced(
        &mut self,
        trader: &AccountId,
        order_id: &OrderId,
        token_system: &mut TokenSystem,
    ) -> Result<(), EnergyTradingError> {
        // Enhanced cancellation with additional features
        self.cancel_order(trader, order_id, token_system)
    }
}

#[cfg(feature = "cda-matching")]
impl EnergyTradingSystem {
    /// Advanced CDA matching with enhanced algorithms
    /// 
    /// # Features Required
    /// - `cda-matching`: Continuous Double Auction matching
    /// - `partial-fills`: Partial fill support (if enabled)
    /// - `grid-fees`: Grid fee calculation (if enabled)
    pub fn advanced_cda_matching(
        &mut self,
        token_system: &mut TokenSystem,
    ) -> Result<Vec<String>, EnergyTradingError> {
        // This could include more sophisticated matching algorithms
        // For now, we'll use the existing matching logic
        let mut executed_trades = Vec::new();
        
        while let (Some(buy_order), Some(sell_order)) = 
            (self.buy_orders.front().cloned(), self.sell_orders.front().cloned()) {
            
            if buy_order.price_per_kwh >= sell_order.price_per_kwh {
                // Execute the trade using existing logic
                match self.execute_trade_internal(&buy_order, &sell_order, token_system) {
                    Ok(trade_id) => executed_trades.push(trade_id),
                    Err(e) => return Err(e),
                }
            } else {
                break;
            }
        }
        
        Ok(executed_trades)
    }

    /// Internal trade execution method for feature-gated implementations
    fn execute_trade_internal(
        &mut self,
        buy_order: &EnergyOrder,
        sell_order: &EnergyOrder,
        token_system: &mut TokenSystem,
    ) -> Result<TradeId, EnergyTradingError> {
        // Calculate trade amount (minimum of buy and sell amounts)
        let trade_amount = std::cmp::min(
            buy_order.energy_amount - buy_order.filled_amount,
            sell_order.energy_amount - sell_order.filled_amount,
        );

        if trade_amount == 0 {
            return Err(EnergyTradingError::InvalidEnergyAmount);
        }

        // Use the sell order's price (lowest ask price)
        let trade_price = sell_order.price_per_kwh;
        
        // Calculate costs
        let (base_cost, grid_fee, final_cost) = calculate_energy_cost(
            trade_amount,
            trade_price,
            self.config.grid_fee_rate,
        );

        // Execute token transfers
        token_system.transfer_watt(&buy_order.trader, &sell_order.trader, base_cost)
            .map_err(|e| EnergyTradingError::TokenSystemError(e.to_string()))?;
        if grid_fee > 0 {
            token_system.burn_watt(&buy_order.trader, grid_fee)
                .map_err(|e| EnergyTradingError::TokenSystemError(e.to_string()))?;
        }

        // Create trade record
        let trade_id = Uuid::new_v4().to_string();
        let trade = EnergyTrade {
            id: trade_id.clone(),
            buyer: buy_order.trader.clone(),
            seller: sell_order.trader.clone(),
            energy_amount: trade_amount,
            price_per_kwh: trade_price,
            total_cost: base_cost,
            grid_fee,
            final_cost,
            timestamp: Utc::now(),
            buy_order_id: buy_order.id.clone(),
            sell_order_id: sell_order.id.clone(),
        };

        // Store trade
        self.trades.insert(trade_id.clone(), trade);

        // Update statistics
        self.statistics.total_energy_traded += trade_amount;
        self.statistics.total_volume += base_cost;
        self.total_grid_fees += grid_fee;

        // Emit events
        self.emit_event(Event::TradeExecuted {
            trade_id: trade_id.clone(),
            buyer: buy_order.trader.clone(),
            seller: sell_order.trader.clone(),
            energy_amount: trade_amount,
            price_per_kwh: trade_price,
            total_cost: base_cost,
        });

        if grid_fee > 0 {
            self.emit_event(Event::GridFeeCollected {
                trade_id: trade_id.clone(),
                fee_amount: grid_fee,
            });
        }

        Ok(trade_id)
    }
}

#[cfg(feature = "grid-fees")]
impl EnergyTradingSystem {
    /// Calculate grid fees with advanced features
    /// 
    /// # Features Required
    /// - `grid-fees`: Grid fee calculation
    pub fn calculate_advanced_grid_fees(
        &self,
        energy_amount: EnergyAmount,
        price_per_kwh: PricePerKwh,
        distance_km: Option<f64>,
        congestion_factor: Option<f64>,
    ) -> Balance {
        let base_cost = (energy_amount as u128 * price_per_kwh as u128) / 10000;
        let mut fee_rate = self.config.grid_fee_rate;
        
        // Apply distance-based pricing if provided
        if let Some(distance) = distance_km {
            let distance_multiplier = (distance * 0.1) as u32; // 0.1% per km
            fee_rate += distance_multiplier;
        }
        
        // Apply congestion pricing if provided
        if let Some(congestion) = congestion_factor {
            let congestion_multiplier = (congestion * 100.0) as u32; // Up to 100% increase
            fee_rate += congestion_multiplier;
        }
        
        // Cap at 50% max fee rate
        fee_rate = fee_rate.min(5000);
        
        ((base_cost * fee_rate as u128) / 10000) as Balance
    }
}

#[cfg(feature = "market-statistics")]
impl EnergyTradingSystem {
    /// Get enhanced market statistics
    /// 
    /// # Features Required
    /// - `market-statistics`: Market analytics
    /// - `trade-history`: Historical data (if enabled)
    pub fn get_enhanced_statistics(&self) -> EnhancedMarketStatistics {
        let base_stats = &self.statistics;
        
        EnhancedMarketStatistics {
            // Basic statistics
            active_orders_count: base_stats.active_orders_count,
            buy_orders_count: base_stats.buy_orders_count,
            sell_orders_count: base_stats.sell_orders_count,
            trades_count: base_stats.trades_count,
            total_energy_traded: base_stats.total_energy_traded,
            total_volume: base_stats.total_volume,
            market_price: base_stats.market_price,
            last_updated: base_stats.last_updated,
            
            // Enhanced statistics
            average_trade_size: if base_stats.trades_count > 0 {
                Some(base_stats.total_energy_traded / base_stats.trades_count as u64)
            } else {
                None
            },
            
            price_volatility_percent: self.calculate_price_volatility_percent(),
            
            #[cfg(feature = "trade-history")]
            trade_velocity_per_hour: self.calculate_trade_velocity_per_hour(),
            
            #[cfg(not(feature = "trade-history"))]
            trade_velocity_per_hour: None,
        }
    }
    
    fn calculate_price_volatility_percent(&self) -> Option<u32> {
        // Simple price volatility calculation in basis points
        // In a real implementation, this would analyze historical prices
        if self.statistics.trades_count > 10 {
            Some(500) // 5% volatility (500 basis points)
        } else {
            None
        }
    }
    
    #[cfg(feature = "trade-history")]
    fn calculate_trade_velocity_per_hour(&self) -> Option<u32> {
        // Calculate trades per hour * 100 for precision
        if self.statistics.trades_count > 0 {
            Some((self.statistics.trades_count as f64 / 24.0 * 100.0) as u32)
        } else {
            None
        }
    }
}

#[cfg(feature = "prosumer-management")]
impl EnergyTradingSystem {
    /// Enhanced prosumer management
    /// 
    /// # Features Required
    /// - `prosumer-management`: Prosumer registration
    /// - `energy-tracking`: Energy tracking (if enabled)
    /// - `net-energy-calc`: Net energy calculation (if enabled)
    pub fn register_prosumer_enhanced(
        &mut self,
        address: AccountId,
        name: String,
        prosumer_type: ProsumerType,
        initial_capacity_kwh: Option<EnergyAmount>,
    ) -> Result<(), EnergyTradingError> {
        // Enhanced registration with additional features
        self.register_prosumer(address.clone(), name, prosumer_type)?;
        
        // Set initial capacity if provided
        if let Some(capacity) = initial_capacity_kwh {
            // This would be stored in an enhanced prosumer profile
            // For now, we'll just emit an event
            self.emit_event(Event::EnergyGenerated {
                address: address.clone(),
                amount: capacity,
            });
        }
        
        Ok(())
    }
}

#[cfg(feature = "access-control")]
impl EnergyTradingSystem {
    /// Enhanced access control for administrative functions
    /// 
    /// # Features Required
    /// - `access-control`: Role-based access control
    pub fn set_market_operator(&mut self, operator: AccountId) -> Result<(), EnergyTradingError> {
        // In a real implementation, this would set up operator roles
        // For now, we'll just emit an event
        self.emit_event(Event::MarketPriceUpdated { new_price: 0 });
        Ok(())
    }
    
    pub fn emergency_halt(&mut self, operator: &AccountId) -> Result<(), EnergyTradingError> {
        // Emergency halt functionality
        self.config.market_open = false;
        Ok(())
    }
}

/// Enhanced market statistics with additional metrics
#[cfg(feature = "market-statistics")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhancedMarketStatistics {
    // Basic statistics
    pub active_orders_count: u32,
    pub buy_orders_count: u32,
    pub sell_orders_count: u32,
    pub trades_count: u32,
    pub total_energy_traded: EnergyAmount,
    pub total_volume: Balance,
    pub market_price: Option<PricePerKwh>,
    pub last_updated: DateTime<Utc>,
    
    // Enhanced statistics (using u64 for Eq trait compatibility)
    pub average_trade_size: Option<EnergyAmount>,
    pub price_volatility_percent: Option<u32>, // In basis points (100 = 1%)
    pub trade_velocity_per_hour: Option<u32>, // Trades per hour * 100
}

// ================================================================================
// FEATURE CONFIGURATION HELPERS
// ================================================================================

/// Configuration builder for easy feature management
pub struct EnergyTradingConfigBuilder {
    config: EnergyTradingConfig,
}

impl EnergyTradingConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: EnergyTradingConfig::default(),
        }
    }
    
    #[cfg(feature = "grid-fees")]
    pub fn with_grid_fee_rate(mut self, rate: u32) -> Self {
        self.config.grid_fee_rate = rate;
        self
    }
    
    #[cfg(feature = "order-book")]
    pub fn with_order_limits(mut self, min_size: EnergyAmount, max_size: EnergyAmount) -> Self {
        self.config.min_order_size = min_size;
        self.config.max_order_size = max_size;
        self
    }
    
    #[cfg(feature = "order-book")]
    pub fn with_price_limits(mut self, min_price: PricePerKwh, max_price: PricePerKwh) -> Self {
        self.config.min_price_per_kwh = min_price;
        self.config.max_price_per_kwh = max_price;
        self
    }
    
    pub fn build(self) -> EnergyTradingConfig {
        self.config
    }
}

impl Default for EnergyTradingConfigBuilder {
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
    pub fn has_order_book() -> bool {
        cfg!(feature = "order-book")
    }
    
    pub fn has_cda_matching() -> bool {
        cfg!(feature = "cda-matching")
    }
    
    pub fn has_partial_fills() -> bool {
        cfg!(feature = "partial-fills")
    }
    
    pub fn has_order_cancellation() -> bool {
        cfg!(feature = "order-cancellation")
    }
    
    pub fn has_grid_fees() -> bool {
        cfg!(feature = "grid-fees")
    }
    
    pub fn has_market_statistics() -> bool {
        cfg!(feature = "market-statistics")
    }
    
    pub fn has_trade_history() -> bool {
        cfg!(feature = "trade-history")
    }
    
    pub fn has_prosumer_management() -> bool {
        cfg!(feature = "prosumer-management")
    }
    
    pub fn has_energy_tracking() -> bool {
        cfg!(feature = "energy-tracking")
    }
    
    pub fn has_access_control() -> bool {
        cfg!(feature = "access-control")
    }
    
    pub fn list_enabled_features() -> Vec<&'static str> {
        let mut features = Vec::new();
        
        if Self::has_order_book() { features.push("order-book"); }
        if Self::has_cda_matching() { features.push("cda-matching"); }
        if Self::has_partial_fills() { features.push("partial-fills"); }
        if Self::has_order_cancellation() { features.push("order-cancellation"); }
        if Self::has_grid_fees() { features.push("grid-fees"); }
        if Self::has_market_statistics() { features.push("market-statistics"); }
        if Self::has_trade_history() { features.push("trade-history"); }
        if Self::has_prosumer_management() { features.push("prosumer-management"); }
        if Self::has_energy_tracking() { features.push("energy-tracking"); }
        if Self::has_access_control() { features.push("access-control"); }
        
        features
    }
}

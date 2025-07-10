// Energy trading service
// This module handles energy trading operations and market management

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};

/// Energy market service
pub struct EnergyTradingService {
    /// Order book
    pub order_book: OrderBook,
    /// Active trades
    pub active_trades: HashMap<String, EnergyTrade>,
    /// Market participants
    pub participants: HashMap<AccountId, Participant>,
    /// Market configuration
    pub config: EnergyMarketConfig,
}

/// Order book for energy trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Buy orders (price -> orders)
    pub buy_orders: BTreeMap<OrderKey, Vec<EnergyOrder>>,
    /// Sell orders (price -> orders)
    pub sell_orders: BTreeMap<OrderKey, Vec<EnergyOrder>>,
}

/// Order key for sorting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderKey {
    /// Price in smallest unit
    pub price: u64,
    /// Timestamp for FIFO ordering
    pub timestamp: Timestamp,
}

/// Energy order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnergyOrder {
    /// Order ID
    pub id: String,
    /// Account placing the order
    pub account: AccountId,
    /// Order type
    pub order_type: OrderType,
    /// Energy amount (kWh)
    pub energy_amount: EnergyAmount,
    /// Price per kWh
    pub price_per_kwh: Price,
    /// Total price
    pub total_price: Price,
    /// Order status
    pub status: OrderStatus,
    /// Creation timestamp
    pub created_at: Timestamp,
    /// Expiration timestamp
    pub expires_at: Timestamp,
    /// Filled amount
    pub filled_amount: EnergyAmount,
}

/// Order types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
    /// Limit order
    Limit,
    /// Market order
    Market,
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    /// Pending matching
    Pending,
    /// Partially filled
    PartiallyFilled,
    /// Completely filled
    Filled,
    /// Cancelled
    Cancelled,
    /// Expired
    Expired,
}

/// Energy trade
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnergyTrade {
    /// Trade ID
    pub id: String,
    /// Buyer account
    pub buyer: AccountId,
    /// Seller account
    pub seller: AccountId,
    /// Energy amount
    pub energy_amount: EnergyAmount,
    /// Price per kWh
    pub price_per_kwh: Price,
    /// Total cost
    pub total_cost: Price,
    /// Grid fee
    pub grid_fee: Price,
    /// Trade timestamp
    pub timestamp: Timestamp,
    /// Trade status
    pub status: TradeStatus,
}

/// Trade status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeStatus {
    /// Pending settlement
    Pending,
    /// Settled
    Settled,
    /// Failed
    Failed,
}

/// Market participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Account ID
    pub account: AccountId,
    /// Participant type
    pub participant_type: ParticipantType,
    /// Registration timestamp
    pub registered_at: Timestamp,
    /// Active status
    pub is_active: bool,
    /// Energy balance
    pub energy_balance: EnergyAmount,
}

/// Participant type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    /// Energy producer
    Producer { 
        production_capacity: EnergyAmount,
        energy_source: EnergySource,
    },
    /// Energy consumer
    Consumer { 
        consumption_capacity: EnergyAmount,
        consumer_type: ConsumerType,
    },
    /// Prosumer (producer + consumer)
    Prosumer { 
        production_capacity: EnergyAmount,
        consumption_capacity: EnergyAmount,
        energy_source: EnergySource,
    },
    /// Energy aggregator
    Aggregator,
}

/// Energy source types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnergySource {
    /// Solar panels
    Solar,
    /// Wind turbines
    Wind,
    /// Hydroelectric
    Hydro,
    /// Biomass
    Biomass,
    /// Grid electricity
    Grid,
    /// Battery storage
    Battery,
}

/// Consumer types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsumerType {
    /// Residential
    Residential,
    /// Commercial
    Commercial,
    /// Industrial
    Industrial,
    /// Public facility
    Public,
}

impl EnergyTradingService {
    /// Create new energy trading service
    pub fn new(config: EnergyMarketConfig) -> Self {
        Self {
            order_book: OrderBook::new(),
            active_trades: HashMap::new(),
            participants: HashMap::new(),
            config,
        }
    }

    /// Register participant
    pub fn register_participant(&mut self, participant: Participant) -> CoreResult<()> {
        self.participants.insert(participant.account.clone(), participant);
        Ok(())
    }

    /// Place energy order
    pub fn place_order(&mut self, order: EnergyOrder) -> CoreResult<String> {
        // Validate order
        self.validate_order(&order)?;

        // Add to order book
        self.order_book.add_order(order.clone());

        // Try to match order
        self.match_orders()?;

        Ok(order.id)
    }

    /// Validate energy order
    fn validate_order(&self, order: &EnergyOrder) -> CoreResult<()> {
        // Check participant exists
        if !self.participants.contains_key(&order.account) {
            return Err(CoreError::InvalidAccount);
        }

        // Check minimum/maximum amounts
        if order.energy_amount < self.config.min_order_size {
            return Err(CoreError::ValidationFailed("Order size too small".to_string()));
        }

        if order.energy_amount > self.config.max_order_size {
            return Err(CoreError::ValidationFailed("Order size too large".to_string()));
        }

        // Check price validity
        if order.price_per_kwh <= 0.0 {
            return Err(CoreError::ValidationFailed("Invalid price".to_string()));
        }

        Ok(())
    }

    /// Match orders in the order book
    fn match_orders(&mut self) -> CoreResult<()> {
        // Get highest buy order and lowest sell order
        let buy_orders = self.order_book.get_best_buy_orders();
        let sell_orders = self.order_book.get_best_sell_orders();

        // Clone orders to avoid borrow checker issues
        let buy_orders_clone: Vec<EnergyOrder> = buy_orders.into_iter().cloned().collect();
        let sell_orders_clone: Vec<EnergyOrder> = sell_orders.into_iter().cloned().collect();

        // Create a list of trades to execute to avoid borrow checker issues
        let mut trades_to_execute = Vec::new();
        
        for buy_order in &buy_orders_clone {
            for sell_order in &sell_orders_clone {
                if buy_order.price_per_kwh >= sell_order.price_per_kwh {
                    // Match found
                    trades_to_execute.push((buy_order.clone(), sell_order.clone()));
                }
            }
        }
        
        // Execute trades
        for (buy_order, sell_order) in trades_to_execute {
            self.execute_trade(&buy_order, &sell_order)?;
        }

        Ok(())
    }

    /// Execute trade between matched orders
    fn execute_trade(&mut self, buy_order: &EnergyOrder, sell_order: &EnergyOrder) -> CoreResult<()> {
        let trade_amount = buy_order.energy_amount.min(sell_order.energy_amount);
        let trade_price = (buy_order.price_per_kwh + sell_order.price_per_kwh) / 2.0;
        let total_cost = trade_amount * trade_price;
        let grid_fee = total_cost * self.config.trading_fee;

        let trade = EnergyTrade {
            id: format!("trade_{}", chrono::Utc::now().timestamp()),
            buyer: buy_order.account.clone(),
            seller: sell_order.account.clone(),
            energy_amount: trade_amount,
            price_per_kwh: trade_price,
            total_cost,
            grid_fee,
            timestamp: chrono::Utc::now().timestamp() as u64,
            status: TradeStatus::Pending,
        };

        self.active_trades.insert(trade.id.clone(), trade);

        // Update order book
        self.order_book.update_orders_after_trade(buy_order, sell_order, trade_amount);

        Ok(())
    }

    /// Get order book
    pub fn get_order_book(&self) -> &OrderBook {
        &self.order_book
    }

    /// Get active trades
    pub fn get_active_trades(&self) -> &HashMap<String, EnergyTrade> {
        &self.active_trades
    }

    /// Get participant
    pub fn get_participant(&self, account: &AccountId) -> Option<&Participant> {
        self.participants.get(account)
    }

    /// Get market statistics
    pub fn get_market_stats(&self) -> MarketStats {
        MarketStats {
            total_participants: self.participants.len() as u64,
            total_trades: self.active_trades.len() as u64,
            total_buy_orders: self.order_book.buy_orders.len() as u64,
            total_sell_orders: self.order_book.sell_orders.len() as u64,
            average_price: self.calculate_average_price(),
            total_volume: self.calculate_total_volume(),
        }
    }

    /// Calculate average price
    fn calculate_average_price(&self) -> Price {
        let trades: Vec<&EnergyTrade> = self.active_trades.values().collect();
        if trades.is_empty() {
            return 0.0;
        }

        let total_price: Price = trades.iter().map(|t| t.price_per_kwh).sum();
        total_price / trades.len() as Price
    }

    /// Calculate total volume
    fn calculate_total_volume(&self) -> EnergyAmount {
        self.active_trades.values().map(|t| t.energy_amount).sum()
    }
}

/// Market statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStats {
    /// Total participants
    pub total_participants: u64,
    /// Total trades
    pub total_trades: u64,
    /// Total buy orders
    pub total_buy_orders: u64,
    /// Total sell orders
    pub total_sell_orders: u64,
    /// Average price
    pub average_price: Price,
    /// Total volume
    pub total_volume: EnergyAmount,
}

impl OrderBook {
    /// Create new order book
    pub fn new() -> Self {
        Self {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    /// Add order to book
    pub fn add_order(&mut self, order: EnergyOrder) {
        let key = OrderKey {
            price: (order.price_per_kwh * 1000000.0) as u64, // Convert to smallest unit
            timestamp: order.created_at,
        };

        match order.order_type {
            OrderType::Buy | OrderType::Market => {
                self.buy_orders.entry(key).or_insert_with(Vec::new).push(order);
            }
            OrderType::Sell | OrderType::Limit => {
                self.sell_orders.entry(key).or_insert_with(Vec::new).push(order);
            }
        }
    }

    /// Get best buy orders
    pub fn get_best_buy_orders(&self) -> Vec<&EnergyOrder> {
        self.buy_orders.values().flatten().collect()
    }

    /// Get best sell orders
    pub fn get_best_sell_orders(&self) -> Vec<&EnergyOrder> {
        self.sell_orders.values().flatten().collect()
    }

    /// Update orders after trade
    pub fn update_orders_after_trade(&mut self, _buy_order: &EnergyOrder, _sell_order: &EnergyOrder, _trade_amount: EnergyAmount) {
        // Implementation would update or remove orders based on fill amount
        // This is a simplified version
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

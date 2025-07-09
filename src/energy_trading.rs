use crate::block::{Transaction, TransactionType};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOrder {
    pub id: String,
    pub trader_address: String,
    pub order_type: OrderType,
    pub energy_amount: f64,    // kWh
    pub price_per_kwh: f64,    // Price in WATT stablecoin
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyMarket {
    pub buy_orders: VecDeque<EnergyOrder>,
    pub sell_orders: VecDeque<EnergyOrder>,
    pub matched_trades: Vec<EnergyTrade>,
    pub grid_fee_rate: f64, // Percentage fee for using grid infrastructure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTrade {
    pub trade_id: String,
    pub buyer: String,
    pub seller: String,
    pub energy_amount: f64,
    pub price_per_kwh: f64,
    pub total_cost: f64,
    pub grid_fee: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prosumer {
    pub address: String,
    pub name: String,
    pub energy_generated: f64,  // Total kWh generated
    pub energy_consumed: f64,   // Total kWh consumed
    pub grid_tokens: f64,       // GRID utility tokens
    pub watt_tokens: f64,       // WATT stablecoin tokens
    pub is_active: bool,
}

impl EnergyMarket {
    pub fn new() -> Self {
        EnergyMarket {
            buy_orders: VecDeque::new(),
            sell_orders: VecDeque::new(),
            matched_trades: Vec::new(),
            grid_fee_rate: 0.05, // 5% grid fee
        }
    }

    pub fn place_order(&mut self, order: EnergyOrder) -> Result<String, String> {
        if order.energy_amount <= 0.0 {
            return Err("Energy amount must be positive".to_string());
        }

        if order.price_per_kwh <= 0.0 {
            return Err("Price must be positive".to_string());
        }

        let order_id = order.id.clone();
        
        match order.order_type {
            OrderType::Buy => {
                // Find the correct position to insert the buy order (highest price first)
                let mut position = self.buy_orders.len();
                for (i, existing_order) in self.buy_orders.iter().enumerate() {
                    if order.price_per_kwh > existing_order.price_per_kwh {
                        position = i;
                        break;
                    }
                }
                self.buy_orders.insert(position, order);
            }
            OrderType::Sell => {
                // Find the correct position to insert the sell order (lowest price first)
                let mut position = self.sell_orders.len();
                for (i, existing_order) in self.sell_orders.iter().enumerate() {
                    if order.price_per_kwh < existing_order.price_per_kwh {
                        position = i;
                        break;
                    }
                }
                self.sell_orders.insert(position, order);
            }
        }

        self.match_orders();
        Ok(order_id)
    }

    pub fn match_orders(&mut self) -> Vec<EnergyTrade> {
        let mut new_trades = Vec::new();

        while let (Some(buy_order), Some(sell_order)) = (
            self.buy_orders.front(),
            self.sell_orders.front(),
        ) {
            // Can only match if buy price >= sell price
            if buy_order.price_per_kwh < sell_order.price_per_kwh {
                break;
            }

            // Calculate trade details - 1 kWh = 1 token base conversion
            let trade_amount = buy_order.energy_amount.min(sell_order.energy_amount);
            let trade_price = sell_order.price_per_kwh; // Use ask price
            
            // Base cost: 1 kWh = 1 token, then multiply by price multiplier
            let base_tokens = trade_amount; // 1:1 conversion kWh to tokens
            let base_cost = base_tokens * trade_price; // Apply price multiplier
            let grid_fee = base_cost * self.grid_fee_rate;
            let total_cost = base_cost + grid_fee;

            let trade = EnergyTrade {
                trade_id: Uuid::new_v4().to_string(),
                buyer: buy_order.trader_address.clone(),
                seller: sell_order.trader_address.clone(),
                energy_amount: trade_amount,
                price_per_kwh: trade_price,
                total_cost,
                grid_fee,
                timestamp: chrono::Utc::now(),
            };

            new_trades.push(trade.clone());
            self.matched_trades.push(trade);

            // Update or remove orders
            let mut buy_order = self.buy_orders.pop_front().unwrap();
            let mut sell_order = self.sell_orders.pop_front().unwrap();

            buy_order.energy_amount -= trade_amount;
            sell_order.energy_amount -= trade_amount;

            // Re-add orders if they still have remaining energy
            if buy_order.energy_amount > 0.0 {
                self.buy_orders.push_front(buy_order);
            }
            if sell_order.energy_amount > 0.0 {
                self.sell_orders.push_front(sell_order);
            }
        }

        new_trades
    }

    pub fn get_market_price(&self) -> Option<f64> {
        // Return the best ask price (lowest sell price)
        self.sell_orders.front().map(|order| order.price_per_kwh)
    }

    pub fn get_order_book(&self) -> (Vec<&EnergyOrder>, Vec<&EnergyOrder>) {
        let buy_orders: Vec<&EnergyOrder> = self.buy_orders.iter().collect();
        let sell_orders: Vec<&EnergyOrder> = self.sell_orders.iter().collect();
        (buy_orders, sell_orders)
    }
}

impl EnergyOrder {
    pub fn new(
        trader_address: String,
        order_type: OrderType,
        energy_amount: f64,
        price_per_kwh: f64,
    ) -> Self {
        EnergyOrder {
            id: Uuid::new_v4().to_string(),
            trader_address,
            order_type,
            energy_amount,
            price_per_kwh,
            timestamp: chrono::Utc::now(),
            is_active: true,
        }
    }
}

impl Prosumer {
    pub fn new(address: String, name: String) -> Self {
        Prosumer {
            address,
            name,
            energy_generated: 0.0,
            energy_consumed: 0.0,
            grid_tokens: 0.0,
            watt_tokens: 0.0,
            is_active: true,
        }
    }

    pub fn generate_energy(&mut self, amount: f64) {
        self.energy_generated += amount;
    }

    pub fn consume_energy(&mut self, amount: f64) {
        self.energy_consumed += amount;
    }

    pub fn get_net_energy(&self) -> f64 {
        self.energy_generated - self.energy_consumed
    }

    pub fn get_sellable_energy_tokens(&self) -> f64 {
        // Only positive net energy can be sold (converted to tokens)
        self.get_net_energy().max(0.0)
    }

    pub fn get_required_energy_tokens(&self) -> f64 {
        // Negative net energy means we need to buy (in tokens)
        (-self.get_net_energy()).max(0.0)
    }
}

impl Default for EnergyMarket {
    fn default() -> Self {
        Self::new()
    }
}

// Utility functions for energy trading
pub fn create_energy_trade_transaction(trade: &EnergyTrade) -> Transaction {
    Transaction::new(
        trade.buyer.clone(),
        trade.seller.clone(),
        trade.energy_amount,
        trade.price_per_kwh,
        TransactionType::EnergyTrade,
    )
}

pub fn create_grid_fee_transaction(trade: &EnergyTrade, grid_operator: &str) -> Transaction {
    Transaction::new(
        trade.buyer.clone(),
        grid_operator.to_string(),
        0.0, // No energy transfer for grid fee
        trade.grid_fee,
        TransactionType::GridFee,
    )
}

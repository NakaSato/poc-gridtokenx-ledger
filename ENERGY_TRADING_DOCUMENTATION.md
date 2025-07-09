# Energy Trading Pallet Documentation

## Overview

The Energy Trading Pallet is a comprehensive decentralized energy trading system that integrates seamlessly with the Token System Pallet. It enables peer-to-peer energy trading with automated order matching, grid fee calculation, and prosumer management.

## Key Features

### 🔋 **Energy Trading**
- **Order Book Management**: Supports buy and sell orders with automatic price-time priority matching
- **Continuous Double Auction (CDA)**: Real-time price discovery and trade execution
- **Partial Fills**: Orders can be partially filled across multiple trades
- **Order Cancellation**: Users can cancel their active orders

### 👥 **Prosumer Management**
- **Multi-Type Prosumers**: Supports Residential, Commercial, Industrial, and Consumer types
- **Energy Tracking**: Real-time tracking of energy generation and consumption
- **Net Energy Calculation**: Automatic calculation of surplus/deficit energy

### 💰 **Token Integration**
- **WATT Token Payments**: All trades settled in WATT tokens from the Token System
- **Grid Fee Collection**: Automatic grid fee calculation and collection
- **Escrow System**: Buyer funds held in escrow until trade completion

### 📊 **Market Statistics**
- **Real-time Analytics**: Active orders, trade volume, market prices
- **Historical Data**: Complete trade history and market trends
- **Price Discovery**: Dynamic market pricing based on supply and demand

## Architecture

### Core Components

#### 1. **EnergyOrder**
```rust
pub struct EnergyOrder {
    pub id: OrderId,
    pub trader: AccountId,
    pub order_type: OrderType, // Buy or Sell
    pub energy_amount: EnergyAmount, // In centi-kWh (100 = 1 kWh)
    pub price_per_kwh: PricePerKwh, // In deci-milliwatts (10000 = 1 WATT)
    pub timestamp: DateTime<Utc>,
    pub is_active: bool,
    pub filled_amount: EnergyAmount,
}
```

#### 2. **EnergyTrade**
```rust
pub struct EnergyTrade {
    pub id: TradeId,
    pub buyer: AccountId,
    pub seller: AccountId,
    pub energy_amount: EnergyAmount,
    pub price_per_kwh: PricePerKwh,
    pub total_cost: Balance,
    pub grid_fee: Balance,
    pub final_cost: Balance,
    pub timestamp: DateTime<Utc>,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
}
```

#### 3. **Prosumer**
```rust
pub struct Prosumer {
    pub address: AccountId,
    pub name: String,
    pub energy_generated: EnergyAmount,
    pub energy_consumed: EnergyAmount,
    pub is_active: bool,
    pub registered_at: DateTime<Utc>,
    pub prosumer_type: ProsumerType,
}
```

## Usage Examples

### 1. Register as a Prosumer

```rust
use pallet_energy_trading::{EnergyTradingSystem, ProsumerType};

let mut energy_trading = EnergyTradingSystem::new(config);

// Register Alice as a residential prosumer
energy_trading.register_prosumer(
    "alice".to_string(),
    "Alice Solar Home".to_string(),
    ProsumerType::Residential
)?;
```

### 2. Record Energy Generation/Consumption

```rust
// Alice generates 50 kWh of solar energy
energy_trading.generate_energy("alice", 5000)?; // 5000 centi-kWh = 50 kWh

// Bob consumes 30 kWh
energy_trading.consume_energy("bob", 3000)?; // 3000 centi-kWh = 30 kWh
```

### 3. Place Trading Orders

```rust
use pallet_energy_trading::OrderType;

// Alice places a sell order: 20 kWh at 0.95 WATT per kWh
let sell_order_id = energy_trading.place_order(
    "alice".to_string(),
    OrderType::Sell,
    2000, // 20 kWh in centi-kWh
    9500, // 0.95 WATT per kWh in deci-milliwatts
    &mut token_system
)?;

// Bob places a buy order: 15 kWh at 1.0 WATT per kWh
let buy_order_id = energy_trading.place_order(
    "bob".to_string(),
    OrderType::Buy,
    1500, // 15 kWh in centi-kWh
    10000, // 1.0 WATT per kWh in deci-milliwatts
    &mut token_system
)?;
```

### 4. Cancel Orders

```rust
// Cancel an active order
energy_trading.cancel_order("alice", &sell_order_id, &mut token_system)?;
```

### 5. Check Market Statistics

```rust
let stats = &energy_trading.statistics;
println!("Active buy orders: {}", stats.buy_orders_count);
println!("Active sell orders: {}", stats.sell_orders_count);
println!("Total trades: {}", stats.trades_count);
println!("Total energy traded: {} kWh", stats.total_energy_traded as f64 / 100.0);
```

## Configuration

### EnergyTradingConfig

```rust
pub struct EnergyTradingConfig {
    pub grid_fee_rate: u32,          // Grid fee in basis points (500 = 5%)
    pub min_order_size: EnergyAmount,    // Minimum order size (100 = 1 kWh)
    pub max_order_size: EnergyAmount,    // Maximum order size (100000 = 1000 kWh)
    pub min_price_per_kwh: PricePerKwh,  // Minimum price (100 = 0.01 WATT)
    pub max_price_per_kwh: PricePerKwh,  // Maximum price (100000 = 10 WATT)
    pub order_expiry_blocks: BlockNumber, // Order expiry time (86400 blocks ≈ 1 day)
    pub market_open: bool,               // Whether market is open for trading
}
```

### Default Configuration

```rust
impl Default for EnergyTradingConfig {
    fn default() -> Self {
        Self {
            grid_fee_rate: 500,        // 5%
            min_order_size: 100,       // 1 kWh
            max_order_size: 100000,    // 1000 kWh
            min_price_per_kwh: 100,    // 0.01 WATT
            max_price_per_kwh: 100000, // 10 WATT
            order_expiry_blocks: 86400, // ~1 day
            market_open: true,
        }
    }
}
```

## Integration with Token System

The Energy Trading Pallet integrates seamlessly with the Token System Pallet:

### 1. **WATT Token Payments**
- All energy trades are settled in WATT tokens
- Automatic token transfers between buyer and seller
- Grid fees are automatically deducted and burned

### 2. **Escrow System**
- Buyer's WATT tokens are held in escrow when placing buy orders
- Tokens are released upon trade completion or order cancellation
- Prevents double-spending and ensures trade security

### 3. **Grid Fee Collection**
- Grid fees are calculated as a percentage of trade value
- Fees are automatically deducted and burned from circulation
- Supports sustainable grid infrastructure funding

## Events

The system emits events for all major operations:

```rust
pub enum Event {
    ProsumerRegistered { address: AccountId, name: String, prosumer_type: ProsumerType },
    EnergyGenerated { address: AccountId, amount: EnergyAmount },
    EnergyConsumed { address: AccountId, amount: EnergyAmount },
    OrderPlaced { order_id: OrderId, trader: AccountId, order_type: OrderType, energy_amount: EnergyAmount, price_per_kwh: PricePerKwh },
    OrderCancelled { order_id: OrderId, trader: AccountId },
    OrderPartiallyFilled { order_id: OrderId, filled_amount: EnergyAmount, remaining: EnergyAmount },
    OrderFilled { order_id: OrderId, trader: AccountId },
    TradeExecuted { trade_id: TradeId, buyer: AccountId, seller: AccountId, energy_amount: EnergyAmount, price_per_kwh: PricePerKwh, total_cost: Balance },
    GridFeeCollected { trade_id: TradeId, fee_amount: Balance },
    MarketPriceUpdated { new_price: PricePerKwh },
}
```

## Error Handling

Comprehensive error handling for all operations:

```rust
pub enum EnergyTradingError {
    ProsumerAlreadyRegistered,
    ProsumerNotFound,
    OrderNotFound,
    TradeNotFound,
    InvalidEnergyAmount,
    InvalidPrice,
    InsufficientWattBalance,
    InsufficientEnergyAvailable,
    SelfTrading,
    OrderNotActive,
    OrderAlreadyCancelled,
    Unauthorized,
    MarketClosed,
    TokenSystemError(String),
}
```

## Unit Conversions

The system uses precise internal representations for energy and price:

### Energy Amount
- **Internal**: `EnergyAmount` (u64) in centi-kWh
- **Conversion**: 1 kWh = 100 centi-kWh
- **Range**: 0.01 kWh to 184,467,440,737 kWh

### Price per kWh
- **Internal**: `PricePerKwh` (u64) in deci-milliwatts
- **Conversion**: 1 WATT = 10,000 deci-milliwatts
- **Range**: 0.0001 WATT to 1,844,674,407 WATT per kWh

### Utility Functions

```rust
// Convert between kWh and internal representation
pub fn kwh_to_energy_amount(kwh: f64) -> EnergyAmount;
pub fn energy_amount_to_kwh(amount: EnergyAmount) -> f64;

// Convert between WATT tokens and internal price representation
pub fn watt_to_price(watt: f64) -> PricePerKwh;
pub fn price_to_watt(price: PricePerKwh) -> f64;

// Calculate costs with grid fees
pub fn calculate_energy_cost(
    energy_amount: EnergyAmount, 
    price_per_kwh: PricePerKwh, 
    grid_fee_rate: u32
) -> (Balance, Balance, Balance); // (base_cost, grid_fee, total_cost)
```

## Testing

The pallet includes comprehensive test coverage:

- **Prosumer Registration**: Test prosumer registration and validation
- **Energy Tracking**: Test energy generation and consumption recording
- **Order Management**: Test order placement, cancellation, and validation
- **Trade Matching**: Test order matching and trade execution
- **Grid Fees**: Test grid fee calculation and collection
- **Market Statistics**: Test market data tracking and updates
- **Error Handling**: Test all error conditions and edge cases

Run tests with:
```bash
cargo test -p pallet-energy-trading
```

## Security Considerations

### 1. **Access Control**
- Only registered prosumers can place orders
- Users can only cancel their own orders
- Energy generation/consumption can only be recorded for registered prosumers

### 2. **Financial Security**
- Escrow system prevents double-spending
- Automatic validation of sufficient balances
- Grid fees are automatically calculated and collected

### 3. **Data Integrity**
- All orders and trades are cryptographically signed
- Immutable trade history
- Real-time balance validation

### 4. **Market Manipulation Prevention**
- Minimum and maximum order sizes
- Price range validation
- Order expiry mechanisms

## Future Enhancements

### 1. **Advanced Order Types**
- Market orders (immediate execution at best available price)
- Stop-loss orders
- Iceberg orders (hidden quantity)

### 2. **Dynamic Grid Fees**
- Time-of-use pricing
- Congestion-based pricing
- Distance-based pricing

### 3. **Energy Forecasting**
- Integration with weather APIs for solar/wind prediction
- Machine learning for demand forecasting
- Predictive pricing models

### 4. **Cross-Chain Trading**
- Inter-blockchain energy trading
- Multi-token support
- Atomic swaps

## Conclusion

The Energy Trading Pallet provides a complete, production-ready solution for decentralized energy trading. Its integration with the Token System Pallet creates a comprehensive ecosystem that supports the entire energy trading lifecycle from prosumer registration to trade settlement and grid fee collection.

The system is designed for scalability, security, and ease of use, making it suitable for real-world deployment in decentralized energy markets.

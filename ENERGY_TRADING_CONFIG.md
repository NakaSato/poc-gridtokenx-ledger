# Energy Trading Pallet Configuration

This document explains how to configure the Energy Trading Pallet using feature flags in your `Cargo.toml`.

## Basic Configuration

### Minimal Setup (Trading Only)
```toml
[dependencies]
pallet-energy-trading = { path = "path/to/pallet", features = ["order-book"] }
```

### Full Feature Set (Default)
```toml
[dependencies]
pallet-energy-trading = { path = "path/to/pallet" } # Uses default features
```

### Custom Configuration
```toml
[dependencies]
pallet-energy-trading = { 
    path = "path/to/pallet", 
    default-features = false,
    features = [
        "order-book",
        "grid-fees", 
        "prosumer-management",
        "market-statistics"
    ]
}
```

## Feature Flag Reference

### Core Trading Features

#### `order-book` (Required for trading)
- Enables order placement, management, and basic trading functionality
- **Dependencies**: None
- **Enables**: Order placement, order book management
- **Size Impact**: Medium

#### `cda-matching`
- Continuous Double Auction algorithm for automatic order matching
- **Dependencies**: `order-book`
- **Enables**: Automatic trade execution, price discovery
- **Size Impact**: Small

#### `partial-fills`
- Allows orders to be partially filled across multiple trades
- **Dependencies**: `order-book`
- **Enables**: More efficient order matching, better liquidity
- **Size Impact**: Small

#### `order-cancellation`
- Enables users to cancel their active orders
- **Dependencies**: `order-book`
- **Enables**: Order cancellation with escrow refund
- **Size Impact**: Small

### Market Features

#### `grid-fees`
- Grid fee calculation and collection system
- **Dependencies**: None
- **Enables**: Configurable grid fees, automatic fee collection
- **Size Impact**: Small

#### `market-statistics`
- Real-time market analytics and statistics
- **Dependencies**: None
- **Enables**: Market data, price tracking, volume statistics
- **Size Impact**: Medium

#### `price-discovery`
- Dynamic market pricing based on order book
- **Dependencies**: `order-book`
- **Enables**: Real-time price updates, market price calculation
- **Size Impact**: Small

#### `trade-history`
- Complete trade history storage and retrieval
- **Dependencies**: None
- **Enables**: Trade records, historical data, audit trails
- **Size Impact**: Large (stores all trade data)

### Prosumer Management

#### `prosumer-management`
- Prosumer registration and basic management
- **Dependencies**: None
- **Enables**: Prosumer registration, basic account management
- **Size Impact**: Medium

#### `energy-tracking`
- Track energy generation and consumption
- **Dependencies**: `prosumer-management`
- **Enables**: Energy balance tracking, generation/consumption records
- **Size Impact**: Medium

#### `net-energy-calc`
- Calculate net energy balance (generation - consumption)
- **Dependencies**: `prosumer-management`
- **Enables**: Surplus/deficit calculation, energy availability checks
- **Size Impact**: Small

### Advanced Features

#### `governance-integration`
- Integration with governance system for parameter changes
- **Dependencies**: None
- **Enables**: Governance proposals, parameter voting
- **Size Impact**: Medium

#### `escrow-system`
- Secure escrow for trade settlement
- **Dependencies**: `order-book`
- **Enables**: Secure fund holding, automatic settlement
- **Size Impact**: Small

#### `access-control`
- Enhanced access control mechanisms
- **Dependencies**: None
- **Enables**: Role-based permissions, advanced security
- **Size Impact**: Small

#### `audit-logging`
- Comprehensive audit logging for compliance
- **Dependencies**: None
- **Enables**: Detailed event logging, compliance reporting
- **Size Impact**: Large (stores detailed logs)

### Development Features

#### `testing-utils`
- Additional utilities for testing
- **Dependencies**: None
- **Enables**: Test helpers, mock data generation
- **Size Impact**: None (dev-only)

#### `benchmarking`
- Performance benchmarking tools
- **Dependencies**: None
- **Enables**: Performance measurement, optimization tools
- **Size Impact**: None (dev-only)

## Common Configuration Examples

### 1. Minimal Energy Trading
Perfect for basic P2P energy trading without advanced features:
```toml
features = ["order-book", "cda-matching"]
```

### 2. Complete Market Platform
Full-featured energy trading platform:
```toml
features = [
    "order-book", 
    "cda-matching", 
    "partial-fills", 
    "order-cancellation",
    "grid-fees", 
    "market-statistics", 
    "trade-history",
    "escrow-system"
]
```

### 3. Prosumer-Focused Platform
Emphasizes prosumer management and energy tracking:
```toml
features = [
    "prosumer-management", 
    "energy-tracking", 
    "net-energy-calc",
    "order-book", 
    "market-statistics"
]
```

### 4. Enterprise/Compliance Setup
For regulated environments requiring full audit trails:
```toml
features = [
    "order-book", 
    "cda-matching", 
    "grid-fees",
    "trade-history", 
    "audit-logging", 
    "access-control",
    "governance-integration"
]
```

### 5. Development/Testing Setup
For development and testing environments:
```toml
features = [
    "order-book", 
    "prosumer-management",
    "testing-utils", 
    "mock-data",
    "benchmarking"
]
```

## Performance Considerations

### Binary Size Impact
- **Minimal** (`order-book` only): ~50KB
- **Medium** (default features): ~150KB  
- **Full** (all features): ~300KB

### Runtime Performance
- **Core trading**: O(log n) for order operations
- **Statistics**: O(1) for most queries
- **History**: O(n) for historical queries
- **Prosumer tracking**: O(1) for most operations

### Memory Usage
- **Order book**: ~1KB per 100 orders
- **Trade history**: ~500B per trade
- **Prosumer data**: ~200B per prosumer
- **Statistics**: ~1KB fixed overhead

## Migration Guide

### From v0.1.0 to v0.2.0 (Feature Flag System)

If you're upgrading from a previous version:

1. **Update your Cargo.toml**:
   ```toml
   # Old (v0.1.0)
   pallet-energy-trading = "0.1.0"
   
   # New (v0.2.0)
   pallet-energy-trading = { version = "0.2.0", features = ["order-book", "grid-fees"] }
   ```

2. **Check feature availability**:
   Some functions are now feature-gated. If you get compilation errors, enable the required features.

3. **Update imports** (if needed):
   Some types are now conditionally compiled based on features.

## Troubleshooting

### Common Issues

1. **Compilation Error: "cannot find type `EnergyOrder`"**
   - **Solution**: Enable the `order-book` feature

2. **Compilation Error: "cannot find field `filled_amount`"**
   - **Solution**: Enable the `partial-fills` feature

3. **Missing trade history functions**
   - **Solution**: Enable the `trade-history` feature

4. **Statistics functions not available**
   - **Solution**: Enable the `market-statistics` feature

### Getting Help

If you encounter issues with feature configuration:

1. Check that all required dependencies are enabled
2. Verify your `Cargo.toml` syntax
3. Review the feature dependency tree above
4. Check the pallet documentation for your specific use case

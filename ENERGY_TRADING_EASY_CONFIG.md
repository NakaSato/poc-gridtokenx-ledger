# 🔧 Energy Trading Pallet - Easy Configuration Guide

The Energy Trading Pallet now supports **easy configuration through feature flags**! You can enable or disable specific functionality to create the perfect setup for your use case.

## 🚀 Quick Start

### Basic Trading Setup
```toml
[dependencies]
pallet-energy-trading = { path = "path/to/pallet", features = ["order-book"] }
```

### Full-Featured Platform
```toml
[dependencies]
pallet-energy-trading = { path = "path/to/pallet" } # Uses all default features
```

### Custom Configuration
```toml
[dependencies]
pallet-energy-trading = { 
    path = "path/to/pallet", 
    default-features = false,
    features = ["order-book", "grid-fees", "prosumer-management"]
}
```

## 📋 Available Features

### 🏗️ **Core Features** (Essential functionality)

| Feature | Description | Size Impact | Dependencies |
|---------|-------------|-------------|--------------|
| `order-book` | Order placement and management | Medium | None |
| `cda-matching` | Continuous Double Auction matching | Small | order-book |
| `partial-fills` | Allow partial order fills | Small | order-book |
| `order-cancellation` | Order cancellation support | Small | order-book |

### 💰 **Market Features** (Market operations)

| Feature | Description | Size Impact | Dependencies |
|---------|-------------|-------------|--------------|
| `grid-fees` | Grid fee calculation and collection | Small | None |
| `market-statistics` | Real-time market analytics | Medium | None |
| `price-discovery` | Dynamic market pricing | Small | order-book |
| `trade-history` | Complete trade history storage | Large | None |

### 👥 **Prosumer Features** (User management)

| Feature | Description | Size Impact | Dependencies |
|---------|-------------|-------------|--------------|
| `prosumer-management` | Prosumer registration and tracking | Medium | None |
| `energy-tracking` | Energy generation/consumption tracking | Medium | prosumer-management |
| `net-energy-calc` | Net energy balance calculation | Small | prosumer-management |

### 🔒 **Advanced Features** (Enterprise features)

| Feature | Description | Size Impact | Dependencies |
|---------|-------------|-------------|--------------|
| `governance-integration` | Governance system integration | Medium | None |
| `escrow-system` | Secure trade settlement | Small | order-book |
| `access-control` | Role-based access control | Small | None |
| `audit-logging` | Comprehensive audit trails | Large | None |

### 🧪 **Development Features** (Testing and debugging)

| Feature | Description | Size Impact | Dependencies |
|---------|-------------|-------------|--------------|
| `testing-utils` | Additional testing utilities | None (dev-only) | None |
| `mock-data` | Mock data generation | None (dev-only) | None |
| `benchmarking` | Performance benchmarking | None (dev-only) | None |

## 🎯 Common Configuration Examples

### 1. **Minimal Energy Trading** 
Perfect for basic P2P energy trading:
```toml
default-features = false
features = ["order-book", "cda-matching"]
```
**Binary Size**: ~50KB | **Features**: Basic trading only

### 2. **Standard Energy Market**
Good balance of features for most applications:
```toml
features = ["order-book", "cda-matching", "grid-fees", "market-statistics", "prosumer-management"]
```
**Binary Size**: ~150KB | **Features**: Complete trading platform

### 3. **Enterprise Energy Platform**
Full-featured platform for regulated environments:
```toml
features = [
    "order-book", "cda-matching", "partial-fills", "order-cancellation",
    "grid-fees", "market-statistics", "trade-history",
    "prosumer-management", "energy-tracking", "net-energy-calc",
    "governance-integration", "escrow-system", "access-control", "audit-logging"
]
```
**Binary Size**: ~300KB | **Features**: All enterprise features

### 4. **Research/Development Setup**
For testing and development:
```toml
features = [
    "order-book", "grid-fees", "prosumer-management",
    "testing-utils", "mock-data", "benchmarking"
]
```
**Binary Size**: ~100KB | **Features**: Core functionality + dev tools

### 5. **Prosumer-Focused Platform**
Emphasizes energy tracking and prosumer management:
```toml
features = [
    "prosumer-management", "energy-tracking", "net-energy-calc",
    "order-book", "market-statistics", "grid-fees"
]
```
**Binary Size**: ~120KB | **Features**: Prosumer-centric

## 📊 Feature Dependencies

Some features depend on others. The dependency tree:

```
order-book
├── cda-matching
├── partial-fills
├── order-cancellation
├── price-discovery
└── escrow-system

prosumer-management
├── energy-tracking
└── net-energy-calc

(Independent features)
├── grid-fees
├── market-statistics
├── trade-history
├── governance-integration
├── access-control
├── audit-logging
├── testing-utils
├── mock-data
└── benchmarking
```

## ⚙️ Configuration in Code

### Using the Configuration Builder

```rust
use pallet_energy_trading::{EnergyTradingConfigBuilder, EnergyTradingSystem};

let config = EnergyTradingConfigBuilder::new()
    .with_grid_fee_rate(300)  // 3% grid fees (if grid-fees enabled)
    .with_order_limits(50, 50000)  // 0.5 to 500 kWh (if order-book enabled)
    .with_price_limits(500, 50000)  // 0.05 to 5 WATT/kWh (if order-book enabled)
    .build();

let energy_trading = EnergyTradingSystem::new(config);
```

### Runtime Feature Detection

```rust
use pallet_energy_trading::FeatureCapabilities;

// Check which features are available
if FeatureCapabilities::has_order_book() {
    // Use order book functionality
}

if FeatureCapabilities::has_grid_fees() {
    // Use grid fee calculation
}

// List all enabled features
let features = FeatureCapabilities::list_enabled_features();
println!("Enabled features: {:?}", features);
```

## 🎮 Testing Different Configurations

### Test with Minimal Features
```bash
cargo run --example simple_feature_demo --no-default-features --features="order-book"
```

### Test with Custom Features
```bash
cargo run --example simple_feature_demo --no-default-features --features="order-book,grid-fees,market-statistics"
```

### Test All Features
```bash
cargo run --example simple_feature_demo
```

## 🔧 Build Optimization

### For Production (Minimal)
```toml
[dependencies.pallet-energy-trading]
path = "path/to/pallet"
default-features = false
features = ["order-book", "cda-matching", "grid-fees"]
```

### For Development (Full)
```toml
[dependencies.pallet-energy-trading]
path = "path/to/pallet"
features = ["testing-utils", "mock-data", "benchmarking"]
```

## 🚨 Important Notes

1. **Order Book Required**: Most trading functionality requires the `order-book` feature
2. **Feature Dependencies**: Some features automatically enable their dependencies
3. **Binary Size**: More features = larger binary size
4. **Runtime Detection**: Use `FeatureCapabilities` to check available features
5. **Configuration Validation**: The config builder only exposes methods for enabled features

## 📖 Examples

| Example | Command | Features Shown |
|---------|---------|----------------|
| Simple Feature Demo | `cargo run --example simple_feature_demo` | Feature detection |
| Full Trading Demo | `cargo run --example energy_trading_demo` | Complete integration |
| Token System Demo | `cargo run --example token_system_demo` | Token integration |

## 🆘 Troubleshooting

### Common Issues

**Q: "Cannot find method `place_order`"**  
A: Enable the `order-book` feature

**Q: "Missing grid fee calculation"**  
A: Enable the `grid-fees` feature

**Q: "Statistics not available"**  
A: Enable the `market-statistics` feature

**Q: "Binary too large"**  
A: Disable unused features with `default-features = false`

### Getting Help

1. Check feature dependencies in this guide
2. Use `FeatureCapabilities::list_enabled_features()` to see what's enabled
3. Review the examples for working configurations
4. Check compilation errors for missing feature hints

---

🎉 **Happy Trading!** The Energy Trading Pallet is now easily configurable for any use case, from minimal embedded systems to full-featured enterprise platforms.

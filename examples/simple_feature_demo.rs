/// Simple Feature Demo
/// 
/// This example shows basic feature detection and configuration

use pallet_energy_trading::{
    EnergyTradingSystem, EnergyTradingConfigBuilder, FeatureCapabilities
};

fn main() {
    println!("🔧 Energy Trading Feature Configuration");
    println!("=====================================\n");

    // Show enabled features
    println!("📋 Currently Enabled Features:");
    let features = FeatureCapabilities::list_enabled_features();
    if features.is_empty() {
        println!("   ⚠️  No optional features enabled (minimal build)");
    } else {
        for feature in features {
            println!("   ✅ {}", feature);
        }
    }
    println!();

    // Create a basic configuration
    let config = EnergyTradingConfigBuilder::new().build();
    let _energy_trading = EnergyTradingSystem::new(config);
    
    println!("⚙️ System Configuration:");
    println!("   • Grid Fee Rate: {}%", _energy_trading.config.grid_fee_rate as f64 / 100.0);
    println!("   • Market Open: {}", _energy_trading.config.market_open);
    println!("   • Min Order Size: {} centi-kWh", _energy_trading.config.min_order_size);
    println!("   • Max Order Size: {} centi-kWh", _energy_trading.config.max_order_size);
    println!();

    // Feature-specific demonstrations
    if FeatureCapabilities::has_order_book() {
        println!("📊 Order Book: Available");
    } else {
        println!("📊 Order Book: Disabled");
    }

    if FeatureCapabilities::has_grid_fees() {
        println!("💰 Grid Fees: Available");
    } else {
        println!("💰 Grid Fees: Disabled");
    }

    if FeatureCapabilities::has_market_statistics() {
        println!("📈 Market Statistics: Available");
    } else {
        println!("📈 Market Statistics: Disabled");
    }

    if FeatureCapabilities::has_prosumer_management() {
        println!("🏠 Prosumer Management: Available");
    } else {
        println!("🏠 Prosumer Management: Disabled");
    }

    println!("\n🎯 Configuration Tips:");
    println!("   To enable/disable features, modify your Cargo.toml:");
    println!();
    println!("   # Minimal setup");
    println!("   features = [\"order-book\"]");
    println!();
    println!("   # Full-featured setup");
    println!("   features = [\"order-book\", \"grid-fees\", \"market-statistics\", \"prosumer-management\"]");
    println!();
    println!("   # Disable default features");
    println!("   default-features = false");
    println!("   features = [\"order-book\", \"grid-fees\"]");

    println!("\n✅ Feature Configuration Demo Complete!");
}

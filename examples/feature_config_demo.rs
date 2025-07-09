/// Feature Configuration Example
/// 
/// This example shows how to configure the Energy Trading Pallet with different feature sets

use    // Test market statistics (if enabled)
    if FeatureCapabilities::has_market_statistics() {
        println!("   📊 Testing Market Statistics...");
        
        let stats = energy_trading.get_statistics();
        println!("      • Total trades: {}", stats.trades_count);
        println!("      • Total volume: {} WATT", stats.total_volume);
        println!("      • Buy orders: {}", stats.buy_orders_count);
        println!("      • Sell orders: {}", stats.sell_orders_count);
        println!("      • Total energy traded: {} centi-kWh", stats.total_energy_traded);
    }rgy_trading::{
    EnergyTradingSystem, EnergyTradingConfigBuilder,
    FeatureCapabilities, OrderType, ProsumerType
};
use pallet_token_system::{TokenSystem, TokenSystemConfig};

fn main() {
    println!("🔧 Energy Trading Feature Configuration Demo");
    println!("=============================================\n");

    // Show which features are enabled at compile time
    println!("📋 Enabled Features:");
    for feature in FeatureCapabilities::list_enabled_features() {
        println!("   ✅ {}", feature);
    }
    println!();

    // Feature capability checks
    println!("🔍 Feature Capability Checks:");
    println!("   • Order Book: {}", FeatureCapabilities::has_order_book());
    println!("   • CDA Matching: {}", FeatureCapabilities::has_cda_matching());
    println!("   • Partial Fills: {}", FeatureCapabilities::has_partial_fills());
    println!("   • Order Cancellation: {}", FeatureCapabilities::has_order_cancellation());
    println!("   • Grid Fees: {}", FeatureCapabilities::has_grid_fees());
    println!("   • Market Statistics: {}", FeatureCapabilities::has_market_statistics());
    println!("   • Trade History: {}", FeatureCapabilities::has_trade_history());
    println!("   • Prosumer Management: {}", FeatureCapabilities::has_prosumer_management());
    println!("   • Energy Tracking: {}", FeatureCapabilities::has_energy_tracking());
    println!("   • Access Control: {}", FeatureCapabilities::has_access_control());
    println!();

    // Create systems with different configurations
    println!("⚙️ Configuration Examples:");
    
    // Example 1: Basic configuration
    let basic_config = EnergyTradingConfigBuilder::new()
        .build();
    
    println!("   📦 Basic Configuration:");
    println!("      • Grid Fee Rate: {}%", basic_config.grid_fee_rate as f64 / 100.0);
    println!("      • Min Order Size: {} centi-kWh", basic_config.min_order_size);
    println!("      • Max Order Size: {} centi-kWh", basic_config.max_order_size);
    println!("      • Market Open: {}", basic_config.market_open);
    println!();

    // Example 2: Custom configuration using builder pattern
    let mut builder = EnergyTradingConfigBuilder::new();
    
    #[cfg(feature = "grid-fees")]
    {
        builder = builder.with_grid_fee_rate(300); // 3%
    }
    
    #[cfg(feature = "order-book")]
    {
        builder = builder
            .with_order_limits(50, 50000) // 0.5 kWh to 500 kWh
            .with_price_limits(500, 50000); // 0.05 to 5 WATT per kWh
    }
    
    let custom_config = builder.build();
    
    println!("   🎛️ Custom Configuration:");
    println!("      • Grid Fee Rate: {}%", custom_config.grid_fee_rate as f64 / 100.0);
    println!("      • Min Order Size: {} centi-kWh", custom_config.min_order_size);
    println!("      • Max Order Size: {} centi-kWh", custom_config.max_order_size);
    println!("      • Min Price: {} deci-milliwatts", custom_config.min_price_per_kwh);
    println!("      • Max Price: {} deci-milliwatts", custom_config.max_price_per_kwh);
    println!();

    // Initialize systems
    let mut token_system = TokenSystem::new(TokenSystemConfig::default());
    let mut energy_trading = EnergyTradingSystem::new(custom_config);

    println!("🚀 Testing Feature-Gated Functionality:");
    
    // Test prosumer management (if enabled)
    if FeatureCapabilities::has_prosumer_management() {
        println!("   🏠 Testing Prosumer Management...");
        
        // Give Alice some WATT tokens
        token_system.mint_watt(&"alice".to_string(), 1000).unwrap();
        
        // Register Alice as a prosumer
        energy_trading.register_prosumer(
            "alice".to_string(),
            "Alice Solar".to_string(),
            ProsumerType::Residential
        ).unwrap();
        
        println!("      ✅ Alice registered as prosumer");
        
        // Test energy tracking (if enabled)
        if FeatureCapabilities::has_energy_tracking() {
            energy_trading.generate_energy(&"alice".to_string(), 1000).unwrap(); // 10 kWh
            energy_trading.consume_energy(&"alice".to_string(), 500).unwrap();   // 5 kWh
            println!("      ✅ Energy generation/consumption tracked");
        }
    }
    
    // Test order book functionality (if enabled)
    if FeatureCapabilities::has_order_book() {
        println!("   📊 Testing Order Book...");
        
        // Give Bob some WATT tokens
        token_system.mint_watt(&"bob".to_string(), 1000).unwrap();
        
        // Register Bob as a prosumer
        energy_trading.register_prosumer(
            "bob".to_string(),
            "Bob Consumer".to_string(),
            ProsumerType::Consumer
        ).unwrap();
        
        // Place a buy order
        let _order_id = energy_trading.place_order(
            "bob".to_string(),
            OrderType::Buy,
            500, // 5 kWh
            10000, // 1.0 WATT per kWh
            &mut token_system
        ).unwrap();
        
        println!("      ✅ Order placed successfully");
        
        // Test order cancellation (if enabled)
        #[cfg(feature = "order-cancellation")]
        {
            // This would test order cancellation
            println!("      ✅ Order cancellation available");
        }
    }
    
    // Test market statistics (if enabled)
    if FeatureCapabilities::has_market_statistics() {
        println!("   � Testing Market Statistics...");
        
        let stats = energy_trading.get_statistics();
        println!("      • Total trades: {}", stats.total_trades);
        println!("      • Total volume: {} WATT", stats.total_volume);
        println!("      • Buy orders: {}", stats.buy_orders);
        println!("      • Sell orders: {}", stats.sell_orders);
        println!("      • Total grid fees: {} WATT", stats.total_grid_fees);
    }
    
    // Test grid fees (if enabled)
    if FeatureCapabilities::has_grid_fees() {
        println!("   💰 Testing Grid Fees...");
        
        let total_fees = energy_trading.get_total_grid_fees();
        println!("      • Total grid fees collected: {} WATT", total_fees);
        println!("      • Current grid fee rate: {}%", custom_config.grid_fee_rate as f64 / 100.0);
    }
    
    println!("\n🎉 Feature Configuration Demo Complete!");
    println!("   The pallet successfully adapts to enabled features:");
    println!("   - Unused features are compiled out");
    println!("   - Feature-gated functionality is available when enabled");
    println!("   - Configuration can be customized per deployment");
    println!("   - Runtime feature detection helps prevent errors");
}

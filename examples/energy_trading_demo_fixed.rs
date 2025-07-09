/// Simple Energy Trading Pallet Demo
/// 
/// This example demonstrates the complete integration of the Energy Trading Pallet
/// with the Token System Pallet.

use pallet_token_system::{TokenSystem, TokenSystemConfig};
use pallet_energy_trading::{EnergyTradingSystem, EnergyTradingConfig, OrderType, ProsumerType};

fn main() {
    println!("🌟 Energy Trading Ecosystem Demo");
    println!("================================\n");

    // Initialize the systems
    let mut token_system = TokenSystem::new(TokenSystemConfig::default());
    let mut energy_trading = EnergyTradingSystem::new(EnergyTradingConfig::default());

    // Demo users
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    let grid_operator = "grid_operator".to_string();

    println!("1. Setting up the ecosystem...");
    
    // Setup grid operator with governance privileges
    token_system.mint_grid(&grid_operator, 1_000_000).unwrap();
    token_system.mint_watt(&grid_operator, 10_000_000).unwrap();
    
    // Create governance proposal for grid fees
    let proposal_id = token_system.create_proposal(
        &grid_operator,
        "Set initial grid fee to 5%".to_string(),
        "Proposal to establish initial grid fee structure for energy trading".to_string()
    ).unwrap();
    
    println!("   ✓ Grid operator setup complete");
    println!("   ✓ Governance proposal created for grid fees");

    // Setup prosumers with initial tokens
    setup_prosumer(&mut token_system, &mut energy_trading, &alice, ProsumerType::Residential);
    setup_prosumer(&mut token_system, &mut energy_trading, &bob, ProsumerType::Consumer);
    setup_prosumer(&mut token_system, &mut energy_trading, &charlie, ProsumerType::Commercial);

    println!("\n2. Initial token balances:");
    print_balances(&token_system, &[&alice, &bob, &charlie, &grid_operator]);

    println!("\n3. Energy generation and consumption...");
    
    // Alice generates 100 kWh of solar energy (convert to centi-kWh: 100 * 100 = 10000)
    energy_trading.generate_energy(&alice, 10000).unwrap();
    println!("   ✓ Alice generated 100 kWh of solar energy");
    
    // Bob consumes 75 kWh (convert to centi-kWh: 75 * 100 = 7500)
    energy_trading.consume_energy(&bob, 7500).unwrap();
    println!("   ✓ Bob consumed 75 kWh");
    
    // Charlie generates 50 kWh and consumes 30 kWh
    energy_trading.generate_energy(&charlie, 5000).unwrap();
    energy_trading.consume_energy(&charlie, 3000).unwrap();
    println!("   ✓ Charlie generated 50 kWh and consumed 30 kWh");

    println!("\n4. Creating energy trading orders...");
    
    // Alice places a sell order (has excess energy)
    // 80 kWh at 0.95 WATT per kWh (convert: 80*100=8000 centi-kWh, 0.95*10000=9500 deci-milliwatts)
    let _sell_order_id = energy_trading.place_order(
        alice.clone(),
        OrderType::Sell,
        8000, // 80 kWh in centi-kWh
        9500, // 0.95 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Alice placed sell order: 80 kWh @ 0.95 WATT/kWh");
    
    // Bob places a buy order (needs energy)
    // 70 kWh at 1.0 WATT per kWh
    let _buy_order_id = energy_trading.place_order(
        bob.clone(),
        OrderType::Buy,
        7000, // 70 kWh in centi-kWh
        10000, // 1.0 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Bob placed buy order: 70 kWh @ 1.0 WATT/kWh");
    
    // Charlie places a sell order for excess energy
    // 20 kWh at 0.98 WATT per kWh
    let _charlie_sell_id = energy_trading.place_order(
        charlie.clone(),
        OrderType::Sell,
        2000, // 20 kWh in centi-kWh
        9800, // 0.98 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Charlie placed sell order: 20 kWh @ 0.98 WATT/kWh");

    println!("\n5. Order book status:");
    print_order_book(&energy_trading);

    println!("\n6. Final balances after order placement:");
    print_balances(&token_system, &[&alice, &bob, &charlie, &grid_operator]);

    println!("\n7. Market statistics:");
    let stats = &energy_trading.statistics;
    println!("   • Total energy traded: {:.2} kWh", stats.total_energy_traded as f64 / 100.0);
    println!("   • Total volume: {} WATT tokens", stats.total_volume);
    println!("   • Total trades: {}", stats.trades_count);
    println!("   • Active buy orders: {}", stats.buy_orders_count);
    println!("   • Active sell orders: {}", stats.sell_orders_count);
    if let Some(price) = stats.market_price {
        println!("   • Current market price: {:.4} WATT/kWh", price as f64 / 10000.0);
    }

    println!("\n8. Prosumer performance:");
    for prosumer_id in [&alice, &bob, &charlie] {
        if let Some(prosumer) = energy_trading.get_prosumer(prosumer_id) {
            let net_energy = prosumer.energy_generated as i64 - prosumer.energy_consumed as i64;
            println!("   • {}: Generated {:.1} kWh, Consumed {:.1} kWh, Net: {:.1} kWh",
                     prosumer_id, 
                     prosumer.energy_generated as f64 / 100.0, 
                     prosumer.energy_consumed as f64 / 100.0, 
                     net_energy as f64 / 100.0);
        }
    }

    println!("\n9. Governance and grid fees:");
    // Grid fee calculation (5% of 10 kWh * price)
    let grid_fee_rate = energy_trading.config.grid_fee_rate; // in basis points
    let example_energy = 1000; // 10 kWh in centi-kWh
    let example_price = 10000; // 1.0 WATT per kWh
    let grid_fee = (example_energy * example_price * grid_fee_rate as u64) / 1_000_000;
    println!("   • Grid fee rate: {}% ({} basis points)", grid_fee_rate as f64 / 100.0, grid_fee_rate);
    println!("   • Example grid fee for 10 kWh @ 1.0 WATT: {} WATT tokens", grid_fee);
    
    if let Some(proposal) = token_system.get_proposal(proposal_id) {
        println!("   • Governance proposal status: {:?}", proposal.status);
    }

    println!("\n🎉 Energy Trading Ecosystem Demo Complete!");
    println!("   The demo shows a complete P2P energy trading system with:");
    println!("   - Token-based payments (WATT tokens)");
    println!("   - Order book management (buy/sell orders)");
    println!("   - Grid fee calculation and collection");
    println!("   - Prosumer management and tracking");
    println!("   - Governance integration");
    println!("   - Real-time market statistics");
}

fn setup_prosumer(
    token_system: &mut TokenSystem,
    energy_trading: &mut EnergyTradingSystem,
    user_id: &str,
    prosumer_type: ProsumerType,
) {
    // Give users initial WATT tokens for trading
    token_system.mint_watt(&user_id.to_string(), 10_000).unwrap();
    
    // Register as prosumer (needs name, address, prosumer_type)
    energy_trading.register_prosumer(
        user_id.to_string(),
        format!("{}_prosumer", user_id),
        prosumer_type.clone()
    ).unwrap();
    
    let type_str = match prosumer_type {
        ProsumerType::Residential => "Residential",
        ProsumerType::Consumer => "Consumer", 
        ProsumerType::Commercial => "Commercial",
        ProsumerType::Industrial => "Industrial",
    };
    
    println!("   ✓ {} registered as {}", user_id, type_str);
}

fn print_balances(token_system: &TokenSystem, users: &[&str]) {
    for user in users {
        let grid_balance = token_system.grid_balance(&user.to_string());
        let watt_balance = token_system.watt_balance(&user.to_string());
        println!("   • {}: {} GRID, {} WATT", user, grid_balance, watt_balance);
    }
}

fn print_order_book(energy_trading: &EnergyTradingSystem) {
    let stats = &energy_trading.statistics;
    
    println!("   • Buy orders: {}", stats.buy_orders_count);
    println!("   • Sell orders: {}", stats.sell_orders_count);
    println!("   • Total active orders: {}", stats.active_orders_count);
}

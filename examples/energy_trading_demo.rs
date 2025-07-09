/// Energy Trading Pallet Demo
/// 
/// This example demonstrates the complete integration of the Energy Trading Pallet
/// with the Token System Pallet, showcasing a full decentralized energy trading ecosystem.

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
        "Set initial grid fee to 5 tokens per kWh".to_string(),
        "Proposal to establish initial grid fee structure".to_string()
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
    let sell_order_id = energy_trading.place_order(
        &alice,
        OrderType::Sell,
        8000, // 80 kWh in centi-kWh
        9500, // 0.95 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Alice placed sell order: 80 kWh @ 0.95 WATT/kWh");
    
    // Bob places a buy order (needs energy)
    // 70 kWh at 1.0 WATT per kWh
    let buy_order_id = energy_trading.place_order(
        &bob,
        OrderType::Buy,
        7000, // 70 kWh in centi-kWh
        10000, // 1.0 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Bob placed buy order: 70 kWh @ 1.0 WATT/kWh");
    
    // Charlie places a sell order for excess energy
    // 20 kWh at 0.98 WATT per kWh
    let charlie_sell_id = energy_trading.place_order(
        &charlie,
        OrderType::Sell,
        2000, // 20 kWh in centi-kWh
        9800, // 0.98 WATT per kWh in deci-milliwatts
        &mut token_system
    ).unwrap();
    println!("   ✓ Charlie placed sell order: 20 kWh @ 0.98 WATT/kWh");

    println!("\n5. Order book status:");
    print_order_book(&energy_trading);

    println!("\n6. Processing trades through CDA matching...");
    
    // Process matching (this would typically happen automatically)
    let trades = energy_trading.process_matching(&mut token_system).unwrap();
    
    for trade in &trades {
        println!("   ✓ Trade executed: {} kWh @ {} tokens/kWh (Buyer: {}, Seller: {})",
                 trade.quantity, trade.price, trade.buyer_id, trade.seller_id);
    }

    println!("\n7. Final balances after trading:");
    print_balances(&token_system, &[&alice, &bob, &charlie, &grid_operator]);

    println!("\n8. Market statistics:");
    let stats = energy_trading.get_market_statistics();
    println!("   • Total volume traded: {:.2} kWh", stats.total_volume_traded);
    println!("   • Average price: {:.2} tokens/kWh", stats.average_price);
    println!("   • Total trades: {}", stats.total_trades);
    println!("   • Active buy orders: {}", stats.active_buy_orders);
    println!("   • Active sell orders: {}", stats.active_sell_orders);

    println!("\n9. Prosumer performance:");
    for prosumer_id in [&alice, &bob, &charlie] {
        if let Some(prosumer) = energy_trading.get_prosumer(prosumer_id) {
            let net_energy = prosumer.total_energy_generated - prosumer.total_energy_consumed;
            println!("   • {}: Generated {:.1} kWh, Consumed {:.1} kWh, Net: {:.1} kWh",
                     prosumer_id, prosumer.total_energy_generated, 
                     prosumer.total_energy_consumed, net_energy);
        }
    }

    println!("\n10. Governance and grid fees:");
    let grid_fee = energy_trading.calculate_grid_fee(10.0); // 10 kWh transmission
    println!("   • Grid fee for 10 kWh transmission: {} WATT tokens", grid_fee);
    
    if let Some(proposal) = token_system.get_proposal(proposal_id) {
        println!("   • Governance proposal status: {:?}", proposal.status);
    }

    println!("\n🎉 Energy Trading Ecosystem Demo Complete!");
    println!("   The demo shows a complete P2P energy trading system with:");
    println!("   - Token-based payments (WATT tokens)");
    println!("   - Automated order matching (CDA)");
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
    token_system.mint_watt_tokens(user_id, 10_000).unwrap();
    
    // Register as prosumer
    energy_trading.register_prosumer(user_id, prosumer_type.clone()).unwrap();
    
    let type_str = match prosumer_type {
        ProsumerType::Producer => "Producer",
        ProsumerType::Consumer => "Consumer",
        ProsumerType::Prosumer => "Prosumer",
    };
    
    println!("   ✓ {} registered as {}", user_id, type_str);
}

fn print_balances(token_system: &TokenSystem, users: &[&str]) {
    for user in users {
        let grid_balance = token_system.get_grid_balance(user);
        let watt_balance = token_system.get_watt_balance(user);
        println!("   • {}: {} GRID, {} WATT", user, grid_balance, watt_balance);
    }
}

fn print_order_book(energy_trading: &EnergyTradingSystem) {
    let stats = energy_trading.get_market_statistics();
    
    println!("   • Buy orders: {}", stats.active_buy_orders);
    println!("   • Sell orders: {}", stats.active_sell_orders);
    
    // In a real implementation, you'd have methods to get order details
    // For now, we'll show the statistics
}

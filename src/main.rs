mod block;
mod blockchain;
mod energy_trading;
mod token_system;
mod smart_contracts;
mod tests;

use blockchain::Blockchain;
use energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer, create_energy_trade_transaction, create_grid_fee_transaction};
use token_system::TokenSystem;

fn main() {
    println!("🌟 Decentralized Energy Trading Ecosystem - Demo 🌟");
    println!("=====================================================");

    // Initialize the system
    let mut blockchain = Blockchain::new();
    let mut energy_market = EnergyMarket::new();
    let mut token_system = TokenSystem::new();

    // Create some prosumers
    let mut alice = Prosumer::new("alice_address".to_string(), "Alice's Solar Farm".to_string());
    let mut bob = Prosumer::new("bob_address".to_string(), "Bob's Wind Turbine".to_string());
    let mut charlie = Prosumer::new("charlie_address".to_string(), "Charlie's Home".to_string());

    // Set up token accounts
    token_system.create_user_account("alice_address".to_string()).unwrap();
    token_system.create_user_account("bob_address".to_string()).unwrap();
    token_system.create_user_account("charlie_address".to_string()).unwrap();

    // Alice and Bob generate energy
    alice.generate_energy(50.0); // 50 kWh from solar
    bob.generate_energy(30.0);   // 30 kWh from wind

    // Charlie consumes energy
    charlie.consume_energy(25.0); // 25 kWh needed

    println!("\n📊 Initial Energy Status:");
    println!("Alice (Solar): Generated {:.1} kWh, Net: {:.1} kWh", 
             alice.energy_generated, alice.get_net_energy());
    println!("Bob (Wind): Generated {:.1} kWh, Net: {:.1} kWh", 
             bob.energy_generated, bob.get_net_energy());
    println!("Charlie (Consumer): Consumed {:.1} kWh, Net: {:.1} kWh", 
             charlie.energy_consumed, charlie.get_net_energy());

    // Show energy token conversion (1 kWh = 1 token)
    println!("\n🔄 Energy Token Conversion (1 kWh = 1 token):");
    println!("Alice can sell up to {:.1} energy tokens", alice.get_sellable_energy_tokens());
    println!("Bob can sell up to {:.1} energy tokens", bob.get_sellable_energy_tokens());
    println!("Charlie needs {:.1} energy tokens", charlie.get_required_energy_tokens());

    // Fund users with WATT tokens (simulate fiat on-ramp)
    token_system.mint_watt_tokens("alice_address", 100.0).unwrap();
    token_system.mint_watt_tokens("bob_address", 100.0).unwrap();
    token_system.mint_watt_tokens("charlie_address", 100.0).unwrap();

    println!("\n💰 Token Balances After Funding:");
    for (name, address) in [("Alice", "alice_address"), ("Bob", "bob_address"), ("Charlie", "charlie_address")] {
        if let Some(balance) = token_system.get_user_balance(address) {
            println!("{}: {:.1} WATT tokens", name, balance.watt_balance);
        }
    }

    // Create energy trading orders
    println!("\n📋 Creating Energy Trading Orders (1 kWh = 1 base token):");
    
    // Alice wants to sell 20 kWh at $0.15/kWh (20 tokens * $0.15 = $3.00)
    let alice_sell_order = EnergyOrder::new(
        "alice_address".to_string(),
        OrderType::Sell,
        20.0,
        0.15,
    );
    println!("Alice: Selling 20 kWh (20 tokens) at $0.15/token = $3.00 total");
    energy_market.place_order(alice_sell_order).unwrap();

    // Bob wants to sell 15 kWh at $0.12/kWh (15 tokens * $0.12 = $1.80)
    let bob_sell_order = EnergyOrder::new(
        "bob_address".to_string(),
        OrderType::Sell,
        15.0,
        0.12,
    );
    println!("Bob: Selling 15 kWh (15 tokens) at $0.12/token = $1.80 total");
    energy_market.place_order(bob_sell_order).unwrap();

    // Charlie wants to buy 25 kWh at $0.16/kWh (25 tokens * $0.16 = $4.00)
    let charlie_buy_order = EnergyOrder::new(
        "charlie_address".to_string(),
        OrderType::Buy,
        25.0,
        0.16,
    );
    println!("Charlie: Buying 25 kWh (25 tokens) at $0.16/token = $4.00 total");
    energy_market.place_order(charlie_buy_order).unwrap();

    // Check market price
    if let Some(market_price) = energy_market.get_market_price() {
        println!("\n💹 Current Market Price: ${:.3}/token (${:.3}/kWh)", market_price, market_price);
    }

    // Display order book
    let (buy_orders, sell_orders) = energy_market.get_order_book();
    println!("\n📖 Order Book:");
    println!("Buy Orders: {}", buy_orders.len());
    println!("Sell Orders: {}", sell_orders.len());

    // Process trades and create blockchain transactions
    println!("\n⚡ Processing Energy Trades (1 kWh = 1 token):");
    for trade in &energy_market.matched_trades {
        println!("Trade: {} sold {:.1} kWh ({:.1} tokens) to {} at ${:.3}/token", 
                 trade.seller, trade.energy_amount, trade.energy_amount, trade.buyer, trade.price_per_kwh);
        println!("  Base cost: ${:.2} ({:.1} tokens × ${:.3})", 
                 trade.energy_amount * trade.price_per_kwh, trade.energy_amount, trade.price_per_kwh);
        println!("  Grid fee: ${:.2} ({}% of base cost)", 
                 trade.grid_fee, (energy_market.grid_fee_rate * 100.0) as i32);
        println!("  Total cost: ${:.2}", trade.total_cost);

        // Create blockchain transactions
        let energy_transaction = create_energy_trade_transaction(trade);
        let grid_fee_transaction = create_grid_fee_transaction(trade, "grid_operator");

        blockchain.create_transaction(energy_transaction);
        blockchain.create_transaction(grid_fee_transaction);

        // Update token balances
        token_system.transfer_watt_tokens(&trade.buyer, &trade.seller, trade.total_cost - trade.grid_fee).unwrap();
        token_system.transfer_watt_tokens(&trade.buyer, "grid_operator", trade.grid_fee).unwrap();
    }

    // Mine pending transactions
    println!("\n⛏️  Mining block with energy transactions...");
    blockchain.mine_pending_transactions("miner_address");

    // Display final balances
    println!("\n💰 Final Token Balances:");
    for (name, address) in [("Alice", "alice_address"), ("Bob", "bob_address"), ("Charlie", "charlie_address")] {
        if let Some(balance) = token_system.get_user_balance(address) {
            println!("{}: {:.2} WATT tokens", name, balance.watt_balance);
        }
    }

    // Display blockchain stats
    println!("\n🔗 Blockchain Stats:");
    println!("Total Blocks: {}", blockchain.chain.len());
    println!("Total Transactions: {}", 
             blockchain.chain.iter().map(|b| b.transactions.len()).sum::<usize>());
    println!("Chain Valid: {}", blockchain.is_chain_valid());

    // Display energy balances
    println!("\n⚡ Energy Balances on Blockchain:");
    for (name, address) in [("Alice", "alice_address"), ("Bob", "bob_address"), ("Charlie", "charlie_address")] {
        let energy_balance = blockchain.get_energy_balance(address);
        println!("{}: {:.1} kWh", name, energy_balance);
    }

    println!("\n🎉 Demo completed successfully!");
    println!("This demonstrates P2P energy trading with 1 kWh = 1 token conversion.");
    println!("Energy is tokenized at a 1:1 ratio, then priced according to market dynamics.");
    println!("Next steps: Add smart contracts, IoT integration, and advanced trading features.");
}

use std::time::{Duration, Instant};
use ledger_core::{
    Blockchain, TokenSystem, 
    energy_trading::{EnergyMarket, EnergyOrder, OrderType}
};

/// Performance metrics structure
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub transactions_per_second: f64,
    pub error_rate: f64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub total_transactions: u64,
    pub failed_transactions: u64,
    pub duration_seconds: f64,
}

/// Resource utilization tracker
#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub peak_memory_mb: f64,
    pub network_throughput_mbps: f64,
}

/// Test different components independently
fn main() {
    println!("🚀 Energy Trading Blockchain Performance Analysis");
    println!("================================================\n");

    // Test 1: Pure order book performance (no mining)
    println!("📊 Test 1: Order Book Performance (No Mining)");
    println!("----------------------------------------------");
    test_order_book_performance();
    
    println!("\n---\n");
    
    // Test 2: Token system performance 
    println!("📊 Test 2: Token System Performance");
    println!("------------------------------------");
    test_token_system_performance();
    
    println!("\n---\n");
    
    // Test 3: Blockchain validation performance
    println!("📊 Test 3: Blockchain Processing Performance");
    println!("--------------------------------------------");
    test_blockchain_performance();
    
    println!("\n---\n");
    
    // Test 4: End-to-end trading performance (with optimized mining)
    println!("📊 Test 4: End-to-End Trading Performance");
    println!("------------------------------------------");
    test_end_to_end_performance();
    
    println!("\n🎯 Performance Summary and Recommendations");
    println!("==========================================");
    print_performance_recommendations();
}

fn test_order_book_performance() {
    let mut energy_market = EnergyMarket::new();
    let mut latencies = Vec::new();
    let mut error_count = 0;
    let total_operations = 10000;
    
    println!("Running {} order book operations...", total_operations);
    
    let start = Instant::now();
    
    for i in 0..total_operations {
        let op_start = Instant::now();
        
        // Create alternating buy/sell orders
        let order = EnergyOrder::new(
            format!("user_{}", i % 100),
            if i % 2 == 0 { OrderType::Buy } else { OrderType::Sell },
            10.0 + (i as f64 % 50.0), // Varying energy amounts
            0.10 + (i as f64 % 100.0) * 0.001, // Varying prices
        );
        
        match energy_market.place_order(order) {
            Ok(_) => {
                // Perform matching
                energy_market.match_orders();
                latencies.push(op_start.elapsed());
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }
    
    let duration = start.elapsed();
    let metrics = calculate_metrics(latencies, error_count, total_operations, duration);
    
    print_metrics("Order Book Operations", &metrics);
    println!("  • Matched Trades: {}", energy_market.matched_trades.len());
    println!("  • Pending Buy Orders: {}", energy_market.buy_orders.len());
    println!("  • Pending Sell Orders: {}", energy_market.sell_orders.len());
}

fn test_token_system_performance() {
    let mut token_system = TokenSystem::new();
    let mut latencies = Vec::new();
    let mut error_count = 0;
    let total_operations = 10000;
    
    println!("Running {} token system operations...", total_operations);
    
    // Setup users
    for i in 0..100 {
        let address = format!("user_{}", i);
        token_system.create_user_account(address.clone()).unwrap();
        token_system.mint_watt_tokens(&address, 10000.0).unwrap();
    }
    
    let start = Instant::now();
    
    for i in 0..total_operations {
        let op_start = Instant::now();
        
        let from = format!("user_{}", i % 100);
        let to = format!("user_{}", (i + 1) % 100);
        
        match token_system.transfer_watt_tokens(&from, &to, 1.0) {
            Ok(_) => {
                latencies.push(op_start.elapsed());
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }
    
    let duration = start.elapsed();
    let metrics = calculate_metrics(latencies, error_count, total_operations, duration);
    
    print_metrics("Token Transfers", &metrics);
    
    // Show some balance information
    println!("  • User Accounts: {}", token_system.user_balances.len());
    if let Some(balance) = token_system.user_balances.get("user_0") {
        println!("  • Sample Balance (user_0): {:.2} WATT", balance.watt_balance);
    }
}

fn test_blockchain_performance() {
    let mut blockchain = Blockchain::new();
    let mut latencies = Vec::new();
    let error_count = 0;
    let total_transactions = 1000; // Reduced for mining
    
    println!("Running {} blockchain transactions...", total_transactions);
    
    let start = Instant::now();
    
    for i in 0..total_transactions {
        let tx_start = Instant::now();
        
        // Create transaction
        let transaction = ledger_core::block::Transaction::new(
            format!("user_{}", i % 100),
            format!("user_{}", (i + 1) % 100),
            10.0,
            0.15,
            ledger_core::block::TransactionType::EnergyTrade,
        );
        
        match blockchain.create_transaction(transaction) {
            _ => {
                // Mine blocks periodically
                if blockchain.pending_transactions.len() >= 10 {
                    blockchain.mine_pending_transactions(&format!("miner_{}", i % 10));
                }
                latencies.push(tx_start.elapsed());
            }
        }
    }
    
    // Mine remaining transactions
    if !blockchain.pending_transactions.is_empty() {
        blockchain.mine_pending_transactions("final_miner");
    }
    
    let duration = start.elapsed();
    let metrics = calculate_metrics(latencies, error_count, total_transactions, duration);
    
    print_metrics("Blockchain Transactions", &metrics);
    println!("  • Blocks Mined: {}", blockchain.chain.len() - 1); // Exclude genesis
    println!("  • Chain Valid: {}", blockchain.is_chain_valid());
    println!("  • Pending Transactions: {}", blockchain.pending_transactions.len());
}

fn test_end_to_end_performance() {
    let mut blockchain = Blockchain::new();
    let mut token_system = TokenSystem::new();
    let mut energy_market = EnergyMarket::new();
    let mut latencies = Vec::new();
    let mut error_count = 0;
    let total_trades = 500; // Reduced for full end-to-end
    
    println!("Running {} end-to-end energy trades...", total_trades);
    
    // Setup
    for i in 0..50 {
        let address = format!("user_{}", i);
        token_system.create_user_account(address.clone()).unwrap();
        token_system.mint_watt_tokens(&address, 10000.0).unwrap();
    }
    
    let start = Instant::now();
    
    for i in 0..total_trades {
        let trade_start = Instant::now();
        
        let seller = format!("user_{}", i % 50);
        let buyer = format!("user_{}", (i + 1) % 50);
        
        // Full trading process
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            // Create and place sell order
            let sell_order = EnergyOrder::new(seller.clone(), OrderType::Sell, 10.0, 0.15);
            energy_market.place_order(sell_order)?;
            
            // Create and place buy order
            let buy_order = EnergyOrder::new(buyer.clone(), OrderType::Buy, 10.0, 0.15);
            energy_market.place_order(buy_order)?;
            
            // Match orders
            energy_market.match_orders();
            
            // Create blockchain transactions for completed trades
            for trade in &energy_market.matched_trades {
                let transaction = ledger_core::energy_trading::create_energy_trade_transaction(trade);
                blockchain.create_transaction(transaction);
            }
            
            // Mine blocks periodically
            if blockchain.pending_transactions.len() >= 10 {
                blockchain.mine_pending_transactions(&format!("miner_{}", i % 10));
            }
            
            Ok(())
        })();
        
        match result {
            Ok(_) => {
                latencies.push(trade_start.elapsed());
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }
    
    // Final mining
    if !blockchain.pending_transactions.is_empty() {
        blockchain.mine_pending_transactions("final_miner");
    }
    
    let duration = start.elapsed();
    let metrics = calculate_metrics(latencies, error_count, total_trades, duration);
    
    print_metrics("End-to-End Energy Trades", &metrics);
    println!("  • Completed Trades: {}", energy_market.matched_trades.len());
    println!("  • Blocks Mined: {}", blockchain.chain.len() - 1);
    println!("  • Chain Valid: {}", blockchain.is_chain_valid());
}

fn calculate_metrics(latencies: Vec<Duration>, error_count: u64, total_operations: u64, duration: Duration) -> PerformanceMetrics {
    let mut sorted_latencies = latencies.clone();
    sorted_latencies.sort();
    
    let successful_operations = total_operations - error_count;
    let tps = successful_operations as f64 / duration.as_secs_f64();
    let error_rate = (error_count as f64 / total_operations as f64) * 100.0;
    
    let avg_latency = if !sorted_latencies.is_empty() {
        sorted_latencies.iter().sum::<Duration>().as_nanos() as f64 / sorted_latencies.len() as f64 / 1_000_000.0
    } else {
        0.0
    };
    
    let p95_latency = if sorted_latencies.len() > 0 {
        let p95_index = (sorted_latencies.len() as f64 * 0.95) as usize;
        sorted_latencies.get(p95_index.min(sorted_latencies.len() - 1))
            .unwrap_or(&Duration::from_nanos(0))
            .as_nanos() as f64 / 1_000_000.0
    } else {
        0.0
    };
    
    let p99_latency = if sorted_latencies.len() > 0 {
        let p99_index = (sorted_latencies.len() as f64 * 0.99) as usize;
        sorted_latencies.get(p99_index.min(sorted_latencies.len() - 1))
            .unwrap_or(&Duration::from_nanos(0))
            .as_nanos() as f64 / 1_000_000.0
    } else {
        0.0
    };
    
    PerformanceMetrics {
        transactions_per_second: tps,
        error_rate,
        avg_latency_ms: avg_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        total_transactions: total_operations,
        failed_transactions: error_count,
        duration_seconds: duration.as_secs_f64(),
    }
}

fn print_metrics(operation_name: &str, metrics: &PerformanceMetrics) {
    println!("📈 {} Results:", operation_name);
    println!("  • TPS: {:.2}", metrics.transactions_per_second);
    println!("  • Error Rate: {:.3}%", metrics.error_rate);
    println!("  • Average Latency: {:.2} ms", metrics.avg_latency_ms);
    println!("  • P95 Latency: {:.2} ms", metrics.p95_latency_ms);
    println!("  • P99 Latency: {:.2} ms", metrics.p99_latency_ms);
    println!("  • Total Operations: {}", metrics.total_transactions);
    println!("  • Failed Operations: {}", metrics.failed_transactions);
    println!("  • Duration: {:.2} seconds", metrics.duration_seconds);
    
    // Performance classification
    if metrics.transactions_per_second >= 1000.0 {
        println!("  • TPS Rating: ✅ EXCELLENT");
    } else if metrics.transactions_per_second >= 100.0 {
        println!("  • TPS Rating: ✅ GOOD");
    } else if metrics.transactions_per_second >= 10.0 {
        println!("  • TPS Rating: ⚠️ FAIR");
    } else {
        println!("  • TPS Rating: ❌ POOR");
    }
    
    if metrics.error_rate < 0.1 {
        println!("  • Error Rate: ✅ EXCELLENT");
    } else if metrics.error_rate < 1.0 {
        println!("  • Error Rate: ✅ GOOD");
    } else if metrics.error_rate < 5.0 {
        println!("  • Error Rate: ⚠️ FAIR");
    } else {
        println!("  • Error Rate: ❌ POOR");
    }
}

fn print_performance_recommendations() {
    println!("💡 Performance Optimization Recommendations:");
    println!("-------------------------------------------");
    println!("1. 🎯 Order Book Optimization:");
    println!("   • Implement efficient data structures (e.g., TreeMap for price levels)");
    println!("   • Use batch processing for order matching");
    println!("   • Add order book depth limits");
    
    println!("\n2. ⚡ Token System Optimization:");
    println!("   • Implement account caching for frequently accessed accounts");
    println!("   • Use atomic operations for balance updates");
    println!("   • Add transaction batching capabilities");
    
    println!("\n3. 🔗 Blockchain Optimization:");
    println!("   • Reduce mining difficulty for testing/development");
    println!("   • Implement transaction pooling and batch processing");
    println!("   • Consider using a more efficient consensus mechanism");
    println!("   • Add transaction compression");
    
    println!("\n4. 🚀 Overall System Optimization:");
    println!("   • Implement async processing for I/O operations");
    println!("   • Add database persistence layer");
    println!("   • Use connection pooling for network operations");
    println!("   • Implement horizontal scaling with sharding");
    
    println!("\n5. 📊 Monitoring & Observability:");
    println!("   • Add comprehensive metrics collection");
    println!("   • Implement real-time performance dashboards");
    println!("   • Set up alerting for performance degradation");
    println!("   • Add distributed tracing for complex operations");
    
    println!("\n6. 🔧 Hardware Recommendations:");
    println!("   • Use SSD storage for faster blockchain access");
    println!("   • Increase available RAM for in-memory operations");
    println!("   • Consider GPU acceleration for mining operations");
    println!("   • Implement load balancing across multiple nodes");
}

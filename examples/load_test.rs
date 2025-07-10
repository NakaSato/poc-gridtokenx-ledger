use std::time::Instant;
use ledger_core::{Blockchain, TokenSystem, energy_trading::{EnergyMarket, EnergyOrder, OrderType}};

// Import the performance benchmarking functionality
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use ledger_core::{
    energy_trading::create_energy_trade_transaction,
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

/// Load testing with concurrent transactions
pub fn load_test_concurrent_transactions(
    duration_seconds: u64,
    concurrent_users: usize,
    transactions_per_user_per_second: u64,
) -> PerformanceMetrics {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let token_system = Arc::new(Mutex::new(TokenSystem::new()));
    let energy_market = Arc::new(Mutex::new(EnergyMarket::new()));
    
    // Setup accounts
    {
        let mut ts = token_system.lock().unwrap();
        for i in 0..concurrent_users {
            let address = format!("user_{}", i);
            ts.create_user_account(address.clone()).unwrap();
            ts.mint_watt_tokens(&address, 10000.0).unwrap();
        }
    }
    
    let start_time = Instant::now();
    let mut handles = vec![];
    let metrics = Arc::new(Mutex::new(Vec::<Duration>::new()));
    let error_count = Arc::new(Mutex::new(0u64));
    let total_count = Arc::new(Mutex::new(0u64));
    
    // Spawn worker threads
    for user_id in 0..concurrent_users {
        let blockchain_clone = blockchain.clone();
        let _token_system_clone = token_system.clone();
        let energy_market_clone = energy_market.clone();
        let metrics_clone = metrics.clone();
        let error_count_clone = error_count.clone();
        let total_count_clone = total_count.clone();
        
        let handle = thread::spawn(move || {
            let user_address = format!("user_{}", user_id);
            
            let mut transaction_counter = 0u64;
            let user_start = Instant::now();
            
            while user_start.elapsed().as_secs() < duration_seconds {
                let tx_start = Instant::now();
                
                // Perform energy trade transaction
                let result = std::panic::catch_unwind(|| {
                    // Create energy order
                    let order = EnergyOrder::new(
                        user_address.clone(),
                        if transaction_counter % 2 == 0 { OrderType::Sell } else { OrderType::Buy },
                        10.0 + (transaction_counter as f64 % 20.0),
                        0.12 + (transaction_counter as f64 % 10.0) * 0.01,
                    );
                    
                    // Place order in market
                    {
                        let mut em = energy_market_clone.lock().unwrap();
                        em.place_order(order)?;
                        em.match_orders();
                    }
                    
                    // Create blockchain transactions for completed trades
                    {
                        let em = energy_market_clone.lock().unwrap();
                        let mut bc = blockchain_clone.lock().unwrap();
                        
                        for trade in &em.matched_trades {
                            let transaction = create_energy_trade_transaction(trade);
                            bc.create_transaction(transaction);
                        }
                        
                        // Mine block if enough pending transactions
                        if bc.pending_transactions.len() >= 50 {
                            bc.mine_pending_transactions(&format!("miner_{}", user_id));
                        }
                    }
                    
                    Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                });
                
                let latency = tx_start.elapsed();
                
                match result {
                    Ok(Ok(())) => {
                        metrics_clone.lock().unwrap().push(latency);
                    }
                    _ => {
                        *error_count_clone.lock().unwrap() += 1;
                    }
                }
                
                *total_count_clone.lock().unwrap() += 1;
                transaction_counter += 1;
                
                // Rate limiting
                let expected_duration = Duration::from_nanos(
                    1_000_000_000 / transactions_per_user_per_second
                );
                if latency < expected_duration {
                    thread::sleep(expected_duration - latency);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_duration = start_time.elapsed();
    let total_transactions = *total_count.lock().unwrap();
    let failed_transactions = *error_count.lock().unwrap();
    let successful_transactions = total_transactions - failed_transactions;
    
    // Calculate metrics
    let latencies = metrics.lock().unwrap();
    let mut sorted_latencies: Vec<Duration> = latencies.clone();
    sorted_latencies.sort();
    
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
        transactions_per_second: successful_transactions as f64 / total_duration.as_secs_f64(),
        error_rate: (failed_transactions as f64 / total_transactions as f64) * 100.0,
        avg_latency_ms: avg_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        total_transactions,
        failed_transactions,
        duration_seconds: total_duration.as_secs_f64(),
    }
}

/// Memory usage tracking
pub fn track_resource_utilization() -> ResourceUtilization {
    // Get current process info
    let pid = std::process::id();
    
    // Basic memory tracking (platform-specific implementation would be more accurate)
    let memory_usage_mb = get_memory_usage_mb(pid);
    let cpu_usage = get_cpu_usage_percent(pid);
    
    ResourceUtilization {
        cpu_usage_percent: cpu_usage,
        memory_usage_mb,
        peak_memory_mb: memory_usage_mb, // Simplified - in real implementation track peak
        network_throughput_mbps: 0.0, // Would need platform-specific network monitoring
    }
}

#[cfg(target_os = "linux")]
fn get_memory_usage_mb(pid: u32) -> f64 {
    use std::fs;
    
    if let Ok(status) = fs::read_to_string(format!("/proc/{}/status", pid)) {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<f64>() {
                        return kb / 1024.0; // Convert KB to MB
                    }
                }
            }
        }
    }
    0.0
}

#[cfg(not(target_os = "linux"))]
fn get_memory_usage_mb(_pid: u32) -> f64 {
    // Simplified cross-platform implementation
    // In production, use system-specific APIs
    0.0
}

#[cfg(target_os = "linux")]
fn get_cpu_usage_percent(pid: u32) -> f64 {
    // Simplified CPU usage calculation
    // In production, implement proper CPU monitoring
    0.0
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_usage_percent(_pid: u32) -> f64 {
    0.0
}

/// Comprehensive performance testing suite
fn main() {
    println!("🚀 Energy Trading Blockchain Performance Testing Suite");
    println!("====================================================\n");

    // Test Configuration
    let test_scenarios = vec![
        (10, 5, 10),   // 10 users, 5 TPS each, 10 seconds
        (50, 10, 15),  // 50 users, 10 TPS each, 15 seconds
        (100, 20, 20), // 100 users, 20 TPS each, 20 seconds
        (200, 50, 30), // 200 users, 50 TPS each, 30 seconds
    ];

    let mut all_results = Vec::new();

    for (users, tps_per_user, duration) in test_scenarios {
        println!("🧪 Test Scenario: {} users, {} TPS per user, {} seconds", users, tps_per_user, duration);
        println!("Expected Total TPS: {}", users * tps_per_user);
        
        let start_time = Instant::now();
        
        // Track resource utilization before test
        let initial_resources = track_resource_utilization();
        
        // Run load test
        let metrics = load_test_concurrent_transactions(duration as u64, users, tps_per_user as u64);
        
        // Track resource utilization after test
        let final_resources = track_resource_utilization();
        
        let test_duration = start_time.elapsed();
        
        // Display results
        print_performance_results(&metrics, &initial_resources, &final_resources, users, tps_per_user as u64);
        
        all_results.push((users, tps_per_user as u64, metrics, test_duration));
        
        println!("---");
        
        // Cool down between tests
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // Summary report
    print_summary_report(&all_results);
    
    // Run single-threaded baseline test
    println!("\n🔧 Running baseline single-threaded test...");
    run_baseline_test();
}

fn print_performance_results(
    metrics: &PerformanceMetrics,
    initial_resources: &ResourceUtilization,
    final_resources: &ResourceUtilization,
    users: usize,
    expected_tps_per_user: u64,
) {
    println!("📊 Results:");
    println!("  • Actual TPS: {:.2}", metrics.transactions_per_second);
    println!("  • Expected TPS: {}", users as u64 * expected_tps_per_user);
    println!("  • Efficiency: {:.1}%", 
        (metrics.transactions_per_second / (users as f64 * expected_tps_per_user as f64)) * 100.0);
    println!("  • Error Rate: {:.3}%", metrics.error_rate);
    println!("  • Total Transactions: {}", metrics.total_transactions);
    println!("  • Failed Transactions: {}", metrics.failed_transactions);
    
    println!("\n⏱️  Latency Metrics:");
    println!("  • Average Latency: {:.2} ms", metrics.avg_latency_ms);
    println!("  • 95th Percentile: {:.2} ms", metrics.p95_latency_ms);
    println!("  • 99th Percentile: {:.2} ms", metrics.p99_latency_ms);
    
    println!("\n💾 Resource Utilization:");
    println!("  • Memory Usage: {:.1} MB → {:.1} MB", 
        initial_resources.memory_usage_mb, final_resources.memory_usage_mb);
    println!("  • Peak Memory: {:.1} MB", final_resources.peak_memory_mb);
    println!("  • CPU Usage: {:.1}%", final_resources.cpu_usage_percent);
    
    // Performance classification
    classify_performance(metrics, users, expected_tps_per_user);
    println!();
}

fn classify_performance(metrics: &PerformanceMetrics, users: usize, expected_tps_per_user: u64) {
    let expected_total_tps = users as f64 * expected_tps_per_user as f64;
    let efficiency = (metrics.transactions_per_second / expected_total_tps) * 100.0;
    
    println!("\n🎯 Performance Classification:");
    
    // TPS Performance
    if efficiency >= 90.0 {
        println!("  • TPS: ✅ EXCELLENT ({:.1}% efficiency)", efficiency);
    } else if efficiency >= 70.0 {
        println!("  • TPS: ✅ GOOD ({:.1}% efficiency)", efficiency);
    } else if efficiency >= 50.0 {
        println!("  • TPS: ⚠️  FAIR ({:.1}% efficiency)", efficiency);
    } else {
        println!("  • TPS: ❌ POOR ({:.1}% efficiency)", efficiency);
    }
    
    // Error Rate Performance
    if metrics.error_rate < 0.1 {
        println!("  • Error Rate: ✅ EXCELLENT ({:.3}%)", metrics.error_rate);
    } else if metrics.error_rate < 1.0 {
        println!("  • Error Rate: ✅ GOOD ({:.3}%)", metrics.error_rate);
    } else if metrics.error_rate < 5.0 {
        println!("  • Error Rate: ⚠️  FAIR ({:.3}%)", metrics.error_rate);
    } else {
        println!("  • Error Rate: ❌ POOR ({:.3}%)", metrics.error_rate);
    }
    
    // Latency Performance
    if metrics.p95_latency_ms < 100.0 {
        println!("  • Latency: ✅ EXCELLENT (P95: {:.1}ms)", metrics.p95_latency_ms);
    } else if metrics.p95_latency_ms < 500.0 {
        println!("  • Latency: ✅ GOOD (P95: {:.1}ms)", metrics.p95_latency_ms);
    } else if metrics.p95_latency_ms < 1000.0 {
        println!("  • Latency: ⚠️  FAIR (P95: {:.1}ms)", metrics.p95_latency_ms);
    } else {
        println!("  • Latency: ❌ POOR (P95: {:.1}ms)", metrics.p95_latency_ms);
    }
}

fn print_summary_report(results: &[(usize, u64, PerformanceMetrics, std::time::Duration)]) {
    println!("\n📈 PERFORMANCE SUMMARY REPORT");
    println!("=============================");
    
    println!("\n| Users | Expected TPS | Actual TPS | Efficiency | Error Rate | P95 Latency |");
    println!("|-------|-------------|------------|------------|------------|-------------|");
    
    let mut max_tps: f64 = 0.0;
    let mut min_error_rate: f64 = 100.0;
    let mut best_efficiency: f64 = 0.0;
    
    for &(users, tps_per_user, ref metrics, _) in results {
        let expected_tps = users as f64 * tps_per_user as f64;
        let efficiency = (metrics.transactions_per_second / expected_tps) * 100.0;
        
        max_tps = max_tps.max(metrics.transactions_per_second);
        min_error_rate = min_error_rate.min(metrics.error_rate);
        best_efficiency = best_efficiency.max(efficiency);
        
        println!("| {:5} | {:11.0} | {:10.1} | {:9.1}% | {:9.3}% | {:8.1} ms |",
            users, expected_tps, metrics.transactions_per_second, 
            efficiency, metrics.error_rate, metrics.p95_latency_ms);
    }
    
    println!("\n🏆 Key Performance Indicators:");
    println!("  • Maximum TPS Achieved: {:.1}", max_tps);
    println!("  • Minimum Error Rate: {:.3}%", min_error_rate);
    println!("  • Best Efficiency: {:.1}%", best_efficiency);
    
    // Recommendations
    println!("\n💡 Recommendations:");
    if max_tps < 100.0 {
        println!("  • Consider optimizing transaction processing pipeline");
        println!("  • Implement batch processing for higher throughput");
    }
    if min_error_rate > 1.0 {
        println!("  • Investigate error sources to improve reliability");
        println!("  • Implement better error handling and retry mechanisms");
    }
    if best_efficiency < 80.0 {
        println!("  • Optimize resource utilization and reduce bottlenecks");
        println!("  • Consider parallel processing improvements");
    }
}

fn run_baseline_test() {
    let start = Instant::now();
    
    let mut blockchain = Blockchain::new();
    let mut token_system = TokenSystem::new();
    let mut energy_market = EnergyMarket::new();
    
    // Setup
    for i in 0..100 {
        let address = format!("user_{}", i);
        token_system.create_user_account(address.clone()).unwrap();
        token_system.mint_watt_tokens(&address, 1000.0).unwrap();
    }
    
    // Execute transactions sequentially
    let mut successful_transactions = 0;
    for i in 0..1000 {
        let seller = format!("user_{}", i % 100);
        let buyer = format!("user_{}", (i + 1) % 100);
        
        // Create orders
        let sell_order = EnergyOrder::new(seller.clone(), OrderType::Sell, 10.0, 0.15);
        let buy_order = EnergyOrder::new(buyer.clone(), OrderType::Buy, 10.0, 0.15);
        
        if energy_market.place_order(sell_order).is_ok() && 
           energy_market.place_order(buy_order).is_ok() {
            energy_market.match_orders();
            successful_transactions += 1;
        }
        
        // Mine blocks periodically
        if i % 50 == 0 {
            blockchain.mine_pending_transactions("miner");
        }
    }
    
    let duration = start.elapsed();
    let baseline_tps = successful_transactions as f64 / duration.as_secs_f64();
    
    println!("📏 Baseline Performance (Single-threaded):");
    println!("  • Transactions: {}", successful_transactions);
    println!("  • Duration: {:.2}s", duration.as_secs_f64());
    println!("  • Baseline TPS: {:.1}", baseline_tps);
    println!("  • This represents the theoretical maximum for comparison");
}

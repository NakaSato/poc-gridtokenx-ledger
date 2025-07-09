use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use ledger_core::{
    Blockchain, TokenSystem, 
    energy_trading::{EnergyMarket, EnergyOrder, OrderType, create_energy_trade_transaction},
    block::{Transaction, TransactionType}
};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

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

/// Benchmark transaction throughput
fn benchmark_transaction_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_throughput");
    
    // Test different batch sizes
    for batch_size in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("energy_trades", batch_size),
            batch_size,
            |b, &batch_size| {
                b.iter(|| {
                    let mut blockchain = Blockchain::new();
                    let mut token_system = TokenSystem::new();
                    let mut energy_market = EnergyMarket::new();
                    
                    // Setup accounts
                    for i in 0..batch_size {
                        let address = format!("user_{}", i);
                        token_system.create_user_account(address.clone()).unwrap();
                        token_system.mint_watt_tokens(&address, 1000.0).unwrap();
                    }
                    
                    // Create and process trades
                    for i in 0..batch_size {
                        let seller = format!("user_{}", i);
                        let buyer = format!("user_{}", (i + 1) % batch_size);
                        
                        // Create sell order
                        let sell_order = EnergyOrder::new(
                            seller.clone(),
                            OrderType::Sell,
                            10.0, // 10 kWh
                            0.15, // $0.15/kWh
                        );
                        energy_market.place_order(sell_order).unwrap();
                        
                        // Create buy order
                        let buy_order = EnergyOrder::new(
                            buyer.clone(),
                            OrderType::Buy,
                            10.0, // 10 kWh
                            0.15, // $0.15/kWh
                        );
                        energy_market.place_order(buy_order).unwrap();
                        
                        // Process trades
                        energy_market.match_orders(&mut token_system);
                        
                        // Create blockchain transactions
                        for trade in &energy_market.matched_trades {
                            let transaction = create_energy_trade_transaction(trade);
                            blockchain.create_transaction(transaction);
                        }
                        
                        // Mine block
                        if blockchain.pending_transactions.len() >= 10 {
                            blockchain.mine_pending_transactions("miner");
                        }
                    }
                    
                    // Mine remaining transactions
                    if !blockchain.pending_transactions.is_empty() {
                        blockchain.mine_pending_transactions("miner");
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark order book operations
fn benchmark_order_book_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_book_operations");
    
    for order_count in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*order_count as u64));
        group.bench_with_input(
            BenchmarkId::new("order_placement", order_count),
            order_count,
            |b, &order_count| {
                b.iter(|| {
                    let mut energy_market = EnergyMarket::new();
                    
                    // Place orders
                    for i in 0..order_count {
                        let order = EnergyOrder::new(
                            format!("user_{}", i),
                            if i % 2 == 0 { OrderType::Buy } else { OrderType::Sell },
                            (10.0 + (i as f64 % 50.0)), // Varying energy amounts
                            0.10 + (i as f64 % 100.0) * 0.001, // Varying prices
                        );
                        energy_market.place_order(order).unwrap();
                    }
                    
                    energy_market
                });
            },
        );
    }
    group.finish();
}

/// Benchmark blockchain validation
fn benchmark_blockchain_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("blockchain_validation");
    
    for block_count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("chain_validation", block_count),
            block_count,
            |b, &block_count| {
                b.iter_setup(
                    || {
                        let mut blockchain = Blockchain::new();
                        let mut token_system = TokenSystem::new();
                        
                        // Create test accounts
                        for i in 0..10 {
                            let address = format!("user_{}", i);
                            token_system.create_user_account(address.clone()).unwrap();
                            token_system.mint_watt_tokens(&address, 1000.0).unwrap();
                        }
                        
                        // Create blocks with transactions
                        for block_idx in 0..*block_count {
                            for tx_idx in 0..10 {
                                let from = format!("user_{}", tx_idx % 10);
                                let to = format!("user_{}", (tx_idx + 1) % 10);
                                
                                let transaction = crate::block::Transaction::new(
                                    from,
                                    to,
                                    10.0, // energy amount
                                    0.15, // price
                                    crate::block::TransactionType::EnergyTrade,
                                );
                                blockchain.create_transaction(transaction);
                            }
                            blockchain.mine_pending_transactions(&format!("miner_{}", block_idx));
                        }
                        
                        blockchain
                    },
                    |blockchain| {
                        blockchain.is_chain_valid()
                    },
                );
            },
        );
    }
    group.finish();
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
        let token_system_clone = token_system.clone();
        let energy_market_clone = energy_market.clone();
        let metrics_clone = metrics.clone();
        let error_count_clone = error_count.clone();
        let total_count_clone = total_count.clone();
        
        let handle = thread::spawn(move || {
            let user_address = format!("user_{}", user_id);
            let target_address = format!("user_{}", (user_id + 1) % concurrent_users);
            
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
                        em.match_orders(&mut token_system_clone.lock().unwrap());
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

criterion_group!(
    benches,
    benchmark_transaction_throughput,
    benchmark_order_book_operations,
    benchmark_blockchain_validation
);
criterion_main!(benches);

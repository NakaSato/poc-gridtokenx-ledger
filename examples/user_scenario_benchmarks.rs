use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

// Import from the current crate
use ledger_core::energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer};
use ledger_core::token_system::TokenSystem;
use ledger_core::blockchain::Blockchain;

/// Comprehensive user scenario benchmarking system
/// Tests the most frequent and critical user actions:
/// 1. High-frequency reads: Market data, order book, wallet balances
/// 2. Medium-frequency writes: Order placement/cancellation  
/// 3. Low-frequency, high-impact: Trade settlement and blockchain operations

#[derive(Debug, Clone)]
struct UserScenarioConfig {
    pub num_users: usize,
    pub test_duration_seconds: u64,
    pub read_operations_per_second: u64,
    pub write_operations_per_second: u64,
    pub settlement_operations_per_second: u64,
}

#[derive(Debug, Clone)]
struct ScenarioMetrics {
    pub read_operations: u64,
    pub write_operations: u64,
    pub settlement_operations: u64,
    pub read_errors: u64,
    pub write_errors: u64,
    pub settlement_errors: u64,
    pub read_latency_ms: Vec<f64>,
    pub write_latency_ms: Vec<f64>,
    pub settlement_latency_ms: Vec<f64>,
    pub start_time: Instant,
    pub end_time: Instant,
}

impl ScenarioMetrics {
    fn new() -> Self {
        ScenarioMetrics {
            read_operations: 0,
            write_operations: 0,
            settlement_operations: 0,
            read_errors: 0,
            write_errors: 0,
            settlement_errors: 0,
            read_latency_ms: Vec::new(),
            write_latency_ms: Vec::new(),
            settlement_latency_ms: Vec::new(),
            start_time: Instant::now(),
            end_time: Instant::now(),
        }
    }

    fn get_read_tps(&self) -> f64 {
        let duration = self.end_time.duration_since(self.start_time).as_secs_f64();
        self.read_operations as f64 / duration
    }

    fn get_write_tps(&self) -> f64 {
        let duration = self.end_time.duration_since(self.start_time).as_secs_f64();
        self.write_operations as f64 / duration
    }

    fn get_settlement_tps(&self) -> f64 {
        let duration = self.end_time.duration_since(self.start_time).as_secs_f64();
        self.settlement_operations as f64 / duration
    }

    fn get_read_error_rate(&self) -> f64 {
        if self.read_operations == 0 { 0.0 } else {
            self.read_errors as f64 / self.read_operations as f64 * 100.0
        }
    }

    fn get_write_error_rate(&self) -> f64 {
        if self.write_operations == 0 { 0.0 } else {
            self.write_errors as f64 / self.write_operations as f64 * 100.0
        }
    }

    fn get_settlement_error_rate(&self) -> f64 {
        if self.settlement_operations == 0 { 0.0 } else {
            self.settlement_errors as f64 / self.settlement_operations as f64 * 100.0
        }
    }

    fn get_avg_read_latency(&self) -> f64 {
        if self.read_latency_ms.is_empty() { 0.0 } else {
            self.read_latency_ms.iter().sum::<f64>() / self.read_latency_ms.len() as f64
        }
    }

    fn get_avg_write_latency(&self) -> f64 {
        if self.write_latency_ms.is_empty() { 0.0 } else {
            self.write_latency_ms.iter().sum::<f64>() / self.write_latency_ms.len() as f64
        }
    }

    fn get_avg_settlement_latency(&self) -> f64 {
        if self.settlement_latency_ms.is_empty() { 0.0 } else {
            self.settlement_latency_ms.iter().sum::<f64>() / self.settlement_latency_ms.len() as f64
        }
    }

    fn get_p95_read_latency(&self) -> f64 {
        if self.read_latency_ms.is_empty() { 0.0 } else {
            let mut sorted = self.read_latency_ms.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = (sorted.len() as f64 * 0.95) as usize;
            sorted[index.min(sorted.len() - 1)]
        }
    }

    fn get_p95_write_latency(&self) -> f64 {
        if self.write_latency_ms.is_empty() { 0.0 } else {
            let mut sorted = self.write_latency_ms.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = (sorted.len() as f64 * 0.95) as usize;
            sorted[index.min(sorted.len() - 1)]
        }
    }

    fn get_p95_settlement_latency(&self) -> f64 {
        if self.settlement_latency_ms.is_empty() { 0.0 } else {
            let mut sorted = self.settlement_latency_ms.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = (sorted.len() as f64 * 0.95) as usize;
            sorted[index.min(sorted.len() - 1)]
        }
    }
}

struct UserScenarioBenchmark {
    config: UserScenarioConfig,
    energy_market: Arc<Mutex<EnergyMarket>>,
    token_system: Arc<Mutex<TokenSystem>>,
    blockchain: Arc<Mutex<Blockchain>>,
    users: Vec<Prosumer>,
    metrics: Arc<Mutex<ScenarioMetrics>>,
}

impl UserScenarioBenchmark {
    fn new(config: UserScenarioConfig) -> Self {
        let mut token_system = TokenSystem::new();
        let mut users = Vec::new();
        
        // Create test users
        for i in 0..config.num_users {
            let address = format!("user_{}", i);
            let name = format!("User {}", i);
            let prosumer = Prosumer::new(address.clone(), name);
            users.push(prosumer);
            
            // Create user account in token system
            token_system.create_user_account(address.clone()).unwrap();
            
            // Give users some initial tokens for testing
            token_system.mint_watt_tokens(&address, 10000.0).unwrap();
            
            // Add some manual balance for grid tokens (since mint_grid_tokens doesn't exist)
            if let Some(balance) = token_system.user_balances.get_mut(&address) {
                balance.grid_balance = 1000.0;
            }
        }

        UserScenarioBenchmark {
            config,
            energy_market: Arc::new(Mutex::new(EnergyMarket::new())),
            token_system: Arc::new(Mutex::new(token_system)),
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
            users,
            metrics: Arc::new(Mutex::new(ScenarioMetrics::new())),
        }
    }

    /// High-frequency read operations: Market data, order book, wallet balances
    fn simulate_read_operations(&self, user_id: usize, operations_per_second: u64) {
        let market = Arc::clone(&self.energy_market);
        let token_system = Arc::clone(&self.token_system);
        let metrics = Arc::clone(&self.metrics);
        let user_address = self.users[user_id].address.clone();
        
        let sleep_duration = Duration::from_nanos(1_000_000_000 / operations_per_second);
        
        loop {
            // Market price lookup
            let start = Instant::now();
            {
                let market_lock = market.lock().unwrap();
                let _price = market_lock.get_market_price();
            }
            let latency = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            {
                let mut metrics_lock = metrics.lock().unwrap();
                metrics_lock.read_operations += 1;
                metrics_lock.read_latency_ms.push(latency);
            }

            // Order book lookup
            let start = Instant::now();
            {
                let market_lock = market.lock().unwrap();
                let _order_book = market_lock.get_order_book();
            }
            let latency = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            {
                let mut metrics_lock = metrics.lock().unwrap();
                metrics_lock.read_operations += 1;
                metrics_lock.read_latency_ms.push(latency);
            }

            // Wallet balance lookup
            let start = Instant::now();
            {
                let token_system_lock = token_system.lock().unwrap();
                let _balance = token_system_lock.get_user_balance(&user_address);
            }
            let latency = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            {
                let mut metrics_lock = metrics.lock().unwrap();
                metrics_lock.read_operations += 1;
                metrics_lock.read_latency_ms.push(latency);
            }

            thread::sleep(sleep_duration);
        }
    }

    /// Medium-frequency write operations: Order placement and cancellation
    fn simulate_write_operations(&self, user_id: usize, operations_per_second: u64) {
        let market = Arc::clone(&self.energy_market);
        let metrics = Arc::clone(&self.metrics);
        let user_address = self.users[user_id].address.clone();
        let mut counter = 0u64;
        
        let sleep_duration = Duration::from_nanos(1_000_000_000 / operations_per_second);
        
        loop {
            // Place order
            let start = Instant::now();
            let order_type = if counter % 2 == 0 { OrderType::Buy } else { OrderType::Sell };
            let energy_amount = 10.0 + (counter as f64 % 90.0); // 10-100 kWh
            let price = 0.5 + (counter as f64 % 10.0) / 10.0; // 0.5-1.5 price
            
            let order = EnergyOrder::new(
                user_address.clone(),
                order_type,
                energy_amount,
                price,
            );
            
            let result = {
                let mut market_lock = market.lock().unwrap();
                market_lock.place_order(order)
            };
            
            let latency = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            {
                let mut metrics_lock = metrics.lock().unwrap();
                metrics_lock.write_operations += 1;
                metrics_lock.write_latency_ms.push(latency);
                if result.is_err() {
                    metrics_lock.write_errors += 1;
                }
            }

            counter += 1;
            thread::sleep(sleep_duration);
        }
    }

    /// Low-frequency, high-impact operations: Trade settlement and blockchain operations
    fn simulate_settlement_operations(&self, operations_per_second: u64) {
        let market = Arc::clone(&self.energy_market);
        let blockchain = Arc::clone(&self.blockchain);
        let token_system = Arc::clone(&self.token_system);
        let metrics = Arc::clone(&self.metrics);
        
        let sleep_duration = Duration::from_nanos(1_000_000_000 / operations_per_second);
        
        loop {
            let start = Instant::now();
            
            // Get matched trades and settle them
            let trades = {
                let mut market_lock = market.lock().unwrap();
                market_lock.match_orders()
            };
            
            let has_trades = !trades.is_empty();
            
            for trade in trades {
                // Create blockchain transaction for the trade
                let trade_transaction = ledger_core::energy_trading::create_energy_trade_transaction(&trade);
                let grid_fee_transaction = ledger_core::energy_trading::create_grid_fee_transaction(&trade, "grid_operator");
                
                // Add transactions to blockchain
                {
                    let mut blockchain_lock = blockchain.lock().unwrap();
                    blockchain_lock.create_transaction(trade_transaction);
                    blockchain_lock.create_transaction(grid_fee_transaction);
                }
                
                // Update token balances
                {
                    let mut token_system_lock = token_system.lock().unwrap();
                    // Buyer pays total cost
                    let _ = token_system_lock.transfer_watt_tokens(
                        &trade.buyer,
                        &trade.seller,
                        trade.total_cost,
                    );
                    
                    // Grid operator receives grid fee
                    let _ = token_system_lock.transfer_watt_tokens(
                        &trade.buyer,
                        "grid_operator",
                        trade.grid_fee,
                    );
                }
            }
            
            // Mine a block to finalize transactions
            if has_trades {
                let mut blockchain_lock = blockchain.lock().unwrap();
                blockchain_lock.mine_pending_transactions("miner");
            }
            
            let latency = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            {
                let mut metrics_lock = metrics.lock().unwrap();
                metrics_lock.settlement_operations += 1;
                metrics_lock.settlement_latency_ms.push(latency);
            }

            thread::sleep(sleep_duration);
        }
    }

    fn run_benchmark(&mut self) -> ScenarioMetrics {
        println!("Starting user scenario benchmark...");
        println!("Configuration:");
        println!("  Users: {}", self.config.num_users);
        println!("  Duration: {}s", self.config.test_duration_seconds);
        println!("  Read ops/s: {}", self.config.read_operations_per_second);
        println!("  Write ops/s: {}", self.config.write_operations_per_second);
        println!("  Settlement ops/s: {}", self.config.settlement_operations_per_second);

        self.metrics.lock().unwrap().start_time = Instant::now();
        
        let mut handles = Vec::new();
        
        // Start read operation threads (one per user)
        for user_id in 0..self.config.num_users {
            let benchmark = self.clone_for_thread();
            let handle = thread::spawn(move || {
                benchmark.simulate_read_operations(user_id, benchmark.config.read_operations_per_second);
            });
            handles.push(handle);
        }
        
        // Start write operation threads (one per user)
        for user_id in 0..self.config.num_users {
            let benchmark = self.clone_for_thread();
            let handle = thread::spawn(move || {
                benchmark.simulate_write_operations(user_id, benchmark.config.write_operations_per_second);
            });
            handles.push(handle);
        }
        
        // Start settlement operation thread
        let benchmark = self.clone_for_thread();
        let handle = thread::spawn(move || {
            benchmark.simulate_settlement_operations(benchmark.config.settlement_operations_per_second);
        });
        handles.push(handle);
        
        // Run for configured duration
        thread::sleep(Duration::from_secs(self.config.test_duration_seconds));
        
        // Stop all threads by dropping handles (threads will terminate when main exits)
        drop(handles);
        
        let mut metrics = self.metrics.lock().unwrap();
        metrics.end_time = Instant::now();
        metrics.clone()
    }

    fn clone_for_thread(&self) -> Self {
        UserScenarioBenchmark {
            config: self.config.clone(),
            energy_market: Arc::clone(&self.energy_market),
            token_system: Arc::clone(&self.token_system),
            blockchain: Arc::clone(&self.blockchain),
            users: self.users.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

fn print_scenario_results(metrics: &ScenarioMetrics) {
    println!("\n=== USER SCENARIO BENCHMARK RESULTS ===");
    println!("Test Duration: {:.2}s", metrics.end_time.duration_since(metrics.start_time).as_secs_f64());
    
    println!("\n--- READ OPERATIONS (Market Data, Order Book, Wallet Balance) ---");
    println!("Total Read Operations: {}", metrics.read_operations);
    println!("Read TPS: {:.2}", metrics.get_read_tps());
    println!("Read Error Rate: {:.2}%", metrics.get_read_error_rate());
    println!("Average Read Latency: {:.2}ms", metrics.get_avg_read_latency());
    println!("P95 Read Latency: {:.2}ms", metrics.get_p95_read_latency());
    
    println!("\n--- WRITE OPERATIONS (Order Placement/Cancellation) ---");
    println!("Total Write Operations: {}", metrics.write_operations);
    println!("Write TPS: {:.2}", metrics.get_write_tps());
    println!("Write Error Rate: {:.2}%", metrics.get_write_error_rate());
    println!("Average Write Latency: {:.2}ms", metrics.get_avg_write_latency());
    println!("P95 Write Latency: {:.2}ms", metrics.get_p95_write_latency());
    
    println!("\n--- SETTLEMENT OPERATIONS (Trade Settlement + Blockchain) ---");
    println!("Total Settlement Operations: {}", metrics.settlement_operations);
    println!("Settlement TPS: {:.2}", metrics.get_settlement_tps());
    println!("Settlement Error Rate: {:.2}%", metrics.get_settlement_error_rate());
    println!("Average Settlement Latency: {:.2}ms", metrics.get_avg_settlement_latency());
    println!("P95 Settlement Latency: {:.2}ms", metrics.get_p95_settlement_latency());
    
    println!("\n--- OVERALL PERFORMANCE ---");
    let total_ops = metrics.read_operations + metrics.write_operations + metrics.settlement_operations;
    let total_tps = metrics.get_read_tps() + metrics.get_write_tps() + metrics.get_settlement_tps();
    println!("Total Operations: {}", total_ops);
    println!("Total TPS: {:.2}", total_tps);
    
    let total_errors = metrics.read_errors + metrics.write_errors + metrics.settlement_errors;
    let overall_error_rate = if total_ops == 0 { 0.0 } else { total_errors as f64 / total_ops as f64 * 100.0 };
    println!("Overall Error Rate: {:.2}%", overall_error_rate);
}

fn main() {
    println!("Energy Trading Ledger - User Scenario Benchmarks");
    println!("=================================================");
    
    // Test different user scenarios
    let scenarios = vec![
        ("Light Load", UserScenarioConfig {
            num_users: 10,
            test_duration_seconds: 30,
            read_operations_per_second: 10,
            write_operations_per_second: 2,
            settlement_operations_per_second: 1,
        }),
        ("Medium Load", UserScenarioConfig {
            num_users: 50,
            test_duration_seconds: 30,
            read_operations_per_second: 50,
            write_operations_per_second: 10,
            settlement_operations_per_second: 5,
        }),
        ("Heavy Load", UserScenarioConfig {
            num_users: 100,
            test_duration_seconds: 30,
            read_operations_per_second: 100,
            write_operations_per_second: 20,
            settlement_operations_per_second: 10,
        }),
    ];
    
    for (scenario_name, config) in scenarios {
        println!("\n\n🧪 Running {} Scenario", scenario_name);
        println!("==================================================");
        
        let mut benchmark = UserScenarioBenchmark::new(config);
        let metrics = benchmark.run_benchmark();
        print_scenario_results(&metrics);
    }
    
    println!("\n\n✅ All user scenario benchmarks completed!");
}

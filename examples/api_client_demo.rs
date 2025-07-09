// API Client Example for Energy Trading Ledger
// This demonstrates how to interact with the API endpoints

use serde_json::json;

const BASE_URL: &str = "http://localhost:3000";

// Example API client functions
pub async fn create_prosumer(address: &str, name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/energy/prosumers", BASE_URL))
        .json(&json!({
            "address": address,
            "name": name
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn create_token_account(address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/tokens/accounts", BASE_URL))
        .json(&json!({
            "address": address
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn update_energy_generation(address: &str, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/energy/generation", BASE_URL))
        .json(&json!({
            "address": address,
            "amount": amount
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn create_sell_order(trader_address: &str, energy_amount: f64, price_per_kwh: f64) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/energy/orders", BASE_URL))
        .json(&json!({
            "trader_address": trader_address,
            "order_type": "sell",
            "energy_amount": energy_amount,
            "price_per_kwh": price_per_kwh
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn create_buy_order(trader_address: &str, energy_amount: f64, price_per_kwh: f64) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/energy/orders", BASE_URL))
        .json(&json!({
            "trader_address": trader_address,
            "order_type": "buy",
            "energy_amount": energy_amount,
            "price_per_kwh": price_per_kwh
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn get_market_statistics() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/api/energy/statistics", BASE_URL))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn get_blockchain_info() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/api/blockchain/info", BASE_URL))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

pub async fn mine_block(miner_address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/blockchain/mine", BASE_URL))
        .json(&json!({
            "miner_address": miner_address
        }))
        .send()
        .await?;
    
    let text = response.text().await?;
    Ok(text)
}

// Example usage function
pub async fn run_api_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Energy Trading Ledger API Demo");
    println!("===================================");
    
    // Check if the API server is running
    match reqwest::get(&format!("{}/health", BASE_URL)).await {
        Ok(response) => {
            println!("✅ API server is running!");
            println!("Response: {}", response.text().await?);
        }
        Err(e) => {
            println!("❌ API server is not running. Please start it first with: cargo run --bin api-server");
            println!("Error: {}", e);
            return Ok(());
        }
    }
    
    println!("\n1. Creating prosumers...");
    let alice_result = create_prosumer("alice_address", "Alice's Solar Farm").await?;
    println!("Alice: {}", alice_result);
    
    let bob_result = create_prosumer("bob_address", "Bob's Wind Turbine").await?;
    println!("Bob: {}", bob_result);
    
    let charlie_result = create_prosumer("charlie_address", "Charlie's Home").await?;
    println!("Charlie: {}", charlie_result);
    
    println!("\n2. Creating token accounts...");
    let alice_token = create_token_account("alice_address").await?;
    println!("Alice token account: {}", alice_token);
    
    let bob_token = create_token_account("bob_address").await?;
    println!("Bob token account: {}", bob_token);
    
    let charlie_token = create_token_account("charlie_address").await?;
    println!("Charlie token account: {}", charlie_token);
    
    println!("\n3. Updating energy generation...");
    let alice_gen = update_energy_generation("alice_address", 50.0).await?;
    println!("Alice generated 50 kWh: {}", alice_gen);
    
    let bob_gen = update_energy_generation("bob_address", 30.0).await?;
    println!("Bob generated 30 kWh: {}", bob_gen);
    
    println!("\n4. Creating energy orders...");
    let alice_sell = create_sell_order("alice_address", 25.0, 0.15).await?;
    println!("Alice sell order: {}", alice_sell);
    
    let bob_sell = create_sell_order("bob_address", 20.0, 0.12).await?;
    println!("Bob sell order: {}", bob_sell);
    
    let charlie_buy = create_buy_order("charlie_address", 30.0, 0.20).await?;
    println!("Charlie buy order: {}", charlie_buy);
    
    println!("\n5. Getting market statistics...");
    let stats = get_market_statistics().await?;
    println!("Market stats: {}", stats);
    
    println!("\n6. Getting blockchain info...");
    let blockchain_info = get_blockchain_info().await?;
    println!("Blockchain info: {}", blockchain_info);
    
    println!("\n7. Mining a block...");
    let mine_result = mine_block("miner_address").await?;
    println!("Mining result: {}", mine_result);
    
    println!("\n✅ API Demo completed successfully!");
    
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_api_demo().await {
        eprintln!("Error running API demo: {}", e);
    }
}

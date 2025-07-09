mod api;
mod block;
mod blockchain;
mod energy_trading;
mod token_system;
mod smart_contracts;
mod tests;

use api::server::start_server;

#[tokio::main]
async fn main() {
    println!("🌟 Energy Trading Ledger - API Server 🌟");
    println!("==========================================");
    
    // Start the API server on port 3000
    start_server(3000).await;
}

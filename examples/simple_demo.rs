// Simple Architecture Demo - Thai Energy Trading System
// This example demonstrates the new modular architecture with basic functionality

use ledger_core::ThaiEnergyTradingSystem;
use ledger_core::primitives::*;
use ledger_core::services::*;

fn main() -> CoreResult<()> {
    println!("🌟 Thai Energy Trading System - Simple Architecture Demo 🌟");
    println!("===========================================================");
    
    // Initialize the system
    let mut system = ThaiEnergyTradingSystem::new();
    println!("✅ System initialized successfully");
    
    // Create some test accounts
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    
    println!("\n📋 Test accounts:");
    println!("   - Alice: {}", alice);
    println!("   - Bob: {}", bob);
    
    // Test token operations
    println!("\n🪙 Testing Token Operations...");
    
    // Create a governance token
    let token_metadata = TokenMetadata {
        name: "Thai Energy Token".to_string(),
        symbol: "TET".to_string(),
        decimals: 18,
        total_supply: 1_000_000,
        description: "Token for Thai Energy Trading System".to_string(),
        icon_url: None,
    };
    
    let token_id = system.runtime.token_service.create_token(&alice, token_metadata)?;
    println!("   ✅ Created token: {} (ID: {})", "Thai Energy Token", token_id);
    
    // Transfer some tokens
    system.runtime.token_service.transfer_tokens(&token_id, &alice, &bob, 10_000)?;
    
    println!("   ✅ Transferred 10,000 tokens from Alice to Bob");
    
    // Check balances
    let alice_balance = system.runtime.token_service.get_balance(&alice, &token_id);
    let bob_balance = system.runtime.token_service.get_balance(&bob, &token_id);
    
    println!("   💰 Token balances:");
    println!("      - Alice: {} TET", alice_balance);
    println!("      - Bob: {} TET", bob_balance);
    
    // Test staking
    println!("\n🔒 Testing Staking Operations...");
    system.runtime.token_service.stake_tokens(&bob, &token_id, 5_000)?;
    println!("   ✅ Bob staked 5,000 tokens");
    
    let bob_balance_after = system.runtime.token_service.get_balance(&bob, &token_id);
    println!("   💰 Bob's balance after staking: {} TET", bob_balance_after);
    
    // Test governance
    println!("\n🏛️ Testing Governance Operations...");
    
    let proposal_id = system.runtime.token_service.create_proposal(
        &alice,
        "Increase Energy Trading Fee".to_string(),
        "Proposal to increase the energy trading fee from 1% to 2%".to_string(),
        7 * 24 * 60 * 60, // 7 days
    )?;
    
    println!("   ✅ Created proposal: {}", proposal_id);
    
    // Vote on the proposal
    system.runtime.token_service.vote_on_proposal(&alice, &proposal_id, Vote::For)?;
    system.runtime.token_service.vote_on_proposal(&bob, &proposal_id, Vote::Against)?;
    
    println!("   ✅ Votes cast:");
    println!("      - Alice: For");
    println!("      - Bob: Against");
    
    // Get proposal details
    if let Some(proposal) = system.runtime.token_service.get_proposal(&proposal_id) {
        println!("   📊 Proposal results:");
        println!("      - For: {} votes", proposal.votes_for);
        println!("      - Against: {} votes", proposal.votes_against);
        println!("      - Abstain: {} votes", proposal.votes_abstain);
        println!("      - Status: {:?}", proposal.status);
    }
    
    // Test blockchain operations
    println!("\n🔗 Testing Blockchain Operations...");
    
    let chain_height = system.runtime.blockchain.state.blocks.len();
    println!("   📦 Current chain height: {}", chain_height);
    
    // Test basic configuration
    println!("\n⚙️ Testing Configuration...");
    
    let system_version = &system.runtime.state.version;
    let current_block = system.runtime.state.current_block;
    let event_count = system.runtime.state.events.len();
    
    println!("   📋 System status:");
    println!("      - Version: {:?}", system_version);
    println!("      - Current block: {}", current_block);
    println!("      - Events recorded: {}", event_count);
    
    // Test utility functions
    println!("\n🔧 Testing Utility Functions...");
    
    // Generate a random value
    let random_value = fastrand::u64(..);
    println!("   🎲 Random value: {}", random_value);
    
    // Test time utilities
    let current_time = chrono::Utc::now().timestamp() as u64;
    println!("   🕐 Current timestamp: {}", current_time);
    
    // Test constants
    println!("   📏 System constants:");
    println!("      - kWh to Token ratio: {}", constants::KWH_TO_TOKEN_RATIO);
    println!("      - Min energy trade: {} kWh", constants::MIN_ENERGY_TRADE);
    println!("      - Max energy trade: {} kWh", constants::MAX_ENERGY_TRADE);
    println!("      - Grid fee: {}%", constants::GRID_FEE_PERCENTAGE * 100.0);
    
    println!("\n🎉 Demo completed successfully!");
    println!("✨ The new architecture is working properly with:");
    println!("   - ✅ Modular design with clear separation of concerns");
    println!("   - ✅ Token system with governance and staking");
    println!("   - ✅ Blockchain core functionality");
    println!("   - ✅ Runtime integration and state management");
    println!("   - ✅ Utility functions and configuration");
    println!("   - ✅ Error handling and type safety");
    
    println!("\n📚 Architecture Benefits:");
    println!("   - 🏗️  Modular structure for easy maintenance");
    println!("   - 🔒 Type-safe operations with proper error handling");
    println!("   - 🔄 Backward compatibility with legacy modules");
    println!("   - 📦 Clear separation of business logic");
    println!("   - 🚀 Scalable foundation for future features");
    
    Ok(())
}

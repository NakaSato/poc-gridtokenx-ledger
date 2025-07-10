// Modern Architecture Demo - Thai Energy Trading System
// This example demonstrates the new modular architecture

use ledger_core::ThaiEnergyTradingSystem;
use ledger_core::primitives::*;
use ledger_core::services::*;

fn main() -> CoreResult<()> {
    println!("🌟 Thai Energy Trading System - Modern Architecture Demo 🌟");
    println!("============================================================");
    
    // Initialize the system
    let mut system = ThaiEnergyTradingSystem::new();
    println!("✅ System initialized successfully");
    
    // Create some test accounts
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    
    println!("\n📋 Creating test accounts...");
    println!("   - Alice: {}", alice);
    println!("   - Bob: {}", bob);
    println!("   - Charlie: {}", charlie);
    
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
    system.runtime.token_service.transfer(&token_id, &alice, &bob, 10_000)?;
    system.runtime.token_service.transfer(&token_id, &alice, &charlie, 5_000)?;
    
    println!("   ✅ Transferred tokens:");
    println!("      - Alice → Bob: 10,000 TET");
    println!("      - Alice → Charlie: 5,000 TET");
    
    // Check balances
    let alice_balance = system.runtime.token_service.get_balance(&alice, &token_id);
    let bob_balance = system.runtime.token_service.get_balance(&bob, &token_id);
    let charlie_balance = system.runtime.token_service.get_balance(&charlie, &token_id);
    
    println!("   💰 Token balances:");
    println!("      - Alice: {} TET", alice_balance);
    println!("      - Bob: {} TET", bob_balance);
    println!("      - Charlie: {} TET", charlie_balance);
    
    // Test staking
    println!("\n🔒 Testing Staking Operations...");
    system.runtime.token_service.stake(&bob, 5_000, None)?;
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
    system.runtime.token_service.vote_on_proposal(&charlie, &proposal_id, Vote::Abstain)?;
    
    println!("   ✅ Votes cast:");
    println!("      - Alice: For");
    println!("      - Bob: Against");
    println!("      - Charlie: Abstain");
    
    // Get proposal details
    if let Some(proposal) = system.runtime.token_service.get_proposal(&proposal_id) {
        println!("   📊 Proposal results:");
        println!("      - For: {} votes", proposal.votes_for);
        println!("      - Against: {} votes", proposal.votes_against);
        println!("      - Abstain: {} votes", proposal.votes_abstain);
        println!("      - Status: {:?}", proposal.status);
    }
    
    // Test energy trading
    println!("\n⚡ Testing Energy Trading Operations...");
    
    // Register participants
    let alice_participant = Participant {
        account: alice.clone(),
        participant_type: ParticipantType::Producer,
        registered_at: chrono::Utc::now().timestamp() as u64,
        is_active: true,
        energy_balance: 1000.0,
    };
    
    let bob_participant = Participant {
        account: bob.clone(),
        participant_type: ParticipantType::Consumer,
        registered_at: chrono::Utc::now().timestamp() as u64,
        is_active: true,
        energy_balance: 0.0,
    };
    
    system.runtime.energy_trading_service.register_participant(alice_participant)?;
    system.runtime.energy_trading_service.register_participant(bob_participant)?;
    
    println!("   ✅ Registered energy trading participants:");
    println!("      - Alice: Producer (1000 kWh available)");
    println!("      - Bob: Consumer");
    
    // Test core blockchain operations
    println!("\n🔗 Testing Blockchain Operations...");
    
    let initial_block_count = system.runtime.blockchain.get_block_count();
    println!("   📦 Initial block count: {}", initial_block_count);
    
    // Create a simple transaction
    let tx_data = serde_json::json!({
        "type": "energy_trade",
        "from": alice,
        "to": bob,
        "amount": 50.0,
        "price": 3.5
    });
    
    let tx_hash = system.runtime.blockchain.create_transaction(tx_data.to_string())?;
    println!("   ✅ Created transaction: {}", tx_hash);
    
    // Mine a block
    system.runtime.blockchain.mine_block()?;
    let new_block_count = system.runtime.blockchain.get_block_count();
    println!("   ⛏️  Mined new block. Block count: {}", new_block_count);
    
    // Test utility functions
    println!("\n🔧 Testing Utility Functions...");
    
    // Generate a random value
    let random_value = fastrand::u64(..);
    println!("   🎲 Random value: {}", random_value);
    
    // Test time utilities
    let current_time = chrono::Utc::now().timestamp() as u64;
    println!("   🕐 Current timestamp: {}", current_time);
    
    // Test configuration
    let config = system.runtime.state.config.clone();
    println!("   ⚙️  Chain spec: {}", config.chain_spec.name);
    
    println!("\n🎉 Demo completed successfully!");
    println!("✨ The new architecture is working properly with:");
    println!("   - Modular design with clear separation of concerns");
    println!("   - Token system with governance and staking");
    println!("   - Energy trading capabilities");
    println!("   - Blockchain core functionality");
    println!("   - Utility functions and configuration");
    
    Ok(())
}

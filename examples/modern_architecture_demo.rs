// Modern Thai Energy Trading System Demo
// This example demonstrates the new modular architecture

use ledger_core::*;

fn main() -> CoreResult<()> {
    println!("🚀 Thai Energy Trading System - Modern Architecture Demo");
    println!("========================================================");

    // Create and initialize system
    let mut system = create_and_initialize_system()?;
    
    println!("✅ System initialized successfully");
    println!("📊 System Info:");
    let info = system.get_info();
    println!("   - Version: {}", info.version);
    println!("   - Name: {}", info.name);
    println!("   - Description: {}", info.description);
    println!("   - Runtime Version: {}", info.runtime_info.version.spec_version);
    println!("   - Current Block: {}", info.runtime_info.current_block);
    
    // Demo 1: Token Operations
    println!("\n💰 Demo 1: Token Operations");
    demo_token_operations(&mut system)?;
    
    // Demo 2: Energy Trading
    println!("\n⚡ Demo 2: Energy Trading");
    demo_energy_trading(&mut system)?;
    
    // Demo 3: Governance
    println!("\n🏛️ Demo 3: Governance");
    demo_governance(&mut system)?;
    
    // Demo 4: System Status
    println!("\n📈 Demo 4: System Status");
    demo_system_status(&system)?;
    
    println!("\n🎉 All demos completed successfully!");
    Ok(())
}

fn demo_token_operations(system: &mut ThaiEnergyTradingSystem) -> CoreResult<()> {
    // Create accounts
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    
    // Token transfer
    let result = system.runtime.execute_call(
        alice.clone(),
        Call::TokenTransfer {
            to: bob.clone(),
            token: "GRID".to_string(),
            amount: 1000 * 10_u128.pow(constants::TOKEN_DECIMALS as u32),
        },
    )?;
    
    println!("   🔄 Token transfer executed:");
    println!("      - Transaction: {}", format_hash_short(&result.transaction_hash));
    println!("      - Success: {}", result.success);
    println!("      - Gas used: {}", result.gas_used);
    
    // Check balances
    let alice_info = system.runtime.get_account_info(&alice);
    let bob_info = system.runtime.get_account_info(&bob);
    
    println!("   💳 Account balances:");
    println!("      - Alice GRID: {}", format_balance(alice_info.grid_balance, constants::TOKEN_DECIMALS));
    println!("      - Bob GRID: {}", format_balance(bob_info.grid_balance, constants::TOKEN_DECIMALS));
    
    Ok(())
}

fn demo_energy_trading(system: &mut ThaiEnergyTradingSystem) -> CoreResult<()> {
    let alice = "alice".to_string();
    
    // Create energy order
    let order = EnergyOrder {
        id: generate_hash(),
        account: alice.clone(),
        order_type: OrderType::Buy,
        energy_amount: 100.0,
        price_per_kwh: 4.50,
        total_price: 450.0,
        status: OrderStatus::Pending,
        created_at: current_timestamp(),
        expires_at: time::add_days(current_timestamp(), 1),
        filled_amount: 0.0,
    };
    
    // Place order
    let result = system.runtime.execute_call(
        alice.clone(),
        Call::EnergyTrade { order },
    )?;
    
    println!("   ⚡ Energy order placed:");
    println!("      - Transaction: {}", format_hash_short(&result.transaction_hash));
    println!("      - Success: {}", result.success);
    println!("      - Energy: {} kWh", 100.0);
    println!("      - Price: {} THB/kWh", 4.50);
    
    // Get market statistics
    let market_stats = system.runtime.energy_trading_service.get_market_stats();
    println!("   📊 Market Statistics:");
    println!("      - Total participants: {}", market_stats.total_participants);
    println!("      - Total trades: {}", market_stats.total_trades);
    println!("      - Average price: {} THB/kWh", market_stats.average_price);
    
    Ok(())
}

fn demo_governance(system: &mut ThaiEnergyTradingSystem) -> CoreResult<()> {
    let alice = "alice".to_string();
    
    // Stake tokens first
    let stake_result = system.runtime.execute_call(
        alice.clone(),
        Call::Stake {
            amount: constants::MIN_VALIDATOR_STAKE,
            validator: None,
        },
    )?;
    
    println!("   🔒 Tokens staked:");
    println!("      - Amount: {}", format_balance(constants::MIN_VALIDATOR_STAKE, constants::TOKEN_DECIMALS));
    println!("      - Success: {}", stake_result.success);
    
    // Create governance proposal
    let proposal = GovernanceProposal {
        id: generate_hash(),
        title: "Increase Block Size".to_string(),
        description: "Proposal to increase block size to improve throughput".to_string(),
        proposer: alice.clone(),
        proposal_type: ProposalType::ParameterChange,
        voting_period: (current_timestamp(), time::add_days(current_timestamp(), 7)),
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Voting,
        execution_time: None,
    };
    
    let proposal_id = proposal.id.clone();
    let result = system.runtime.execute_call(
        alice.clone(),
        Call::CreateProposal { proposal },
    )?;
    
    println!("   📋 Governance proposal created:");
    println!("      - Proposal ID: {}", format_hash_short(&proposal_id));
    println!("      - Success: {}", result.success);
    
    // Vote on proposal
    let vote_result = system.runtime.execute_call(
        alice.clone(),
        Call::Vote {
            proposal_id,
            vote: Vote::For,
        },
    )?;
    
    println!("   🗳️ Vote cast:");
    println!("      - Success: {}", vote_result.success);
    println!("      - Vote: For");
    
    Ok(())
}

fn demo_system_status(system: &ThaiEnergyTradingSystem) -> CoreResult<()> {
    let status = system.runtime.get_system_status();
    
    println!("   🏥 System Health: {}", if status.is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    println!("   📊 System Metrics:");
    println!("      - Current Block: {}", status.current_block);
    println!("      - Total Accounts: {}", status.total_accounts);
    println!("      - Total Tokens: {}", status.total_tokens);
    println!("      - Active Trades: {}", status.active_trades);
    println!("      - Active Proposals: {}", status.active_proposals);
    
    // Runtime statistics
    let runtime_info = system.runtime.get_runtime_info();
    println!("   📈 Runtime Statistics:");
    println!("      - Total Blocks: {}", runtime_info.stats.total_blocks);
    println!("      - Total Transactions: {}", runtime_info.stats.total_transactions);
    println!("      - Total Energy Trades: {}", runtime_info.stats.total_energy_trades);
    println!("      - Total Token Transfers: {}", runtime_info.stats.total_token_transfers);
    
    Ok(())
}

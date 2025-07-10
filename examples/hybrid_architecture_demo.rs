// Demo of the hybrid blockchain architecture for Thailand energy trading
// This example showcases the three-part system: Public, Consortium, and Oracle chains

use ledger_core::hybrid_architecture::*;
use ledger_core::token_system::*;
use ledger_core::energy_trading::*;

fn main() -> Result<(), String> {
    println!("🔋 Thailand Energy Trading Hybrid Blockchain Demo");
    println!("=================================================");
    
    // Create and initialize the hybrid system
    let mut system = create_hybrid_system()?;
    
    // Initialize Thailand-specific compliance
    system.initialize_thailand_compliance()?;
    
    println!("✅ Hybrid system initialized with Thai regulatory compliance");
    
    // Demo 1: Governance Proposal
    println!("\n📊 Demo 1: Governance Proposal Submission");
    demo_governance_proposal(&mut system)?;
    
    // Demo 2: Energy Trading
    println!("\n⚡ Demo 2: Cross-chain Energy Trading");
    demo_energy_trade(&mut system)?;
    
    // Demo 3: Oracle Updates
    println!("\n🌡️ Demo 3: Oracle Data Updates");
    demo_oracle_update(&mut system)?;
    
    // Demo 4: Cross-chain Transfer
    println!("\n🔄 Demo 4: Cross-chain Token Transfer");
    demo_cross_chain_transfer(&mut system)?;
    
    // Demo 5: Compliance Reporting
    println!("\n📋 Demo 5: Compliance Reporting");
    demo_compliance_reporting(&mut system)?;
    
    // Show final system status
    println!("\n📈 Final System Status");
    show_system_status(&system);
    
    println!("\n✅ All demos completed successfully!");
    Ok(())
}

fn create_hybrid_system() -> Result<HybridSystem, String> {
    let config = HybridConfig {
        public_chain_config: PublicChainConfig {
            consensus: "nominated-proof-of-stake".to_string(),
            block_time: 6, // seconds
            validators: 100,
            public_participation: true,
        },
        consortium_chain_config: ConsortiumChainConfig {
            consensus: "practical-byzantine-fault-tolerance".to_string(),
            block_time: 1, // seconds
            validators: 21,
            permission_required: true,
        },
        oracle_config: OracleConfig {
            data_sources: vec!["weather".to_string(), "grid-status".to_string(), "energy-prices".to_string()],
            update_frequency: 10, // seconds
            fault_tolerance: 0.67,
        },
    };
    
    let system = HybridSystem::new(config);
    
    println!("🏗️ Created hybrid system with:");
    println!("   - Public Chain: 100 validators, 6s blocks");
    println!("   - Consortium Chain: 21 validators, 1s blocks");
    println!("   - Oracle Gateway: 3 data sources, 10s updates");
    
    Ok(system)
}

fn demo_governance_proposal(system: &mut HybridSystem) -> Result<(), String> {
    // Create a sample governance proposal
    let proposal = GovernanceProposal {
        id: format!("prop_{}", chrono::Utc::now().timestamp()),
        title: "Renewable Energy Incentive Policy".to_string(),
        description: "Proposal to increase incentives for solar energy production".to_string(),
        proposer: "thai_energy_authority".to_string(),
        votes_for: 0.0,
        votes_against: 0.0,
        voting_deadline: chrono::Utc::now() + chrono::Duration::days(30),
        status: ProposalStatus::Active,
    };
    
    // Submit governance proposal through cross-chain transaction
    let transaction = CrossChainTransaction {
        id: format!("gov_tx_{}", chrono::Utc::now().timestamp()),
        source_chain: ChainType::Public,
        target_chain: ChainType::Public,
        transaction_type: TransactionType::GovernanceVote,
        amount: 0.0,
        data: serde_json::to_value(&proposal).map_err(|e| e.to_string())?,
        timestamp: chrono::Utc::now(),
    };
    
    let tx_id = system.process_cross_chain_transaction(transaction)?;
    println!("   📝 Submitted governance proposal: {}", proposal.title);
    println!("   🆔 Transaction ID: {}", tx_id);
    
    Ok(())
}

fn demo_energy_trade(system: &mut HybridSystem) -> Result<(), String> {
    // Create a sample energy trade
    let trade = EnergyTrade {
        trade_id: format!("trade_{}", chrono::Utc::now().timestamp()),
        buyer: "industrial_consumer_bangkok".to_string(),
        seller: "solar_farm_chiang_mai".to_string(),
        energy_amount: 1000.0, // kWh
        price_per_kwh: 4.50, // THB
        total_cost: 4500.0, // THB
        grid_fee: 45.0, // 1% grid fee
        timestamp: chrono::Utc::now(),
    };
    
    // Process through cross-chain transaction
    let transaction = CrossChainTransaction {
        id: format!("energy_tx_{}", chrono::Utc::now().timestamp()),
        source_chain: ChainType::Consortium,
        target_chain: ChainType::Consortium,
        transaction_type: TransactionType::EnergyTrade,
        amount: trade.energy_amount,
        data: serde_json::to_value(&trade).map_err(|e| e.to_string())?,
        timestamp: chrono::Utc::now(),
    };
    
    let tx_id = system.process_cross_chain_transaction(transaction)?;
    println!("   ⚡ Energy trade executed: {} kWh", trade.energy_amount);
    println!("   💰 Total cost: {} THB", trade.total_cost);
    println!("   🆔 Transaction ID: {}", tx_id);
    
    Ok(())
}

fn demo_oracle_update(system: &mut HybridSystem) -> Result<(), String> {
    // Create sample oracle data update
    let oracle_data = serde_json::json!({
        "temperature": 32.5,
        "humidity": 78.0,
        "solar_irradiance": 850.0,
        "grid_load": 15420.0,
        "energy_price": 4.75
    });
    
    // Process oracle update through cross-chain transaction
    let transaction = CrossChainTransaction {
        id: format!("oracle_tx_{}", chrono::Utc::now().timestamp()),
        source_chain: ChainType::Oracle,
        target_chain: ChainType::Oracle,
        transaction_type: TransactionType::OracleUpdate,
        amount: 0.0,
        data: oracle_data,
        timestamp: chrono::Utc::now(),
    };
    
    let tx_id = system.process_cross_chain_transaction(transaction)?;
    println!("   🌡️ Weather data updated: 32.5°C, 78% humidity");
    println!("   ☀️ Solar irradiance: 850 W/m²");
    println!("   🔌 Grid load: 15,420 MW");
    println!("   🆔 Transaction ID: {}", tx_id);
    
    Ok(())
}

fn demo_cross_chain_transfer(system: &mut HybridSystem) -> Result<(), String> {
    // Create sample token transfer data
    let transfer_data = serde_json::json!({
        "token_type": "GRID",
        "amount": 1000.0,
        "from_address": "public_chain_user_001",
        "to_address": "consortium_participant_002",
        "purpose": "staking_reward"
    });
    
    // Process cross-chain token transfer
    let transaction = CrossChainTransaction {
        id: format!("transfer_tx_{}", chrono::Utc::now().timestamp()),
        source_chain: ChainType::Public,
        target_chain: ChainType::Consortium,
        transaction_type: TransactionType::TokenTransfer,
        amount: 1000.0,
        data: transfer_data,
        timestamp: chrono::Utc::now(),
    };
    
    let tx_id = system.process_cross_chain_transaction(transaction)?;
    println!("   🔄 Cross-chain transfer: 1000 GRID tokens");
    println!("   📍 From: Public Chain → Consortium Chain");
    println!("   🆔 Transaction ID: {}", tx_id);
    
    Ok(())
}

fn demo_compliance_reporting(system: &mut HybridSystem) -> Result<(), String> {
    // Create sample compliance report
    let compliance_data = serde_json::json!({
        "report_type": "SEC_quarterly",
        "quarter": "Q1_2025",
        "total_transactions": 125000,
        "total_volume_kwh": 1500000.0,
        "total_value_thb": 6750000.0,
        "compliance_status": "compliant",
        "audit_notes": "All transactions verified, KYC requirements met"
    });
    
    // Submit compliance report through cross-chain transaction
    let transaction = CrossChainTransaction {
        id: format!("compliance_tx_{}", chrono::Utc::now().timestamp()),
        source_chain: ChainType::Public,
        target_chain: ChainType::Public,
        transaction_type: TransactionType::ComplianceReport,
        amount: 0.0,
        data: compliance_data,
        timestamp: chrono::Utc::now(),
    };
    
    let tx_id = system.process_cross_chain_transaction(transaction)?;
    println!("   📋 SEC quarterly report submitted");
    println!("   📊 Q1 2025: 125k transactions, 1.5M kWh");
    println!("   ✅ Compliance status: COMPLIANT");
    println!("   🆔 Transaction ID: {}", tx_id);
    
    Ok(())
}

fn show_system_status(system: &HybridSystem) {
    let status = system.get_system_status();
    
    println!("   🔗 Public Chain: {}", status.public_chain_status);
    println!("   🤝 Consortium Chain: {}", status.consortium_chain_status);
    println!("   🔮 Oracle Gateway: {}", status.oracle_gateway_status);
    println!("   📋 Compliance: {}", status.compliance_status);
}

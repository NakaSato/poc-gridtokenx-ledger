# Hybrid Architecture Pallet

A comprehensive hybrid blockchain architecture system for Thailand's P2P energy trading platform.

## Overview

This pallet implements a three-layer hybrid blockchain architecture designed specifically for Thailand's energy trading ecosystem, providing:

- **Public Chain Layer**: Governance, transparency, and public participation
- **Consortium Chain Layer**: Energy trading and operational efficiency  
- **Oracle Gateway Layer**: External data integration and API connectivity

## Architecture

### Three-Layer Design

#### 1. Public Chain Layer
- **Consensus**: Nominated Proof of Stake (NPoS)
- **Block Time**: 6 seconds
- **Validators**: 100 validators
- **Purpose**: Governance proposals, public transparency, compliance reporting
- **Features**: Token staking, governance voting, public participation

#### 2. Consortium Chain Layer
- **Consensus**: Practical Byzantine Fault Tolerance (PBFT)
- **Block Time**: 1 second
- **Validators**: 21 authorized validators
- **Purpose**: Energy trading, settlement, operational efficiency
- **Features**: High-throughput trading, instant settlement, permissioned access

#### 3. Oracle Gateway Layer
- **Purpose**: External data integration
- **Data Sources**: Weather, grid status, energy prices, regulatory data
- **Update Frequency**: 10 seconds
- **Features**: Data validation, fault tolerance, API connectivity

### Cross-Chain Integration

The pallet provides seamless cross-chain communication through:

- **Transaction Routing**: Automatic routing based on transaction type
- **State Synchronization**: Real-time state updates across chains
- **Asset Transfers**: Secure cross-chain token transfers
- **Message Passing**: Inter-chain communication protocol

## Features

### Core Features

- `public-chain`: Enable public governance chain
- `consortium-chain`: Enable consortium energy trading chain
- `oracle-gateway`: Enable oracle data gateway
- `cross-chain`: Enable cross-chain integration
- `integration-bridge`: Cross-chain bridge functionality

### Thai Regulatory Compliance

- `sec-compliance`: SEC Thailand compliance
- `erc-compliance`: ERC Thailand compliance
- `pdpa-compliance`: PDPA compliance
- `automated-reporting`: Automated compliance reporting

### Performance & Monitoring

- `performance-optimized`: Performance optimizations
- `batch-processing`: Batch transaction processing
- `metrics`: Performance metrics collection
- `health-checks`: System health monitoring
- `alerting`: System alerting

### Security

- `access-control`: Enhanced access control
- `audit-logging`: Comprehensive audit logging
- `transaction-validation`: Enhanced transaction validation
- `multi-signature`: Multi-signature support

## Usage

### Basic Setup

```rust
use pallet_hybrid_architecture::*;

// Create default configuration
let config = HybridArchitectureConfig::default();

// Create hybrid architecture system
let mut system = HybridArchitectureSystem::new(config);
```

### Custom Configuration

```rust
// Build custom configuration
let config = HybridArchitectureConfigBuilder::new()
    .with_public_chain(PublicChainConfig {
        consensus: "nominated-proof-of-stake".to_string(),
        block_time: 6,
        validator_count: 100,
        public_participation: true,
        min_validator_stake: 1_000_000,
        governance_threshold: 100_000,
    })
    .with_consortium_chain(ConsortiumChainConfig {
        consensus: "practical-byzantine-fault-tolerance".to_string(),
        block_time: 1,
        validator_count: 21,
        permission_required: true,
        max_throughput: 10_000,
        settlement_finality: 2,
    })
    .with_compliance(ComplianceConfig {
        sec_enabled: true,
        erc_enabled: true,
        pdpa_enabled: true,
        automated_reporting: true,
        audit_retention_days: 2555,
    })
    .build();

let mut system = HybridArchitectureSystem::new(config);
```

### Cross-Chain Transactions

```rust
// Create cross-chain transaction
let transaction = CrossChainTransaction {
    id: "tx_123".to_string(),
    source_chain: ChainType::Public,
    target_chain: ChainType::Consortium,
    transaction_type: TransactionType::TokenTransfer,
    amount: 1000.0,
    data: serde_json::json!({"recipient": "0x123", "amount": 1000}),
    timestamp: Utc::now(),
    status: TransactionStatus::Pending,
    confirmations: 0,
    required_confirmations: 12,
};

// Submit transaction
let tx_id = system.submit_cross_chain_transaction(transaction)?;

// Process pending transactions
let processed = system.process_pending_transactions()?;
```

### Health Monitoring

```rust
// Perform health check
system.perform_health_check()?;

// Get health status
let health = system.get_health_status();
println!("Overall health: {:?}", health.overall_health);
println!("Public chain: {:?}", health.public_chain_health);
println!("Consortium chain: {:?}", health.consortium_chain_health);
```

### Performance Metrics

```rust
// Get performance statistics
let stats = system.get_performance_stats();
println!("Total transactions: {}", stats.total_transactions);
println!("Success rate: {:.2}%", stats.success_rate * 100.0);
println!("Average processing time: {:.2}ms", stats.avg_processing_time);
println!("Current throughput: {:.2} TPS", stats.current_throughput);
```

## Transaction Types

The system supports various transaction types:

- `GovernanceVote`: Governance proposals and voting (Public Chain)
- `EnergyTrade`: Energy trading transactions (Consortium Chain)
- `TokenTransfer`: Token transfers between chains (Public/Consortium)
- `OracleUpdate`: Oracle data updates (Oracle Gateway)
- `ComplianceReport`: Regulatory compliance reports (Public Chain)
- `Emergency`: Emergency transactions (All chains)

## Chain Status

Monitor the status of each chain:

```rust
// Get chain status
let public_status = system.get_chain_status(&ChainType::Public);
let consortium_status = system.get_chain_status(&ChainType::Consortium);
let oracle_status = system.get_chain_status(&ChainType::Oracle);

// Check operational status
if let Some(status) = public_status {
    println!("Public chain operational: {}", status.is_operational);
    println!("Block height: {}", status.current_block_height);
    println!("Validator count: {}", status.validator_count);
    println!("Throughput: {} TPS", status.transaction_throughput);
}
```

## Events

The system emits various events for monitoring:

- `CrossChainTransactionInitiated`: Transaction started
- `CrossChainTransactionConfirmed`: Transaction confirmed
- `CrossChainTransactionFinalized`: Transaction finalized
- `ChainStatusUpdated`: Chain status changed
- `ValidatorAdded/Removed`: Validator changes
- `ComplianceReportSubmitted`: Compliance report submitted
- `EmergencyAlertIssued`: Emergency alert
- `SystemHealthCheck`: Health check performed

## Error Handling

The system provides comprehensive error handling:

```rust
match system.submit_cross_chain_transaction(transaction) {
    Ok(tx_id) => println!("Transaction submitted: {}", tx_id),
    Err(HybridArchitectureError::ChainNotOperational(chain)) => {
        println!("Chain not operational: {}", chain);
    }
    Err(HybridArchitectureError::InvalidTransactionType(msg)) => {
        println!("Invalid transaction type: {}", msg);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test
cargo test --features "cross-chain,compliance,metrics,health-checks"
```

## Performance Characteristics

- **Public Chain**: 1,000 TPS, 6-second finality
- **Consortium Chain**: 10,000 TPS, 1-second finality
- **Oracle Gateway**: 500 TPS, 10-second data updates
- **Cross-Chain Bridge**: 100ms average processing time

## Compliance

The system is designed to meet Thailand's regulatory requirements:

- **SEC Thailand**: Securities and Exchange Commission compliance
- **ERC Thailand**: Energy Regulatory Commission compliance
- **PDPA**: Personal Data Protection Act compliance
- **Automated Reporting**: Quarterly and annual compliance reports

## Security

- **Multi-signature**: Support for multi-signature transactions
- **Access Control**: Role-based access control
- **Audit Logging**: Comprehensive audit trails
- **Transaction Validation**: Multi-layer validation
- **Consensus Security**: Byzantine fault tolerance

## License

Licensed under the Apache License, Version 2.0.

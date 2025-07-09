#[cfg(test)]
mod tests {
    use crate::block::{Transaction, TransactionType};
    use crate::blockchain::Blockchain;
    use crate::energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer};
    use crate::token_system::TokenSystem;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Genesis block
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_energy_transaction_creation() {
        let transaction = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            10.0,
            0.15,
            TransactionType::EnergyTrade,
        );

        assert_eq!(transaction.from, "alice");
        assert_eq!(transaction.to, "bob");
        assert_eq!(transaction.energy_amount, 10.0);
        assert_eq!(transaction.price_per_kwh, 0.15);
        assert_eq!(transaction.energy_amount * transaction.price_per_kwh, 1.5);
    }

    #[test]
    fn test_prosumer_energy_management() {
        let mut prosumer = Prosumer::new("alice".to_string(), "Alice's Solar".to_string());
        
        prosumer.generate_energy(50.0);
        prosumer.consume_energy(30.0);
        
        assert_eq!(prosumer.energy_generated, 50.0);
        assert_eq!(prosumer.energy_consumed, 30.0);
        assert_eq!(prosumer.get_net_energy(), 20.0);
    }

    #[test]
    fn test_energy_market_order_placement() {
        let mut market = EnergyMarket::new();
        
        let sell_order = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            10.0,
            0.15,
        );
        
        let buy_order = EnergyOrder::new(
            "bob".to_string(),
            OrderType::Buy,
            10.0,
            0.16,
        );
        
        market.place_order(sell_order).unwrap();
        market.place_order(buy_order).unwrap();
        
        // Orders should be matched
        assert_eq!(market.matched_trades.len(), 1);
        assert_eq!(market.matched_trades[0].energy_amount, 10.0);
        assert_eq!(market.matched_trades[0].price_per_kwh, 0.15); // Sell price
    }

    #[test]
    fn test_energy_market_price_priority() {
        let mut market = EnergyMarket::new();
        
        // Add sell orders with different prices
        let sell_order_1 = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            10.0,
            0.15,
        );
        
        let sell_order_2 = EnergyOrder::new(
            "bob".to_string(),
            OrderType::Sell,
            10.0,
            0.12, // Lower price, should be first
        );
        
        market.place_order(sell_order_1).unwrap();
        market.place_order(sell_order_2).unwrap();
        
        // Market price should be the lowest sell price
        assert_eq!(market.get_market_price(), Some(0.12));
    }

    #[test]
    fn test_token_system_account_creation() {
        let mut token_system = TokenSystem::new();
        
        token_system.create_user_account("alice".to_string()).unwrap();
        
        let balance = token_system.get_user_balance("alice").unwrap();
        assert_eq!(balance.address, "alice");
        assert_eq!(balance.grid_balance, 0.0);
        assert_eq!(balance.watt_balance, 0.0);
    }

    #[test]
    fn test_watt_token_mint_burn() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        
        // Mint tokens
        token_system.mint_watt_tokens("alice", 100.0).unwrap();
        let balance = token_system.get_user_balance("alice").unwrap();
        assert_eq!(balance.watt_balance, 100.0);
        assert_eq!(token_system.watt_token.total_supply, 100.0);
        
        // Burn tokens
        token_system.burn_watt_tokens("alice", 50.0).unwrap();
        let balance = token_system.get_user_balance("alice").unwrap();
        assert_eq!(balance.watt_balance, 50.0);
        assert_eq!(token_system.watt_token.total_supply, 50.0);
    }

    #[test]
    fn test_grid_token_staking() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        
        // Add some GRID tokens first
        token_system.user_balances.get_mut("alice").unwrap().grid_balance = 1000.0;
        
        // Stake tokens
        token_system.stake_grid_tokens("alice", 500.0).unwrap();
        
        let balance = token_system.get_user_balance("alice").unwrap();
        assert_eq!(balance.grid_balance, 500.0);
        assert_eq!(balance.staked_grid, 500.0);
    }

    #[test]
    fn test_watt_token_transfer() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        token_system.create_user_account("bob".to_string()).unwrap();
        
        // Give Alice some tokens
        token_system.mint_watt_tokens("alice", 100.0).unwrap();
        
        // Transfer tokens
        token_system.transfer_watt_tokens("alice", "bob", 30.0).unwrap();
        
        let alice_balance = token_system.get_user_balance("alice").unwrap();
        let bob_balance = token_system.get_user_balance("bob").unwrap();
        
        assert_eq!(alice_balance.watt_balance, 70.0);
        assert_eq!(bob_balance.watt_balance, 30.0);
    }

    #[test]
    fn test_governance_proposal_creation() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        
        // Give Alice enough staked tokens to create a proposal
        token_system.user_balances.get_mut("alice").unwrap().grid_balance = 2000.0;
        token_system.stake_grid_tokens("alice", 1500.0).unwrap();
        
        let proposal_id = token_system.create_governance_proposal(
            "alice",
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            7,
        ).unwrap();
        
        assert_eq!(token_system.grid_token.governance_proposals.len(), 1);
        assert_eq!(token_system.grid_token.governance_proposals[0].id, proposal_id);
    }

    #[test]
    fn test_governance_voting() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        token_system.create_user_account("bob".to_string()).unwrap();
        
        // Setup staking for both users
        token_system.user_balances.get_mut("alice").unwrap().grid_balance = 2000.0;
        token_system.user_balances.get_mut("bob").unwrap().grid_balance = 1000.0;
        
        token_system.stake_grid_tokens("alice", 1500.0).unwrap();
        token_system.stake_grid_tokens("bob", 500.0).unwrap();
        
        // Create proposal
        let proposal_id = token_system.create_governance_proposal(
            "alice",
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            7,
        ).unwrap();
        
        // Vote on proposal
        token_system.vote_on_proposal("alice", &proposal_id, true).unwrap();
        token_system.vote_on_proposal("bob", &proposal_id, false).unwrap();
        
        let proposal = &token_system.grid_token.governance_proposals[0];
        assert_eq!(proposal.votes_for, 1500.0);
        assert_eq!(proposal.votes_against, 500.0);
    }

    #[test]
    fn test_blockchain_energy_balance() {
        let mut blockchain = Blockchain::new();
        
        let energy_transaction = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            10.0,
            0.15,
            TransactionType::EnergyTrade,
        );
        
        blockchain.create_transaction(energy_transaction);
        blockchain.mine_pending_transactions("miner");
        
        let alice_balance = blockchain.get_energy_balance("alice");
        let bob_balance = blockchain.get_energy_balance("bob");
        
        assert_eq!(alice_balance, -10.0); // Alice sold energy
        assert_eq!(bob_balance, 10.0);    // Bob bought energy
    }

    #[test]
    fn test_market_grid_fees() {
        let mut market = EnergyMarket::new();
        market.grid_fee_rate = 0.10; // 10% grid fee
        
        let sell_order = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            10.0,
            0.15,
        );
        
        let buy_order = EnergyOrder::new(
            "bob".to_string(),
            OrderType::Buy,
            10.0,
            0.16,
        );
        
        market.place_order(sell_order).unwrap();
        market.place_order(buy_order).unwrap();
        
        let trade = &market.matched_trades[0];
        assert_eq!(trade.energy_amount, 10.0);
        assert_eq!(trade.price_per_kwh, 0.15);
        assert!((trade.grid_fee - 0.15).abs() < 0.0001); // Use floating point comparison
        assert!((trade.total_cost - 1.65).abs() < 0.0001); // Use floating point comparison
    }

    #[test]
    fn test_order_partial_matching() {
        let mut market = EnergyMarket::new();
        
        let sell_order = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            20.0, // Selling 20 kWh
            0.15,
        );
        
        let buy_order = EnergyOrder::new(
            "bob".to_string(),
            OrderType::Buy,
            10.0, // Buying only 10 kWh
            0.16,
        );
        
        market.place_order(sell_order).unwrap();
        market.place_order(buy_order).unwrap();
        
        // Should create one trade for 10 kWh
        assert_eq!(market.matched_trades.len(), 1);
        assert_eq!(market.matched_trades[0].energy_amount, 10.0);
        
        // Alice should still have 10 kWh left to sell
        assert_eq!(market.sell_orders.len(), 1);
        assert_eq!(market.sell_orders[0].energy_amount, 10.0);
        
        // Bob's order should be completely filled
        assert_eq!(market.buy_orders.len(), 0);
    }

    #[test]
    fn test_blockchain_validation() {
        let mut blockchain = Blockchain::new();
        
        // Add some transactions
        let transaction1 = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            10.0,
            0.15,
            TransactionType::EnergyTrade,
        );
        
        blockchain.create_transaction(transaction1);
        blockchain.mine_pending_transactions("miner");
        
        // Blockchain should be valid
        assert!(blockchain.is_chain_valid());
        
        // Tamper with a block hash
        blockchain.chain[1].hash = "tampered_hash".to_string();
        
        // Blockchain should now be invalid
        assert!(!blockchain.is_chain_valid());
    }

    #[test]
    fn test_insufficient_funds_error() {
        let mut token_system = TokenSystem::new();
        token_system.create_user_account("alice".to_string()).unwrap();
        
        // Try to transfer more than available
        let result = token_system.transfer_watt_tokens("alice", "bob", 50.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient WATT balance");
    }

    #[test]
    fn test_invalid_order_parameters() {
        let mut market = EnergyMarket::new();
        
        // Test negative energy amount
        let invalid_order = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            -10.0, // Negative amount
            0.15,
        );
        
        let result = market.place_order(invalid_order);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Energy amount must be positive");
        
        // Test zero price
        let invalid_order2 = EnergyOrder::new(
            "alice".to_string(),
            OrderType::Sell,
            10.0,
            0.0, // Zero price
        );
        
        let result2 = market.place_order(invalid_order2);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), "Price must be positive");
    }

    #[test]
    fn test_one_kwh_equals_one_token() {
        let mut alice = Prosumer::new("alice".to_string(), "Alice Solar".to_string());
        
        // Alice generates 50 kWh
        alice.generate_energy(50.0);
        alice.consume_energy(10.0);
        
        // Net energy should be 40 kWh
        assert_eq!(alice.get_net_energy(), 40.0);
        
        // Energy tokens should be 1:1 with net energy
        assert_eq!(alice.get_sellable_energy_tokens(), 40.0);
        assert_eq!(alice.get_required_energy_tokens(), 0.0);
    }

    #[test]
    fn test_consumer_token_requirements() {
        let mut charlie = Prosumer::new("charlie".to_string(), "Charlie Consumer".to_string());
        
        // Charlie consumes 25 kWh
        charlie.consume_energy(25.0);
        
        // Net energy should be negative
        assert_eq!(charlie.get_net_energy(), -25.0);
        
        // Energy tokens should show requirements
        assert_eq!(charlie.get_sellable_energy_tokens(), 0.0);
        assert_eq!(charlie.get_required_energy_tokens(), 25.0);
    }

    #[test]
    fn test_energy_token_conversion_functions() {
        // Test simple 1:1 conversion logic
        let alice_energy = 10.0;
        let price = 0.15;
        
        // 1 kWh = 1 token, cost = tokens * price
        let cost = alice_energy * price;
        assert_eq!(cost, 1.5); // 10 tokens * $0.15 = $1.50
        
        let larger_amount = 25.0;
        let different_price = 0.12;
        let larger_cost = larger_amount * different_price;
        assert_eq!(larger_cost, 3.0); // 25 tokens * $0.12 = $3.00
    }
}

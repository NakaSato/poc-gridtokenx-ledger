use crate::block::{Block, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
            pending_transactions: Vec::new(),
            mining_reward: 10.0,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, Vec::new(), "0".to_string());
        self.chain.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: &str) {
        let reward_transaction = Transaction::new(
            "System".to_string(),
            mining_reward_address.to_string(),
            0.0, // No energy transfer for mining reward
            0.0,
            crate::block::TransactionType::TokenStaking,
        );
        self.pending_transactions.push(reward_transaction);

        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.get_latest_block().hash.clone(),
        );
        block.mine_block(self.difficulty);
        
        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address {
                    balance -= transaction.calculate_total_cost();
                }
                if transaction.to == address {
                    balance += transaction.calculate_total_cost();
                }
            }
        }

        balance
    }

    pub fn get_energy_balance(&self, address: &str) -> f64 {
        let mut energy_balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                match transaction.transaction_type {
                    crate::block::TransactionType::EnergyTrade => {
                        if transaction.from == address {
                            energy_balance -= transaction.energy_amount;
                        }
                        if transaction.to == address {
                            energy_balance += transaction.energy_amount;
                        }
                    }
                    _ => {}
                }
            }
        }

        energy_balance
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn get_transactions_for_address(&self, address: &str) -> Vec<&Transaction> {
        let mut transactions = Vec::new();

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address || transaction.to == address {
                    transactions.push(transaction);
                }
            }
        }

        transactions
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

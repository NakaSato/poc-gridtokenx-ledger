use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub energy_amount: f64, // kWh
    pub price_per_kwh: f64, // Price in WATT stablecoin
    pub timestamp: DateTime<Utc>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    EnergyTrade,
    GridFee,
    TokenStaking,
    GovernanceVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            serde_json::to_string(&self.transactions).unwrap_or_default(),
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

impl Transaction {
    pub fn new(
        from: String,
        to: String,
        energy_amount: f64,
        price_per_kwh: f64,
        transaction_type: TransactionType,
    ) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            energy_amount,
            price_per_kwh,
            timestamp: Utc::now(),
            transaction_type,
        }
    }
}
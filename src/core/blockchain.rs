// Core blockchain functionality
// This module contains the fundamental blockchain operations

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Block header structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockHeader {
    /// Block number
    pub number: BlockNumber,
    /// Previous block hash
    pub parent_hash: Hash,
    /// State root hash
    pub state_root: Hash,
    /// Transactions root hash
    pub transactions_root: Hash,
    /// Block timestamp
    pub timestamp: Timestamp,
    /// Block author/validator
    pub author: AccountId,
    /// Difficulty (for PoW chains)
    pub difficulty: u64,
    /// Nonce
    pub nonce: u64,
}

/// Block body structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockBody {
    /// Transactions in the block
    pub transactions: Vec<Transaction>,
    /// Justifications/proofs
    pub justifications: Vec<Justification>,
}

/// Complete block structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Block body
    pub body: BlockBody,
}

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Transaction hash
    pub hash: Hash,
    /// Sender account
    pub from: AccountId,
    /// Receiver account
    pub to: AccountId,
    /// Transaction amount
    pub amount: Balance,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Transaction data
    pub data: Vec<u8>,
    /// Transaction fee
    pub fee: Balance,
    /// Block number when included
    pub block_number: Option<BlockNumber>,
    /// Transaction timestamp
    pub timestamp: Timestamp,
    /// Transaction signature
    pub signature: Option<String>,
}

/// Transaction types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    /// Token transfer
    Transfer,
    /// Energy trade
    EnergyTrade,
    /// Staking operation
    Staking,
    /// Governance vote
    GovernanceVote,
    /// Smart contract deployment
    ContractDeploy,
    /// Smart contract call
    ContractCall,
}

/// Justification for block validity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Justification {
    /// Justification type
    pub justification_type: JustificationType,
    /// Proof data
    pub proof: Vec<u8>,
    /// Validator signatures
    pub signatures: Vec<ValidatorSignature>,
}

/// Justification types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JustificationType {
    /// Finality proof
    Finality,
    /// Consensus proof
    Consensus,
    /// Authority proof
    Authority,
}

/// Validator signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidatorSignature {
    /// Validator account
    pub validator: AccountId,
    /// Signature
    pub signature: String,
    /// Timestamp
    pub timestamp: Timestamp,
}

/// Blockchain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainState {
    /// Current best block
    pub best_block: Block,
    /// Finalized block
    pub finalized_block: Block,
    /// Block storage
    pub blocks: HashMap<Hash, Block>,
    /// Transaction storage
    pub transactions: HashMap<Hash, Transaction>,
    /// Account states
    pub accounts: HashMap<AccountId, AccountState>,
    /// Pending transactions
    pub pending_transactions: Vec<Transaction>,
}

/// Account state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountState {
    /// Account balance
    pub balance: Balance,
    /// Account nonce
    pub nonce: u64,
    /// Staked amount
    pub staked: Balance,
    /// Account type
    pub account_type: AccountType,
    /// Energy balance
    pub energy_balance: EnergyAmount,
    /// Last activity timestamp
    pub last_activity: Timestamp,
}

/// Account types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    /// Regular user
    User,
    /// Energy producer
    Producer,
    /// Energy consumer
    Consumer,
    /// Prosumer (producer + consumer)
    Prosumer,
    /// Validator
    Validator,
    /// Authority
    Authority,
}

/// Blockchain implementation
pub struct Blockchain {
    /// Blockchain state
    pub state: BlockchainState,
    /// System configuration
    pub config: SystemConfig,
}

impl Blockchain {
    /// Create new blockchain
    pub fn new(config: SystemConfig) -> Self {
        let genesis_block = Self::create_genesis_block(&config);
        
        let mut blocks = HashMap::new();
        blocks.insert(genesis_block.header.parent_hash.clone(), genesis_block.clone());
        
        let state = BlockchainState {
            best_block: genesis_block.clone(),
            finalized_block: genesis_block,
            blocks,
            transactions: HashMap::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        };

        Self { state, config }
    }

    /// Create genesis block
    fn create_genesis_block(_config: &SystemConfig) -> Block {
        let header = BlockHeader {
            number: 0,
            parent_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            state_root: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            transactions_root: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            author: "genesis".to_string(),
            difficulty: 0,
            nonce: 0,
        };

        let body = BlockBody {
            transactions: Vec::new(),
            justifications: Vec::new(),
        };

        Block { header, body }
    }

    /// Add transaction to pending pool
    pub fn add_transaction(&mut self, transaction: Transaction) -> CoreResult<()> {
        // Validate transaction
        self.validate_transaction(&transaction)?;
        
        // Add to pending pool
        self.state.pending_transactions.push(transaction);
        
        Ok(())
    }

    /// Validate transaction
    fn validate_transaction(&self, transaction: &Transaction) -> CoreResult<()> {
        // Check sender exists
        if !self.state.accounts.contains_key(&transaction.from) {
            return Err(CoreError::InvalidAccount);
        }

        // Check balance
        let account = &self.state.accounts[&transaction.from];
        if account.balance < transaction.amount + transaction.fee {
            return Err(CoreError::InsufficientBalance);
        }

        // Check amount validity
        if transaction.amount == 0 {
            return Err(CoreError::InvalidAmount);
        }

        Ok(())
    }

    /// Mine new block
    pub fn mine_block(&mut self, author: AccountId) -> CoreResult<Block> {
        let pending_transactions = self.state.pending_transactions.clone();
        self.state.pending_transactions.clear();

        let header = BlockHeader {
            number: self.state.best_block.header.number + 1,
            parent_hash: self.calculate_block_hash(&self.state.best_block),
            state_root: self.calculate_state_root(),
            transactions_root: self.calculate_transactions_root(&pending_transactions),
            timestamp: chrono::Utc::now().timestamp() as u64,
            author,
            difficulty: 1,
            nonce: 0,
        };

        let body = BlockBody {
            transactions: pending_transactions,
            justifications: Vec::new(),
        };

        let block = Block { header, body };
        
        // Add block to chain
        let block_hash = self.calculate_block_hash(&block);
        self.state.blocks.insert(block_hash, block.clone());
        self.state.best_block = block.clone();

        // Process transactions
        self.process_block_transactions(&block)?;

        Ok(block)
    }

    /// Process transactions in a block
    fn process_block_transactions(&mut self, block: &Block) -> CoreResult<()> {
        for transaction in &block.body.transactions {
            self.process_transaction(transaction)?;
        }
        Ok(())
    }

    /// Process individual transaction
    fn process_transaction(&mut self, transaction: &Transaction) -> CoreResult<()> {
        // Update sender balance
        if let Some(sender) = self.state.accounts.get_mut(&transaction.from) {
            sender.balance -= transaction.amount + transaction.fee;
            sender.nonce += 1;
        }

        // Update receiver balance
        if let Some(receiver) = self.state.accounts.get_mut(&transaction.to) {
            receiver.balance += transaction.amount;
        } else {
            // Create new account
            let account = AccountState {
                balance: transaction.amount,
                nonce: 0,
                staked: 0,
                account_type: AccountType::User,
                energy_balance: 0.0,
                last_activity: transaction.timestamp,
            };
            self.state.accounts.insert(transaction.to.clone(), account);
        }

        // Store transaction
        self.state.transactions.insert(transaction.hash.clone(), transaction.clone());

        Ok(())
    }

    /// Calculate block hash
    fn calculate_block_hash(&self, block: &Block) -> Hash {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        block.header.hash(&mut hasher);
        format!("0x{:x}", hasher.finish())
    }

    /// Calculate state root
    fn calculate_state_root(&self) -> Hash {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", self.state.accounts).hash(&mut hasher);
        format!("0x{:x}", hasher.finish())
    }

    /// Calculate transactions root
    fn calculate_transactions_root(&self, transactions: &[Transaction]) -> Hash {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", transactions).hash(&mut hasher);
        format!("0x{:x}", hasher.finish())
    }

    /// Get account state
    pub fn get_account(&self, account_id: &AccountId) -> Option<&AccountState> {
        self.state.accounts.get(account_id)
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, hash: &Hash) -> Option<&Transaction> {
        self.state.transactions.get(hash)
    }

    /// Get block by hash
    pub fn get_block(&self, hash: &Hash) -> Option<&Block> {
        self.state.blocks.get(hash)
    }

    /// Get current block height
    pub fn get_block_height(&self) -> BlockNumber {
        self.state.best_block.header.number
    }

    /// Get balance of account
    pub fn get_balance(&self, account_id: &AccountId) -> Balance {
        self.state.accounts.get(account_id)
            .map(|account| account.balance)
            .unwrap_or(0)
    }
}

impl Default for AccountState {
    fn default() -> Self {
        Self {
            balance: 0,
            nonce: 0,
            staked: 0,
            account_type: AccountType::User,
            energy_balance: 0.0,
            last_activity: 0,
        }
    }
}

use std::hash::{Hash as StdHash, Hasher};

impl StdHash for BlockHeader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
        self.parent_hash.hash(state);
        self.state_root.hash(state);
        self.transactions_root.hash(state);
        self.timestamp.hash(state);
        self.author.hash(state);
        self.difficulty.hash(state);
        self.nonce.hash(state);
    }
}

use crate::api::models::*;
use crate::blockchain::Blockchain;
use crate::energy_trading::{EnergyMarket, EnergyOrder, OrderType, Prosumer};
use crate::token_system::TokenSystem;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<LedgerState>>;

pub struct LedgerState {
    pub blockchain: Blockchain,
    pub token_system: TokenSystem,
    pub energy_market: EnergyMarket,
    pub prosumers: std::collections::HashMap<String, Prosumer>,
}

impl LedgerState {
    pub fn new() -> Self {
        Self {
            blockchain: Blockchain::new(),
            token_system: TokenSystem::new(),
            energy_market: EnergyMarket::new(),
            prosumers: std::collections::HashMap::new(),
        }
    }
}

// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Energy Trading Ledger API is running".to_string()))
}

// Blockchain handlers
pub async fn get_blockchain_info(State(state): State<AppState>) -> Json<ApiResponse<BlockchainInfo>> {
    let state = state.lock().unwrap();
    let info = BlockchainInfo {
        chain_length: state.blockchain.chain.len(),
        difficulty: state.blockchain.difficulty,
        pending_transactions: state.blockchain.pending_transactions.len(),
        latest_block_hash: state.blockchain.get_latest_block().hash.clone(),
    };
    Json(ApiResponse::success(info))
}

pub async fn get_blocks(State(state): State<AppState>) -> Json<ApiResponse<Vec<crate::block::Block>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.blockchain.chain.clone()))
}

pub async fn get_block(State(state): State<AppState>, Path(index): Path<usize>) -> Result<Json<ApiResponse<crate::block::Block>>, StatusCode> {
    let state = state.lock().unwrap();
    if let Some(block) = state.blockchain.chain.get(index) {
        Ok(Json(ApiResponse::success(block.clone())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn mine_block(State(state): State<AppState>, Json(request): Json<MineBlockRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    state.blockchain.mine_pending_transactions(&request.miner_address);
    Json(ApiResponse::success("Block mined successfully".to_string()))
}

pub async fn get_pending_transactions(State(state): State<AppState>) -> Json<ApiResponse<Vec<crate::block::Transaction>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.blockchain.pending_transactions.clone()))
}

// Token System handlers
pub async fn create_token_account(State(state): State<AppState>, Json(request): Json<CreateAccountRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.create_user_account(request.address.clone()) {
        Ok(_) => Json(ApiResponse::success("Account created successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn get_token_balance(State(state): State<AppState>, Path(address): Path<String>) -> Result<Json<ApiResponse<crate::token_system::UserTokenBalance>>, StatusCode> {
    let state = state.lock().unwrap();
    if let Some(balance) = state.token_system.user_balances.get(&address) {
        Ok(Json(ApiResponse::success(balance.clone())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn transfer_tokens(State(state): State<AppState>, Json(request): Json<TransferRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    
    let result = match request.token_type.as_str() {
        "grid" => {
            // For GRID tokens, we need to implement transfer logic
            // For now, let's implement a simple transfer
            match state.token_system.user_balances.get(&request.from) {
                Some(from_balance) => {
                    if from_balance.grid_balance >= request.amount {
                        state.token_system.user_balances.get_mut(&request.from).unwrap().grid_balance -= request.amount;
                        state.token_system.user_balances.get_mut(&request.to).unwrap().grid_balance += request.amount;
                        Ok(())
                    } else {
                        Err("Insufficient balance".to_string())
                    }
                }
                None => Err("Sender account not found".to_string()),
            }
        }
        "watt" => state.token_system.transfer_watt_tokens(&request.from, &request.to, request.amount),
        _ => Err("Invalid token type".to_string()),
    };
    
    match result {
        Ok(_) => Json(ApiResponse::success("Transfer completed successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn stake_tokens(State(state): State<AppState>, Json(request): Json<StakeRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.stake_grid_tokens(&request.address, request.amount) {
        Ok(_) => Json(ApiResponse::success("Tokens staked successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn unstake_tokens(State(state): State<AppState>, Json(request): Json<StakeRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.unstake_grid_tokens(&request.address, request.amount) {
        Ok(_) => Json(ApiResponse::success("Tokens unstaked successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn claim_rewards(State(state): State<AppState>, Path(address): Path<String>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.claim_staking_rewards(&address) {
        Ok(rewards) => Json(ApiResponse::success(format!("Claimed {} GRID tokens as rewards", rewards))),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn create_governance_proposal(State(state): State<AppState>, Json(request): Json<GovernanceProposalRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.create_governance_proposal(&request.proposer, request.title, request.description, request.voting_duration_hours as i64) {
        Ok(proposal_id) => Json(ApiResponse::success(format!("Proposal created with ID: {}", proposal_id))),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn vote_on_proposal(State(state): State<AppState>, Json(request): Json<VoteRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.token_system.vote_on_proposal(&request.voter, &request.proposal_id, request.vote) {
        Ok(_) => Json(ApiResponse::success("Vote recorded successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn get_governance_proposals(State(state): State<AppState>) -> Json<ApiResponse<Vec<crate::token_system::GovernanceProposal>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.token_system.grid_token.governance_proposals.clone()))
}

// Energy Trading handlers
pub async fn create_prosumer(State(state): State<AppState>, Json(request): Json<ProsumerRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    let prosumer = Prosumer::new(request.address.clone(), request.name);
    state.prosumers.insert(request.address.clone(), prosumer);
    Json(ApiResponse::success("Prosumer created successfully".to_string()))
}

pub async fn get_prosumer(State(state): State<AppState>, Path(address): Path<String>) -> Result<Json<ApiResponse<ProsumerInfo>>, StatusCode> {
    let state = state.lock().unwrap();
    if let Some(prosumer) = state.prosumers.get(&address) {
        let info = ProsumerInfo {
            address: prosumer.address.clone(),
            name: prosumer.name.clone(),
            energy_generated: prosumer.energy_generated,
            energy_consumed: prosumer.energy_consumed,
            net_energy: prosumer.get_net_energy(),
            grid_tokens: prosumer.grid_tokens,
            watt_tokens: prosumer.watt_tokens,
        };
        Ok(Json(ApiResponse::success(info)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_prosumers(State(state): State<AppState>) -> Json<ApiResponse<Vec<ProsumerInfo>>> {
    let state = state.lock().unwrap();
    let prosumers: Vec<ProsumerInfo> = state.prosumers.values().map(|p| ProsumerInfo {
        address: p.address.clone(),
        name: p.name.clone(),
        energy_generated: p.energy_generated,
        energy_consumed: p.energy_consumed,
        net_energy: p.get_net_energy(),
        grid_tokens: p.grid_tokens,
        watt_tokens: p.watt_tokens,
    }).collect();
    Json(ApiResponse::success(prosumers))
}

pub async fn update_energy_generation(State(state): State<AppState>, Json(request): Json<EnergyUpdateRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    if let Some(prosumer) = state.prosumers.get_mut(&request.address) {
        prosumer.generate_energy(request.amount);
        Json(ApiResponse::success(format!("Added {} kWh to energy generation", request.amount)))
    } else {
        Json(ApiResponse::error("Prosumer not found".to_string()))
    }
}

pub async fn update_energy_consumption(State(state): State<AppState>, Json(request): Json<EnergyUpdateRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    if let Some(prosumer) = state.prosumers.get_mut(&request.address) {
        prosumer.consume_energy(request.amount);
        Json(ApiResponse::success(format!("Added {} kWh to energy consumption", request.amount)))
    } else {
        Json(ApiResponse::error("Prosumer not found".to_string()))
    }
}

pub async fn create_energy_order(State(state): State<AppState>, Json(request): Json<CreateOrderRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    
    let order_type = match request.order_type.as_str() {
        "buy" => OrderType::Buy,
        "sell" => OrderType::Sell,
        _ => return Json(ApiResponse::error("Invalid order type".to_string())),
    };
    
    let order = EnergyOrder {
        id: uuid::Uuid::new_v4().to_string(),
        trader_address: request.trader_address,
        order_type,
        energy_amount: request.energy_amount,
        price_per_kwh: request.price_per_kwh,
        timestamp: chrono::Utc::now(),
        is_active: true,
    };
    
    match state.energy_market.place_order(order) {
        Ok(order_id) => Json(ApiResponse::success(format!("Order placed with ID: {}", order_id))),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn cancel_energy_order(State(state): State<AppState>, Json(request): Json<CancelOrderRequest>) -> Json<ApiResponse<String>> {
    let mut state = state.lock().unwrap();
    match state.energy_market.cancel_order(&request.order_id) {
        Ok(_) => Json(ApiResponse::success("Order cancelled successfully".to_string())),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn get_buy_orders(State(state): State<AppState>) -> Json<ApiResponse<Vec<EnergyOrder>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.energy_market.buy_orders.iter().cloned().collect()))
}

pub async fn get_sell_orders(State(state): State<AppState>) -> Json<ApiResponse<Vec<EnergyOrder>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.energy_market.sell_orders.iter().cloned().collect()))
}

pub async fn get_trade_history(State(state): State<AppState>) -> Json<ApiResponse<Vec<crate::energy_trading::EnergyTrade>>> {
    let state = state.lock().unwrap();
    Json(ApiResponse::success(state.energy_market.matched_trades.clone()))
}

pub async fn get_market_statistics(State(state): State<AppState>) -> Json<ApiResponse<MarketStatistics>> {
    let state = state.lock().unwrap();
    
    let total_volume = state.energy_market.matched_trades.iter().map(|t| t.energy_amount).sum();
    let average_price = if !state.energy_market.matched_trades.is_empty() {
        state.energy_market.matched_trades.iter().map(|t| t.price_per_kwh).sum::<f64>() / state.energy_market.matched_trades.len() as f64
    } else {
        0.0
    };
    
    let stats = MarketStatistics {
        total_buy_orders: state.energy_market.buy_orders.len(),
        total_sell_orders: state.energy_market.sell_orders.len(),
        total_trades: state.energy_market.matched_trades.len(),
        average_price,
        total_volume,
        grid_fee_rate: state.energy_market.grid_fee_rate,
    };
    
    Json(ApiResponse::success(stats))
}

# Energy Trading Ledger API Documentation

## Overview

The Energy Trading Ledger API provides a RESTful interface for interacting with the decentralized energy trading system. This API allows you to manage prosumers, trade energy, handle tokens, and interact with the blockchain.

## Base URL

```
http://localhost:3000
```

## Authentication

Currently, the API is open for demonstration purposes. In a production environment, you would implement proper authentication using API keys or JWT tokens.

## API Endpoints

### Health Check

#### GET /health
Returns the health status of the API server.

**Response:**
```json
{
  "success": true,
  "data": "Energy Trading Ledger API is running",
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

### Blockchain Endpoints

#### GET /api/blockchain/info
Get information about the blockchain.

**Response:**
```json
{
  "success": true,
  "data": {
    "chain_length": 5,
    "difficulty": 4,
    "pending_transactions": 3,
    "latest_block_hash": "000a1b2c3d4e5f..."
  },
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

#### GET /api/blockchain/blocks
Get all blocks in the blockchain.

#### GET /api/blockchain/blocks/:index
Get a specific block by index.

#### POST /api/blockchain/mine
Mine a new block.

**Request:**
```json
{
  "miner_address": "miner_wallet_address"
}
```

#### GET /api/blockchain/transactions/pending
Get all pending transactions.

### Token System Endpoints

#### POST /api/tokens/accounts
Create a new token account.

**Request:**
```json
{
  "address": "user_wallet_address"
}
```

#### GET /api/tokens/balance/:address
Get token balance for a specific address.

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "user_wallet_address",
    "grid_balance": 100.0,
    "watt_balance": 50.0,
    "staked_grid": 25.0,
    "staking_rewards": 2.5,
    "last_reward_claim": "2025-07-09T10:30:00Z"
  },
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

#### POST /api/tokens/transfer
Transfer tokens between accounts.

**Request:**
```json
{
  "from": "sender_address",
  "to": "recipient_address",
  "amount": 10.0,
  "token_type": "grid"
}
```

#### POST /api/tokens/stake
Stake GRID tokens.

**Request:**
```json
{
  "address": "user_address",
  "amount": 25.0
}
```

#### POST /api/tokens/unstake
Unstake GRID tokens.

**Request:**
```json
{
  "address": "user_address",
  "amount": 10.0
}
```

#### POST /api/tokens/rewards/:address
Claim staking rewards.

### Governance Endpoints

#### GET /api/governance/proposals
Get all governance proposals.

#### POST /api/governance/proposals
Create a new governance proposal.

**Request:**
```json
{
  "title": "Proposal Title",
  "description": "Detailed description of the proposal",
  "proposer": "proposer_address",
  "voting_duration_hours": 168
}
```

#### POST /api/governance/vote
Vote on a governance proposal.

**Request:**
```json
{
  "proposal_id": "proposal_uuid",
  "voter": "voter_address",
  "vote": true,
  "stake_amount": 50.0
}
```

### Energy Trading Endpoints

#### POST /api/energy/prosumers
Create a new prosumer.

**Request:**
```json
{
  "address": "prosumer_address",
  "name": "Prosumer Name"
}
```

#### GET /api/energy/prosumers
Get all prosumers.

#### GET /api/energy/prosumers/:address
Get a specific prosumer by address.

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "prosumer_address",
    "name": "Alice's Solar Farm",
    "energy_generated": 100.0,
    "energy_consumed": 25.0,
    "net_energy": 75.0,
    "grid_tokens": 50.0,
    "watt_tokens": 30.0
  },
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

#### POST /api/energy/generation
Update energy generation for a prosumer.

**Request:**
```json
{
  "address": "prosumer_address",
  "amount": 50.0
}
```

#### POST /api/energy/consumption
Update energy consumption for a prosumer.

**Request:**
```json
{
  "address": "prosumer_address",
  "amount": 25.0
}
```

#### POST /api/energy/orders
Create a new energy order.

**Request:**
```json
{
  "trader_address": "trader_address",
  "order_type": "sell",
  "energy_amount": 25.0,
  "price_per_kwh": 0.15
}
```

#### POST /api/energy/orders/cancel
Cancel an existing energy order.

**Request:**
```json
{
  "order_id": "order_uuid",
  "trader_address": "trader_address"
}
```

#### GET /api/energy/orders/buy
Get all active buy orders.

#### GET /api/energy/orders/sell
Get all active sell orders.

#### GET /api/energy/trades
Get trade history.

#### GET /api/energy/statistics
Get market statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "total_buy_orders": 5,
    "total_sell_orders": 8,
    "total_trades": 12,
    "average_price": 0.145,
    "total_volume": 250.0,
    "grid_fee_rate": 0.05
  },
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

## Error Handling

All endpoints return a consistent error format:

```json
{
  "success": false,
  "data": null,
  "message": "Error description",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

Common HTTP status codes:
- `200 OK` - Success
- `400 Bad Request` - Invalid request data
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## Getting Started

1. **Start the API server:**
   ```bash
   cargo run --bin api-server
   ```

2. **Run the API client demo:**
   ```bash
   cargo run --example api_client_demo
   ```

3. **Test the health endpoint:**
   ```bash
   curl http://localhost:3000/health
   ```

## Example Workflow

1. **Create prosumers:**
   ```bash
   curl -X POST http://localhost:3000/api/energy/prosumers \
     -H "Content-Type: application/json" \
     -d '{"address": "alice_address", "name": "Alice Solar Farm"}'
   ```

2. **Create token accounts:**
   ```bash
   curl -X POST http://localhost:3000/api/tokens/accounts \
     -H "Content-Type: application/json" \
     -d '{"address": "alice_address"}'
   ```

3. **Update energy generation:**
   ```bash
   curl -X POST http://localhost:3000/api/energy/generation \
     -H "Content-Type: application/json" \
     -d '{"address": "alice_address", "amount": 50.0}'
   ```

4. **Create a sell order:**
   ```bash
   curl -X POST http://localhost:3000/api/energy/orders \
     -H "Content-Type: application/json" \
     -d '{"trader_address": "alice_address", "order_type": "sell", "energy_amount": 25.0, "price_per_kwh": 0.15}'
   ```

5. **Get market statistics:**
   ```bash
   curl http://localhost:3000/api/energy/statistics
   ```

## Architecture

The API is built using:
- **Axum** - Modern, ergonomic web framework
- **Tokio** - Asynchronous runtime
- **Serde** - Serialization/deserialization
- **Tower** - Middleware and services
- **UUID** - Unique identifiers

The API provides a RESTful interface to the core ledger functionality, including blockchain operations, token management, and energy trading features.

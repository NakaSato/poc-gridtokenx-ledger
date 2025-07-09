# Energy Trading Ledger - Backend API Layer

## Quick Start

### 1. Install Dependencies

```bash
cargo build
```

### 2. Start the API Server

```bash
cargo run --bin api-server
```

The server will start on `http://localhost:3000` and display all available endpoints.

### 3. Test the API

Run the interactive demo:
```bash
cargo run --example api_client_demo
```

Or test individual endpoints:
```bash
# Health check
curl http://localhost:3000/health

# Get blockchain info
curl http://localhost:3000/api/blockchain/info

# Create a prosumer
curl -X POST http://localhost:3000/api/energy/prosumers \
  -H "Content-Type: application/json" \
  -d '{"address": "alice_address", "name": "Alice Solar Farm"}'

# Get market statistics
curl http://localhost:3000/api/energy/statistics
```

## API Features

### 🔗 Blockchain Operations
- View blockchain information
- Mine new blocks
- Query transactions
- Get block details

### 🪙 Token Management
- Create token accounts
- Transfer GRID and WATT tokens
- Stake/unstake tokens
- Claim staking rewards

### 🗳️ Governance
- Create proposals
- Vote on proposals
- View proposal status

### ⚡ Energy Trading
- Create prosumer profiles
- Update energy generation/consumption
- Place buy/sell orders
- View market statistics
- Track trade history

## Project Structure

```
src/
├── api/
│   ├── mod.rs              # API module exports
│   ├── handlers.rs         # Request handlers
│   ├── middleware.rs       # Middleware (CORS, logging)
│   ├── models.rs           # API request/response models
│   └── server.rs           # Server setup and routing
├── api_server.rs           # API server binary
├── main.rs                 # CLI demo binary
└── ... (other modules)
```

## Available Binaries

- `cargo run --bin api-server` - Start the REST API server
- `cargo run --bin ledger` - Run the CLI demo
- `cargo run --example api_client_demo` - Test the API endpoints

## Development

### Adding New Endpoints

1. Add request/response models to `src/api/models.rs`
2. Implement handlers in `src/api/handlers.rs`
3. Add routes in `src/api/server.rs`
4. Update documentation

### Testing

The API includes comprehensive error handling and logging. All responses follow a consistent format:

```json
{
  "success": true,
  "data": { ... },
  "message": "Success",
  "timestamp": "2025-07-09T10:30:00Z"
}
```

## Production Considerations

- [ ] Add proper authentication/authorization
- [ ] Implement rate limiting
- [ ] Add database persistence
- [ ] Configure HTTPS
- [ ] Add metrics and monitoring
- [ ] Implement API versioning
- [ ] Add comprehensive testing

## Documentation

See [API_DOCUMENTATION.md](./API_DOCUMENTATION.md) for detailed endpoint documentation.

## License

This project is part of the Energy Trading Ledger system.

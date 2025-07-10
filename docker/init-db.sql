-- Initialize the Thai Energy Trading database
-- This script sets up the basic database structure

-- Create the main database if it doesn't exist
CREATE DATABASE IF NOT EXISTS thai_energy_trading;

-- Connect to the database
\c thai_energy_trading;

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create tables for blockchain data
CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    block_number BIGINT NOT NULL UNIQUE,
    block_hash VARCHAR(64) NOT NULL UNIQUE,
    parent_hash VARCHAR(64) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    merkle_root VARCHAR(64) NOT NULL,
    difficulty BIGINT NOT NULL DEFAULT 0,
    nonce BIGINT NOT NULL DEFAULT 0,
    transactions_count INTEGER NOT NULL DEFAULT 0,
    size_bytes INTEGER NOT NULL DEFAULT 0
);

-- Create tables for transactions
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    transaction_hash VARCHAR(64) NOT NULL UNIQUE,
    block_id INTEGER REFERENCES blocks(id),
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount DECIMAL(20,8) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    gas_used BIGINT NOT NULL DEFAULT 0,
    gas_price DECIMAL(20,8) NOT NULL DEFAULT 0,
    status VARCHAR(10) NOT NULL DEFAULT 'pending'
);

-- Create tables for energy trading
CREATE TABLE IF NOT EXISTS energy_orders (
    id SERIAL PRIMARY KEY,
    order_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    prosumer_address VARCHAR(42) NOT NULL,
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('buy', 'sell')),
    energy_amount DECIMAL(10,2) NOT NULL,
    price_per_kwh DECIMAL(10,4) NOT NULL,
    total_amount DECIMAL(20,8) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'open',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
);

-- Create tables for energy trades
CREATE TABLE IF NOT EXISTS energy_trades (
    id SERIAL PRIMARY KEY,
    trade_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
    buyer_address VARCHAR(42) NOT NULL,
    seller_address VARCHAR(42) NOT NULL,
    energy_amount DECIMAL(10,2) NOT NULL,
    price_per_kwh DECIMAL(10,4) NOT NULL,
    total_cost DECIMAL(20,8) NOT NULL,
    grid_fee DECIMAL(20,8) NOT NULL,
    transaction_hash VARCHAR(64),
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) NOT NULL DEFAULT 'completed'
);

-- Create tables for prosumers
CREATE TABLE IF NOT EXISTS prosumers (
    id SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    prosumer_type VARCHAR(20) NOT NULL,
    energy_generated DECIMAL(10,2) NOT NULL DEFAULT 0,
    energy_consumed DECIMAL(10,2) NOT NULL DEFAULT 0,
    reputation_score INTEGER NOT NULL DEFAULT 100,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create tables for token balances
CREATE TABLE IF NOT EXISTS token_balances (
    id SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL UNIQUE,
    watt_balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    energy_balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_blocks_number ON blocks(block_number);
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blocks(block_hash);
CREATE INDEX IF NOT EXISTS idx_transactions_hash ON transactions(transaction_hash);
CREATE INDEX IF NOT EXISTS idx_transactions_from ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_energy_orders_prosumer ON energy_orders(prosumer_address);
CREATE INDEX IF NOT EXISTS idx_energy_orders_type ON energy_orders(order_type);
CREATE INDEX IF NOT EXISTS idx_energy_orders_status ON energy_orders(status);
CREATE INDEX IF NOT EXISTS idx_energy_trades_buyer ON energy_trades(buyer_address);
CREATE INDEX IF NOT EXISTS idx_energy_trades_seller ON energy_trades(seller_address);
CREATE INDEX IF NOT EXISTS idx_prosumers_address ON prosumers(address);
CREATE INDEX IF NOT EXISTS idx_token_balances_address ON token_balances(address);

-- Insert some initial data
INSERT INTO prosumers (address, name, prosumer_type) VALUES
('alice_address', 'Alice''s Solar Farm', 'producer'),
('bob_address', 'Bob''s Wind Turbine', 'producer'),
('charlie_address', 'Charlie''s Home', 'consumer'),
('grid_operator', 'Grid Operator', 'system')
ON CONFLICT (address) DO NOTHING;

INSERT INTO token_balances (address, watt_balance, energy_balance) VALUES
('alice_address', 1000.0, 0.0),
('bob_address', 1000.0, 0.0),
('charlie_address', 1000.0, 0.0),
('grid_operator', 1000000.0, 0.0)
ON CONFLICT (address) DO NOTHING;

-- Create views for convenience
CREATE OR REPLACE VIEW prosumer_summary AS
SELECT 
    p.address,
    p.name,
    p.prosumer_type,
    p.energy_generated,
    p.energy_consumed,
    (p.energy_generated - p.energy_consumed) as net_energy,
    p.reputation_score,
    tb.watt_balance,
    tb.energy_balance
FROM prosumers p
LEFT JOIN token_balances tb ON p.address = tb.address;

CREATE OR REPLACE VIEW order_book AS
SELECT 
    order_type,
    COUNT(*) as order_count,
    SUM(energy_amount) as total_energy,
    AVG(price_per_kwh) as avg_price,
    MIN(price_per_kwh) as min_price,
    MAX(price_per_kwh) as max_price
FROM energy_orders
WHERE status = 'open'
GROUP BY order_type;

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE thai_energy_trading TO thai_energy;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO thai_energy;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO thai_energy;

# Docker Development Setup

## Quick Start

### Prerequisites
- Docker Desktop or Docker Engine (>= 20.10)
- Docker Compose (>= 2.0)
- At least 4GB RAM available for Docker

### Launch the System

1. **Start all services:**
   ```bash
   ./docker-compose.sh up
   ```

2. **Access the application:**
   - API Server: http://localhost:8080
   - API Documentation: http://localhost:8080/docs
   - Health Check: http://localhost:8080/health
   - Grafana: http://localhost:3000 (admin/admin123)
   - Prometheus: http://localhost:9090

### Management Commands

```bash
# Build all services
./docker-compose.sh build

# Start services in background
./docker-compose.sh up -d

# Stop all services
./docker-compose.sh down

# View logs
./docker-compose.sh logs

# View logs for specific service
./docker-compose.sh logs thai-energy-trading

# Execute commands in containers
./docker-compose.sh exec thai-energy-trading bash

# Scale services
./docker-compose.sh scale thai-energy-trading=3
```

## Services Overview

### Thai Energy Trading System
- **Container**: `thai-energy-trading`
- **Ports**: 8080 (API), 9944 (WebSocket), 30333 (P2P)
- **Health**: http://localhost:8080/health

### PostgreSQL Database
- **Container**: `thai-energy-postgres`
- **Port**: 5432
- **Database**: `thai_energy_trading`
- **User**: `thai_energy`
- **Password**: `password123`

### Redis Cache
- **Container**: `thai-energy-redis`
- **Port**: 6379

### Monitoring Stack
- **Prometheus**: Port 9090
- **Grafana**: Port 3000
- **Nginx**: Port 80

## Development Workflow

### Code Changes
1. Make changes to source code
2. Rebuild: `./docker-compose.sh build`
3. Restart: `./docker-compose.sh restart thai-energy-trading`
4. Test changes

### Database Access
```bash
# Connect to database
./docker-compose.sh exec postgres psql -U thai_energy -d thai_energy_trading

# Run SQL queries
./docker-compose.sh exec postgres psql -U thai_energy -d thai_energy_trading -c "SELECT * FROM energy_orders LIMIT 10;"
```

### Application Debugging
```bash
# View application logs
./docker-compose.sh logs -f thai-energy-trading

# Access container shell
./docker-compose.sh exec thai-energy-trading bash

# Run Rust tests
./docker-compose.sh exec thai-energy-trading cargo test
```

## Configuration

### Environment Variables
Edit `docker-compose.yml` to modify environment variables:
- `RUST_LOG`: Log level (debug, info, warn, error)
- `DATABASE_PASSWORD`: Database password
- `JWT_SECRET`: JWT signing secret
- `GRID_OPERATOR_KEY`: Grid operator authentication key

### Volume Mounts
- `thai_energy_data`: Application data
- `thai_energy_logs`: Application logs
- `postgres_data`: Database data
- `redis_data`: Redis cache

## API Testing

### Health Check
```bash
curl http://localhost:8080/health
```

### Token System
```bash
# Create token
curl -X POST http://localhost:8080/api/tokens \
  -H "Content-Type: application/json" \
  -d '{"amount": 1000, "user_id": "user123"}'

# Get token balance
curl http://localhost:8080/api/tokens/balance/user123
```

### Energy Trading
```bash
# Create energy order
curl -X POST http://localhost:8080/api/energy/orders \
  -H "Content-Type: application/json" \
  -d '{
    "prosumer_id": "prosumer123",
    "order_type": "sell",
    "energy_amount": 100.0,
    "price_per_kwh": 0.12
  }'

# Get market data
curl http://localhost:8080/api/energy/market
```

## Performance Testing

### Load Testing
```bash
# Run performance benchmarks
./docker-compose.sh exec thai-energy-trading cargo bench

# Stress test API
./docker-compose.sh exec thai-energy-trading cargo run --example performance_focused_test
```

### Monitoring
- **Prometheus**: View metrics at http://localhost:9090
- **Grafana**: Dashboard at http://localhost:3000
- **Logs**: `./docker-compose.sh logs -f`

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean Docker cache
docker system prune -a

# Rebuild with no cache
./docker-compose.sh build --no-cache
```

#### Connection Issues
```bash
# Check service status
./docker-compose.sh ps

# Test database connection
./docker-compose.sh exec thai-energy-trading ping postgres
```

#### Performance Issues
```bash
# Check resource usage
docker stats

# Scale application
./docker-compose.sh scale thai-energy-trading=3
```

### Reset Everything
```bash
# Stop and remove all containers and volumes
./docker-compose.sh down -v

# Remove all images
docker rmi $(docker images -q)

# Rebuild everything
./docker-compose.sh build --no-cache
./docker-compose.sh up
```

## Production Deployment

For production deployment, see [DOCKER_KUBERNETES_DEPLOYMENT_GUIDE.md](DOCKER_KUBERNETES_DEPLOYMENT_GUIDE.md).

## Support

For issues and questions:
1. Check the logs: `./docker-compose.sh logs`
2. Review the troubleshooting section
3. Check the main README.md for additional information
